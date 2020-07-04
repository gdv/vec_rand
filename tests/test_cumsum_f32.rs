extern crate vec_rand;

const SIZE: usize = 1000;
const ITER: usize = 1000;

#[test]
fn test_cumsum_f32_scan() {
    for _ in 0..ITER {
        let weights = vec_rand::gen_random_vec_f32(SIZE, 0xBAD5eed);

        let truth = vec_rand::cumsum_f32::cumsum_f32(&weights);
        let tested = vec_rand::cumsum_f32::cumsum_f32_scan(&weights);

        for (a, b) in truth.iter().zip(tested.iter()) {
            assert!((a - b).abs() < 0.0001);
        }
    }
}

#[test]
fn test_cumsum_f32_unrolled() {
    for _ in 0..ITER {
        let weights = vec_rand::gen_random_vec_f32(SIZE, 0xBAD5eed);

        let truth = vec_rand::cumsum_f32::cumsum_f32(&weights);
        let tested = vec_rand::cumsum_f32::cumsum_f32_unrolled(&weights);

        for (a, b) in truth.iter().zip(tested.iter()) {
            assert!((a - b).abs() < 0.001);
        }
    }
}

//#[test]
fn test_cumsum_f32_sse_intrinsics() {
    for _ in 0..ITER {
        let weights = vec_rand::gen_random_vec_f32(SIZE, 0xBAD5eed);

        let truth = vec_rand::cumsum_f32::cumsum_f32(&weights);
        let tested = vec_rand::cumsum_f32::cumsum_f32_sse_intrinsics(&weights);

        for (a, b) in truth.iter().zip(tested.iter()) {
            assert!((a - b).abs() < 0.0001);
        }
    }
}