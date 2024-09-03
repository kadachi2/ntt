//! Experimental implementation of number theoretic transfrom in Rust.
//! This is fully based off of the following paper eprint.iacr.org/2024/585.pdf.
//!
//! TODO list:
//! - [x] add simd option
//! - [x] implement cyclic_convovle
//! - [ ] implement negacyclic_convolve
//! - [ ] implement NTT-based convolution

use std::cmp;

fn linear_convolve(g: &[u32], h: &[u32]) -> Vec<u32> {
    // perhaps could be written using only iterators?
    let mut res: Vec<u32> = vec![0; g.len() + h.len() - 1];

    for i in 0..g.len() {
        for j in 0..h.len() {
            let deg: usize = i + j
            res[deg] += g[i] * h[j];
        }
    }

    res
}

fn positive_wrapped_convolve(g: &[u32], h: &[u32]) -> Vec<u32> {
    let max_deg: usize = cmp::max(g.len(), h.len());
    let mut res: Vec<u32> = vec![0; cmp::max(g.len(), h.len())];

    for i in 0..g.len() {
        for j in 0..h.len() {
            // here, degree wraps around
            let deg: usize = (i + j) % max_deg;
            res[deg] += g[i] * h[j]; 
        }
    }

    res
}

fn negative_wrapped_convolve(g: &[u32], h: &[u32]) -> Vec<u32> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_convolve_returns_correct_result() {
        let g = vec![1, 2, 3, 4];
        let h = vec![5, 6, 7, 8];

        let result: Vec<u32> = linear_convolve(&g, &h);
        let expected_result: Vec<u32> = vec![5, 16, 34, 60, 61, 52, 32];

        linear_convolve(&g, &h)
            .iter()
            .zip(expected_result.iter())
            .map(|(y, expected)| assert_eq!(y, expected));
    }

    #[test]
    fn linear_convolve_returns_correct_result_for_differently_sized_vector() {
        let g = vec![1, 2];
        let h = vec![3, 4, 5, 6, 7, 8];

        let result: Vec<u32> = linear_convolve(&g, &h);
        let expected_result: Vec<u32> = vec![5, 16, 34, 60, 61, 52, 32];

        linear_convolve(&g, &h)
            .iter()
            .zip(expected_result.iter())
            .map(|(y, expected)| assert_eq!(y, expected));
    }

    #[test]
    fn positive_wrapped_convolve_returns_correct_result() {
        let g = vec![1, 2, 3, 4];
        let h = vec![5, 6, 7, 8];

        let result: Vec<u32> = linear_convolve(&g, &h);
        let expected_result: Vec<u32> = vec![66, 68, 66, 60];

        positive_wrapped_convolve(&g, &h)
            .iter()
            .zip(expected_result.iter())
            .map(|(y, expected)| assert_eq!(y, expected));
    }
}
