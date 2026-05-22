use crate::kernels::simd_scan::{
    count_greater_branchless, count_greater_branchy, count_greater_explicit, SimdScanInput,
};

const DEFAULT_INPUT_LEN: usize = 1 << 20;
const DEFAULT_THRESHOLD: u8 = 127;
const DEFAULT_SEED: u64 = 0x51DD_F00D_1EAF_0001;

#[derive(Clone, Copy, Debug)]
pub enum SimdVariant {
    ScalarBranchy,
    CompilerVectorized,
    ExplicitSimd,
}

impl SimdVariant {
    pub const ALL: [Self; 3] = [
        Self::ScalarBranchy,
        Self::CompilerVectorized,
        Self::ExplicitSimd,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::ScalarBranchy => "scalar_branchy",
            Self::CompilerVectorized => "compiler_vectorized_candidate",
            Self::ExplicitSimd => "explicit_simd",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SimdCase {
    pub variant: SimdVariant,
    pub work_units: u64,
}

pub struct SimdScanExperiment {
    input: SimdScanInput,
}

impl Default for SimdScanExperiment {
    fn default() -> Self {
        Self {
            input: SimdScanInput::build(DEFAULT_INPUT_LEN, DEFAULT_THRESHOLD, DEFAULT_SEED),
        }
    }
}

impl SimdScanExperiment {
    pub fn cases(&self) -> [SimdCase; 3] {
        SimdVariant::ALL.map(|variant| SimdCase {
            variant,
            work_units: self.len() as u64,
        })
    }

    pub fn len(&self) -> usize {
        self.input.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.input.data.is_empty()
    }

    pub fn benchmark_input_label(&self) -> String {
        format!("{}-bytes", self.len())
    }

    pub fn run(&self, variant: SimdVariant) -> usize {
        match variant {
            SimdVariant::ScalarBranchy => {
                count_greater_branchy(&self.input.data, self.input.threshold)
            }
            SimdVariant::CompilerVectorized => {
                count_greater_branchless(&self.input.data, self.input.threshold)
            }
            SimdVariant::ExplicitSimd => {
                count_greater_explicit(&self.input.data, self.input.threshold)
            }
        }
    }

    pub fn verify(&self) {
        let branchy = self.run(SimdVariant::ScalarBranchy);
        let auto = self.run(SimdVariant::CompilerVectorized);
        let explicit = self.run(SimdVariant::ExplicitSimd);
        assert_eq!(
            branchy, auto,
            "branchless candidate diverged from scalar baseline"
        );
        assert_eq!(
            branchy, explicit,
            "explicit SIMD diverged from scalar baseline"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::SimdScanExperiment;

    #[test]
    fn simd_variants_agree() {
        SimdScanExperiment::default().verify();
    }
}
