use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct MatrixInputs {
    pub size: usize,
    pub tile_size: usize,
    a: Vec<f32>,
    b: Vec<f32>,
}

impl MatrixInputs {
    pub fn build(size: usize, tile_size: usize, seed: u64) -> Self {
        assert!(size > 0, "matrix size must be non-zero");
        assert!(tile_size > 0, "tile size must be non-zero");

        let mut rng = SmallRng::seed_from_u64(seed);
        let mut next_value = || rng.gen_range(-1.0_f32..1.0_f32);

        let a = (0..size * size).map(|_| next_value()).collect();
        let b = (0..size * size).map(|_| next_value()).collect();

        Self {
            size,
            tile_size,
            a,
            b,
        }
    }

    pub fn output_len(&self) -> usize {
        self.size * self.size
    }
}

pub fn multiply_naive(inputs: &MatrixInputs, output: &mut [f32]) {
    let n = inputs.size;
    assert_eq!(
        output.len(),
        inputs.output_len(),
        "output buffer size mismatch"
    );

    output.fill(0.0);

    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0_f32;
            for k in 0..n {
                sum += inputs.a[i * n + k] * inputs.b[k * n + j];
            }
            output[i * n + j] = sum;
        }
    }
}

pub fn multiply_blocked(inputs: &MatrixInputs, output: &mut [f32], tile_size: usize) {
    let n = inputs.size;
    assert_eq!(
        output.len(),
        inputs.output_len(),
        "output buffer size mismatch"
    );

    output.fill(0.0);

    for ii in (0..n).step_by(tile_size) {
        for kk in (0..n).step_by(tile_size) {
            for jj in (0..n).step_by(tile_size) {
                let i_limit = (ii + tile_size).min(n);
                let k_limit = (kk + tile_size).min(n);
                let j_limit = (jj + tile_size).min(n);

                for i in ii..i_limit {
                    for k in kk..k_limit {
                        let a_ik = inputs.a[i * n + k];
                        for j in jj..j_limit {
                            output[i * n + j] += a_ik * inputs.b[k * n + j];
                        }
                    }
                }
            }
        }
    }
}

pub fn checksum(output: &[f32]) -> f32 {
    let len = output.len();
    output[0] + output[len / 2] + output[len - 1]
}

pub fn approx_eq_slice(left: &[f32], right: &[f32], epsilon: f32) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right.iter())
            .all(|(lhs, rhs)| (lhs - rhs).abs() <= epsilon)
}
