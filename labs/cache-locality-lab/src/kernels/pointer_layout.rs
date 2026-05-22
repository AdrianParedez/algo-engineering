use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::collections::BTreeSet;
use std::ptr::NonNull;

const PAGE_SHIFT: usize = 12;

#[derive(Debug)]
struct BoxedNode {
    value: u64,
    next: NonNull<BoxedNode>,
}

#[derive(Debug)]
struct ArenaNode {
    value: u64,
    next: NonNull<ArenaNode>,
}

#[derive(Clone, Copy, Debug)]
struct PackedIndexNode {
    value: u64,
    next: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct PointerLayoutFootprint {
    pub boxed_unique_pages: usize,
    pub arena_unique_pages: usize,
    pub packed_unique_pages: usize,
    pub values_unique_pages: usize,
    pub next_indices_unique_pages: usize,
}

pub struct PointerLayoutData {
    boxed_head: NonNull<BoxedNode>,
    arena_head: NonNull<ArenaNode>,
    start_index: usize,
    pub steps: usize,
    values: Vec<u64>,
    next_indices: Vec<usize>,
    packed_nodes: Vec<PackedIndexNode>,
    arena_nodes: Vec<ArenaNode>,
    // Intentional benchmark axis: one box per node preserves allocator scatter.
    #[allow(clippy::vec_box)]
    boxed_nodes: Vec<Box<BoxedNode>>,
    footprint: PointerLayoutFootprint,
}

impl PointerLayoutData {
    pub fn build(len: usize, seed: u64) -> Self {
        assert!(len > 0, "pointer-chase inputs must be non-empty");

        let mut rng = SmallRng::seed_from_u64(seed);
        let mut traversal_order: Vec<usize> = (0..len).collect();
        traversal_order.shuffle(&mut rng);

        let values: Vec<u64> = (0..len).map(|_| rng.gen_range(0..1024_u64)).collect();
        let mut next_indices = vec![0; len];

        for window in traversal_order.windows(2) {
            next_indices[window[0]] = window[1];
        }
        next_indices[*traversal_order.last().unwrap()] = traversal_order[0];

        let packed_nodes: Vec<PackedIndexNode> = (0..len)
            .map(|index| PackedIndexNode {
                value: values[index],
                next: next_indices[index],
            })
            .collect();

        let mut arena_nodes: Vec<ArenaNode> = values
            .iter()
            .copied()
            .map(|value| ArenaNode {
                value,
                next: NonNull::dangling(),
            })
            .collect();
        let arena_base = arena_nodes.as_mut_ptr();
        for (index, next_index) in next_indices.iter().copied().enumerate() {
            arena_nodes[index].next = unsafe { NonNull::new_unchecked(arena_base.add(next_index)) };
        }

        let mut boxed_nodes: Vec<Box<BoxedNode>> = values
            .iter()
            .copied()
            .map(|value| {
                Box::new(BoxedNode {
                    value,
                    next: NonNull::dangling(),
                })
            })
            .collect();
        let boxed_raw_nodes: Vec<NonNull<BoxedNode>> = boxed_nodes
            .iter_mut()
            .map(|node| NonNull::from(node.as_mut()))
            .collect();
        for (index, next_index) in next_indices.iter().copied().enumerate() {
            boxed_nodes[index].next = boxed_raw_nodes[next_index];
        }

        let boxed_head = boxed_raw_nodes[traversal_order[0]];
        let arena_head = unsafe { NonNull::new_unchecked(arena_base.add(traversal_order[0])) };
        let footprint = PointerLayoutFootprint {
            boxed_unique_pages: unique_pages(
                boxed_raw_nodes
                    .iter()
                    .copied()
                    .map(|ptr| ptr.as_ptr() as usize),
            ),
            arena_unique_pages: unique_pages(
                arena_nodes.iter().map(|node| node as *const _ as usize),
            ),
            packed_unique_pages: unique_pages(
                packed_nodes.iter().map(|node| node as *const _ as usize),
            ),
            values_unique_pages: unique_pages(
                values.iter().map(|value| value as *const _ as usize),
            ),
            next_indices_unique_pages: unique_pages(
                next_indices
                    .iter()
                    .map(|next_index| next_index as *const _ as usize),
            ),
        };

        Self {
            boxed_head,
            arena_head,
            start_index: traversal_order[0],
            steps: len,
            values,
            next_indices,
            packed_nodes,
            arena_nodes,
            boxed_nodes,
            footprint,
        }
    }

    pub fn footprint(&self) -> PointerLayoutFootprint {
        self.footprint
    }

    pub fn touch_bytes(&self) -> usize {
        self.values.len() * std::mem::size_of::<u64>()
    }
}

pub fn sum_boxed_pointer_chain(data: &PointerLayoutData) -> u64 {
    debug_assert_eq!(data.boxed_nodes.len(), data.steps);

    let mut current = data.boxed_head;
    let mut acc = 0_u64;

    for _ in 0..data.steps {
        let node = unsafe { current.as_ref() };
        acc += node.value;
        current = node.next;
    }

    acc
}

pub fn sum_arena_pointer_chain(data: &PointerLayoutData) -> u64 {
    debug_assert_eq!(data.arena_nodes.len(), data.steps);

    let mut current = data.arena_head;
    let mut acc = 0_u64;

    for _ in 0..data.steps {
        let node = unsafe { current.as_ref() };
        acc += node.value;
        current = node.next;
    }

    acc
}

pub fn sum_packed_index_chase(data: &PointerLayoutData) -> u64 {
    let mut index = data.start_index;
    let mut acc = 0_u64;

    for _ in 0..data.steps {
        let node = &data.packed_nodes[index];
        acc += node.value;
        index = node.next;
    }

    acc
}

pub fn sum_split_soa_index_chase(data: &PointerLayoutData) -> u64 {
    let mut index = data.start_index;
    let mut acc = 0_u64;

    for _ in 0..data.steps {
        acc += data.values[index];
        index = data.next_indices[index];
    }

    acc
}

pub fn sum_flat_sequential(data: &PointerLayoutData) -> u64 {
    data.values.iter().copied().sum()
}

fn unique_pages(addresses: impl Iterator<Item = usize>) -> usize {
    addresses
        .map(|address| address >> PAGE_SHIFT)
        .collect::<BTreeSet<_>>()
        .len()
}
