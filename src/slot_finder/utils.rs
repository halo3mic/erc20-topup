use rand;
use ethers::prelude::*;
pub use crate::utils::*;


pub fn ratio_f64(val1: U256, val2: U256, precision_mul: Option<u128>) -> f64 {
    let p_mul = precision_mul.unwrap_or(10_000);
    let ur_bn = U512::from(val1) * U512::from(p_mul) / U512::from(val2);
    let update_ratio = 
        if ur_bn <= U512::max_value() / U512::from(4) {
            ur_bn.as_u128()
        } else {
            u128::max_value()
        } as f64 / p_mul as f64;
    update_ratio
}

pub fn rand_num<T>() -> T 
    where rand::distributions::Standard: rand::distributions::Distribution<T>
{
    rand::random::<T>()
}

