use std::{ops::{AddAssign, Div, DivAssign, Mul, MulAssign}, time::Duration};

use crate::rng::TickBasedRng;

pub fn lfsr(lfsr: u32) -> u32 {
    let bit = ((lfsr >> 0) ^ (lfsr >> 1) ^ (lfsr >> 2) ^ (lfsr >> 22)) & 1;
    (lfsr >> 1) | (bit << 31)
}

pub fn avg(vals: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for val in vals {
        sum += *val;
    }
    let len = vals.len();
    sum / len as f64
}

pub fn product<T: From<i32> + MulAssign + Copy>(vals: &Vec<T>) -> T {
    let mut product = 1.into();
    vals.iter().map(|val| product *= *val).collect::<Vec<_>>();
    product
}

pub fn insert_noise(vals: &mut Vec<&f64>, max_noise: f64, rng: &mut TickBasedRng) {

}

pub fn frequency_to_period(f: u32) -> Duration {
    Duration::from_millis(((1.0 / f as f64) * 1000.0) as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_avg() {
        let vals = vec![1.1, 4.2, 5.5, 2.3];
        assert!(avg(&vals) > 3.2 && avg(&vals) < 3.3);
    }

    #[test]
    fn test_insert_noise() {
        let mut vals = vec![2.0, 3.0];
        let vals_old = vals.clone();
        let mut rng = TickBasedRng::new(0, 0);
        insert_noise(&mut vals, 0.1, &mut rng);
        let vals_bits: Vec<_> = vals.iter().map(|val: &f64| val.to_bits()).collect();
        let vals_old_bits: Vec<_> = vals_old.iter().map(|val: &f64| val.to_bits()).collect();

        assert!(vals_bits != vals_old_bits && vals[0] < 2.11 && vals[0] > 1.89 && vals[1] < 3.11 && vals[1] > 2.89);
        
        let mut vals = vec![0.0, 0.0];
        insert_noise(&mut vals, 0.5, &mut rng);
        assert!(vals[0] > -0.01 && vals[1] > -0.01);
        
        let mut vals = vec![1.0, 1.0];
        insert_noise(&mut vals, 0.5, &mut rng);
        assert!(vals[0] < 1.01 && vals[1] < 1.01);
    }
}
