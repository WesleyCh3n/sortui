use rand::prelude::*;

pub fn gen_rand_vec<'a>(n: usize) -> Vec<u64> {
    let start = 5;
    let mut values: Vec<u64> = (start..n as u64 + start).collect();
    let mut rng = rand::thread_rng();
    values.shuffle(&mut rng);

    values
}
