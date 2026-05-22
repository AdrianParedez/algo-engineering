use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct SimdScanInput {
    pub data: Vec<u8>,
    pub threshold: u8,
}

impl SimdScanInput {
    pub fn build(len: usize, threshold: u8, seed: u64) -> Self {
        assert!(len > 0, "SIMD scan input must be non-empty");

        let mut rng = SmallRng::seed_from_u64(seed);
        let data = (0..len).map(|_| rng.gen_range(u8::MIN..=u8::MAX)).collect();

        Self { data, threshold }
    }
}

pub fn count_greater_branchy(input: &[u8], threshold: u8) -> usize {
    let mut count = 0_usize;

    for &value in input {
        if value > threshold {
            count += 1;
        }
    }

    count
}

pub fn count_greater_branchless(input: &[u8], threshold: u8) -> usize {
    input
        .iter()
        .map(|&value| usize::from(value > threshold))
        .sum()
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn count_greater_explicit(input: &[u8], threshold: u8) -> usize {
    if std::arch::is_x86_feature_detected!("avx2") {
        unsafe { count_greater_avx2(input, threshold) }
    } else {
        count_greater_branchless(input, threshold)
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn count_greater_explicit(input: &[u8], threshold: u8) -> usize {
    count_greater_branchless(input, threshold)
}

#[cfg(target_arch = "x86")]
use std::arch::x86::{
    __m256i, _mm256_cmpgt_epi8, _mm256_loadu_si256, _mm256_movemask_epi8, _mm256_set1_epi8,
    _mm256_xor_si256,
};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{
    __m256i, _mm256_cmpgt_epi8, _mm256_loadu_si256, _mm256_movemask_epi8, _mm256_set1_epi8,
    _mm256_xor_si256,
};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn count_greater_avx2(input: &[u8], threshold: u8) -> usize {
    let len = input.len();
    let bias = _mm256_set1_epi8(i8::MIN);
    let threshold_vector = _mm256_xor_si256(_mm256_set1_epi8(threshold as i8), bias);

    let mut index = 0_usize;
    let mut count = 0_usize;

    while index + 32 <= len {
        let chunk = _mm256_loadu_si256(input.as_ptr().add(index) as *const __m256i);
        let adjusted = _mm256_xor_si256(chunk, bias);
        let mask = _mm256_cmpgt_epi8(adjusted, threshold_vector);
        count += (_mm256_movemask_epi8(mask) as u32).count_ones() as usize;
        index += 32;
    }

    count + count_greater_branchless(&input[index..], threshold)
}
