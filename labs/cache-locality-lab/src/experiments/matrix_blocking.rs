use crate::kernels::matrix::{
    approx_eq_slice, checksum, multiply_blocked, multiply_naive, MatrixInputs,
};

const DEFAULT_MATRIX_SIZE: usize = 128;
const DEFAULT_TILE_SIZE: usize = 32;
const DEFAULT_SEED: u64 = 0xBA10_C0DE_0D15_EA5E;

#[derive(Clone, Copy, Debug)]
pub enum MatrixVariant {
    Naive,
    Blocked { tile_size: usize },
}

impl MatrixVariant {
    pub fn name(self) -> &'static str {
        match self {
            Self::Naive => "naive_ijk",
            Self::Blocked { tile_size: 32 } => "blocked_32",
            Self::Blocked { tile_size: 64 } => "blocked_64",
            Self::Blocked { .. } => "blocked_custom",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MatrixCase {
    pub variant: MatrixVariant,
    pub work_units: u64,
}

pub struct MatrixBlockingExperiment {
    inputs: MatrixInputs,
}

impl Default for MatrixBlockingExperiment {
    fn default() -> Self {
        Self {
            inputs: MatrixInputs::build(DEFAULT_MATRIX_SIZE, DEFAULT_TILE_SIZE, DEFAULT_SEED),
        }
    }
}

impl MatrixBlockingExperiment {
    pub fn cases(&self) -> [MatrixCase; 2] {
        [
            MatrixCase {
                variant: MatrixVariant::Naive,
                work_units: self.multiply_accumulate_count(),
            },
            MatrixCase {
                variant: MatrixVariant::Blocked {
                    tile_size: self.inputs.tile_size,
                },
                work_units: self.multiply_accumulate_count(),
            },
        ]
    }

    pub fn size(&self) -> usize {
        self.inputs.size
    }

    pub fn output_len(&self) -> usize {
        self.inputs.output_len()
    }

    pub fn multiply_accumulate_count(&self) -> u64 {
        let n = self.inputs.size as u64;
        n * n * n
    }

    pub fn benchmark_input_label(&self) -> String {
        format!("n{}-{}-macs", self.size(), self.multiply_accumulate_count())
    }

    pub fn run(&self, variant: MatrixVariant, output: &mut [f32]) -> f32 {
        match variant {
            MatrixVariant::Naive => multiply_naive(&self.inputs, output),
            MatrixVariant::Blocked { tile_size } => {
                multiply_blocked(&self.inputs, output, tile_size)
            }
        }
        checksum(output)
    }

    pub fn verify(&self) {
        let mut baseline = vec![0.0; self.output_len()];
        let mut blocked = vec![0.0; self.output_len()];
        let baseline_checksum = self.run(MatrixVariant::Naive, &mut baseline);
        let blocked_checksum = self.run(
            MatrixVariant::Blocked {
                tile_size: self.inputs.tile_size,
            },
            &mut blocked,
        );
        assert!(
            approx_eq_slice(&baseline, &blocked, 1.0e-3),
            "blocked output diverged from naive output"
        );
        assert!(
            (baseline_checksum - blocked_checksum).abs() <= 1.0e-3,
            "blocked checksum diverged from naive checksum"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::MatrixBlockingExperiment;

    #[test]
    fn matrix_variants_agree() {
        MatrixBlockingExperiment::default().verify();
    }
}
