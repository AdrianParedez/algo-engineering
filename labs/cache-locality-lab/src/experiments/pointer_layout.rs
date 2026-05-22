use crate::kernels::pointer_layout::{
    sum_arena_pointer_chain, sum_boxed_pointer_chain, sum_flat_sequential, sum_packed_index_chase,
    sum_split_soa_index_chase, PointerLayoutData, PointerLayoutFootprint,
};

const DEFAULT_ELEMENT_COUNT: usize = 1 << 19;
const DEFAULT_SEED: u64 = 0xCACE_10CA_11AB_1E55;

#[derive(Clone, Copy, Debug)]
pub enum PointerLayoutVariant {
    BoxedPointerChain,
    ArenaPointerChain,
    PackedIndexChase,
    SplitSoaIndexChase,
    FlatSequential,
}

impl PointerLayoutVariant {
    pub const ALL: [Self; 5] = [
        Self::BoxedPointerChain,
        Self::ArenaPointerChain,
        Self::PackedIndexChase,
        Self::SplitSoaIndexChase,
        Self::FlatSequential,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::BoxedPointerChain => "boxed_pointer_chain",
            Self::ArenaPointerChain => "arena_pointer_chain",
            Self::PackedIndexChase => "packed_index_chase",
            Self::SplitSoaIndexChase => "split_soa_index_chase",
            Self::FlatSequential => "flat_sequential",
        }
    }

    pub fn interpretation(self) -> &'static str {
        match self {
            Self::BoxedPointerChain => {
                "allocator scatter + pointer dependency + packed node payload"
            }
            Self::ArenaPointerChain => {
                "contiguous allocation + pointer dependency + packed node payload"
            }
            Self::PackedIndexChase => {
                "contiguous allocation + integer dependency + packed node payload"
            }
            Self::SplitSoaIndexChase => {
                "contiguous allocation + integer dependency + split value/next arrays"
            }
            Self::FlatSequential => "contiguous streaming baseline without dependency chasing",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PointerLayoutCase {
    pub variant: PointerLayoutVariant,
    pub work_units: u64,
}

pub struct PointerLayoutExperiment {
    data: PointerLayoutData,
}

impl Default for PointerLayoutExperiment {
    fn default() -> Self {
        Self {
            data: PointerLayoutData::build(DEFAULT_ELEMENT_COUNT, DEFAULT_SEED),
        }
    }
}

impl PointerLayoutExperiment {
    pub fn cases(&self) -> Vec<PointerLayoutCase> {
        PointerLayoutVariant::ALL
            .into_iter()
            .map(|variant| PointerLayoutCase {
                variant,
                work_units: self.element_count() as u64,
            })
            .collect()
    }

    pub fn element_count(&self) -> usize {
        self.data.steps
    }

    pub fn touch_bytes(&self) -> usize {
        self.data.touch_bytes()
    }

    pub fn footprint(&self) -> PointerLayoutFootprint {
        self.data.footprint()
    }

    pub fn benchmark_input_label(&self) -> String {
        format!(
            "{}-elements-{}-bytes",
            self.element_count(),
            self.touch_bytes()
        )
    }

    pub fn run(&self, variant: PointerLayoutVariant) -> u64 {
        match variant {
            PointerLayoutVariant::BoxedPointerChain => sum_boxed_pointer_chain(&self.data),
            PointerLayoutVariant::ArenaPointerChain => sum_arena_pointer_chain(&self.data),
            PointerLayoutVariant::PackedIndexChase => sum_packed_index_chase(&self.data),
            PointerLayoutVariant::SplitSoaIndexChase => sum_split_soa_index_chase(&self.data),
            PointerLayoutVariant::FlatSequential => sum_flat_sequential(&self.data),
        }
    }

    pub fn verify(&self) {
        let sequential = self.run(PointerLayoutVariant::FlatSequential);
        for variant in PointerLayoutVariant::ALL {
            let result = self.run(variant);
            assert_eq!(
                result,
                sequential,
                "{} diverged from the sequential baseline",
                variant.name()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PointerLayoutExperiment;

    #[test]
    fn pointer_layout_variants_agree() {
        PointerLayoutExperiment::default().verify();
    }
}
