use ethers::types::{I256, U256};
use eyre::{eyre, Result};

pub const ONE_X18: i128 = 1000000000000000000;
pub const ONE_X6: i128 = 1_000_000;
const UONE: u128 = 1000000000000000000;

fn signed_to_unsigned(x: i128, y: i128) -> (u128, u128, i128) {
    if x >= 0 && y >= 0 {
        (x as u128, y as u128, 1)
    } else if x >= 0 && y < 0 {
        (x as u128, (-y) as u128, -1)
    } else if x < 0 && y >= 0 {
        ((-x) as u128, y as u128, -1)
    } else {
        ((-x) as u128, (-y) as u128, 1)
    }
}

pub fn mul_x18(x: i128, y: i128) -> i128 {
    let (mut x, mut y, sign) = signed_to_unsigned(x, y);
    if x > y {
        std::mem::swap(&mut x, &mut y);
    }
    (if y < UONE {
        x * y / UONE
    } else if x < UONE {
        let (c, d) = (y / UONE, y % UONE);
        x * c + x * d / UONE
    } else {
        let (a, b) = (x / UONE, x % UONE);
        let (c, d) = (y / UONE, y % UONE);
        a * c * UONE + a * d + b * c + b * d / UONE
    } as i128)
        * sign
}

// TODO: replacing mul_x18 with fmul_x18 can result in 1.5-2x speedup
pub fn fmul_x18(x: i128, y: i128) -> i128 {
    ((x as f64 * y as f64) / 1e18) as i128
}

pub fn div_x18(x: i128, y: i128) -> i128 {
    let (mut x, y, sign) = signed_to_unsigned(x, y);
    let mut ret = 0;
    if x >= y {
        ret += x / y * UONE;
        x %= y;
    }
    if x <= UONE {
        ret += x * UONE / y;
    } else {
        ret += (U256::from(x) * U256::from(UONE) / U256::from(y)).low_u128()
    }
    (ret as i128) * sign
}

pub fn pow_x18(x: i128, y: i128) -> i128 {
    let xf = x18_to_f64(x);
    let yf = x18_to_f64(y);
    let resultf = xf.powf(yf);
    let mut result = (resultf.trunc() as i128) * ONE_X18;
    result += (resultf.fract() * 1e18f64) as i128;
    result
}

pub fn sqrt_x18(x: i128) -> i128 {
    pow_x18(x, ONE_X18 / 2)
}

pub fn mul_div_x18(x: i128, y: i128, z: i128) -> i128 {
    (I256::from(x) * I256::from(y) / I256::from(z)).low_i128()
}

pub fn x18_to_f64_mil(x: i128) -> f64 {
    let x = x / 1_000_000;
    x18_to_f64(x)
}

pub fn x18_to_f64(x: i128) -> f64 {
    let mut result = (x / ONE_X18) as f64;
    result += (x % ONE_X18) as f64 / 1e18;
    result
}

pub fn f64_to_x18(x: f64) -> i128 {
    let mut result = (x.trunc() as i128) * ONE_X18;
    result += (x.fract() * 1e18) as i128;
    result
}

pub fn split_i256(x: I256) -> (i128, i128) {
    let base = I256::from(2).pow(127);
    ((x / base).as_i128(), (x % base).as_i128())
}

pub fn merge_i128(x: i128, y: i128) -> I256 {
    let base = I256::from(2).pow(127);
    I256::from(x) * base + I256::from(y)
}

pub fn i256_to_f64(x: I256) -> f64 {
    let (high, low) = split_i256(x);
    x18_to_f64(high) * (2.0_f64).powi(127) + x18_to_f64(low)
}

pub fn to_u128_x18(x: u128) -> u128 {
    x * (ONE_X18 as u128)
}

pub fn to_i128_x18(x: i128) -> i128 {
    x * ONE_X18
}

pub fn to_i128_fp(x: f64) -> i128 {
    (x * 10.0_f64.powi(9)) as i128 * 1000000000
}

pub fn to_i32_fp(x: f64) -> i32 {
    (x * 10.0_f64.powi(9)) as i32
}

pub fn to_u128_x6(x: u128) -> u128 {
    x * 1000000
}

pub fn to_i128_x6(x: i128) -> i128 {
    x * 1000000
}

pub fn fexp_x18(mut x: i128, y: i128) -> i128 {
    assert!(y >= 0);
    let mut i = 1;
    let mut ret = to_i128_x18(1);
    while i <= y {
        if i & y != 0 {
            ret = mul_x18(ret, x);
        }
        x = mul_x18(x, x);
        i <<= 1;
    }
    ret
}

pub fn fexp(mut x: i128, y: i128) -> i128 {
    assert!(y >= 0);
    let mut i = 1;
    let mut ret = 1;
    while i <= y {
        if i & y != 0 {
            ret *= x;
        }
        x *= x;
        i <<= 1;
    }
    ret
}

pub fn check_diff_gt_threshold_x18(left_x18: i128, right_x18: i128, threshold: f64) -> bool {
    let diff = (left_x18 - right_x18).abs();
    let percent_diff = div_x18(diff, right_x18);
    let percent_threshold: f64 = threshold * 1e18f64;
    percent_diff > percent_threshold as i128
}

pub fn check_within_range_x18(
    left_x18: i128,
    right_x18: i128,
    threshold_lower: f64,
    threshold_upper: f64,
) -> bool {
    let percent = div_x18(left_x18, right_x18);
    let percent_threshold_lower: f64 = threshold_lower * 1e18f64;
    let percent_threshold_upper: f64 = threshold_upper * 1e18f64;
    (percent > percent_threshold_lower as i128) && (percent < percent_threshold_upper as i128)
}

pub trait TryMath {
    fn try_add(self, v: i128) -> Result<i128>;
    fn try_div(self, v: i128) -> Result<i128>;
    fn try_mul(self, v: i128) -> Result<i128>;
    fn try_sub(self, v: i128) -> Result<i128>;
    fn try_rem(self, v: i128) -> Result<i128>;
    fn try_mul_x18(self, v: i128) -> Result<i128>;
    fn try_div_x18(self, v: i128) -> Result<i128>;
    fn try_sqrt_x18(self) -> Result<i128>;
}

impl TryMath for i128 {
    fn try_add(self, v: i128) -> Result<i128> {
        self.checked_add(v).ok_or(eyre!("Overflow: add"))
    }

    fn try_div(self, v: i128) -> Result<i128> {
        self.checked_div(v).ok_or(eyre!("Overflow: div"))
    }

    fn try_mul(self, v: i128) -> Result<i128> {
        self.checked_mul(v).ok_or(eyre!("Overflow: mul"))
    }

    fn try_sub(self, v: i128) -> Result<i128> {
        self.checked_sub(v).ok_or(eyre!("Overflow: sub"))
    }

    fn try_rem(self, v: i128) -> Result<i128> {
        self.checked_rem(v).ok_or(eyre!("Overflow: rem"))
    }

    fn try_mul_x18(self, v: i128) -> Result<i128> {
        Ok((I256::from(self) * I256::from(v) / I256::exp10(18)).as_i128())
    }

    fn try_div_x18(self, v: i128) -> Result<i128> {
        Ok((I256::from(self) * I256::exp10(18) / I256::from(v)).as_i128())
    }

    fn try_sqrt_x18(self) -> Result<i128> {
        let mut hi = 1;
        while hi.try_mul_x18(hi)? < self {
            hi = hi.try_mul(2)?;
        }
        let mut lo = hi.try_div(2)?;
        while lo < hi {
            let mid = lo.try_add(hi)?.try_div(2)?;
            if mid.try_mul_x18(mid)? < self {
                lo = mid.try_add(1)?;
            } else {
                hi = mid;
            }
        }
        Ok(lo)
    }
}

pub fn lp_value(balance: i128, x: i128, y: i128, supply: i128, price: i128) -> i128 {
    if supply == 0 {
        0
    } else {
        let pool_total_value = mul_x18(x, price) + y;
        mul_div_x18(balance, pool_total_value, supply)
    }
}
