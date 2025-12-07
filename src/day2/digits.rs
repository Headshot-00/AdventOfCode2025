use std::ops::{Add, Div, Mul, Rem, Sub};

const PROPER_DIVISORS: [&[u32]; 20] = [
    &[],              // 0
    &[],              // 1
    &[1],             // 2
    &[1],             // 3
    &[1, 2],          // 4
    &[1],             // 5
    &[1, 2, 3],       // 6
    &[1],             // 7
    &[1, 2, 4],       // 8
    &[1, 3],          // 9
    &[1, 2, 5],       // 10
    &[1],             // 11
    &[1, 2, 3, 4, 6], // 12
    &[1],             // 13
    &[1, 2, 7],       // 14
    &[1, 3, 5],       // 15
    &[1, 2, 4, 8],    // 16
    &[1],             // 17
    &[1, 2, 3, 6, 9], // 18
    &[1],             // 19
];

const POW10: [u64; 20] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
    10_000_000_000_000_000_000,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DigitsU64(u8);

impl DigitsU64 {
    pub fn new(d: u32) -> Option<Self> {
        if (1..=19).contains(&d) {
            Some(DigitsU64(d as u8))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn idx(self) -> usize {
        self.0 as usize
    }
}

impl Add<u32> for DigitsU64 {
    type Output = DigitsU64;

    fn add(self, rhs: u32) -> DigitsU64 {
        DigitsU64((self.0 as u32 + rhs) as u8)
    }
}

impl Sub<u32> for DigitsU64 {
    type Output = DigitsU64;

    fn sub(self, rhs: u32) -> DigitsU64 {
        DigitsU64((self.0 as u32 - rhs) as u8)
    }
}

impl Mul<u32> for DigitsU64 {
    type Output = DigitsU64;

    fn mul(self, rhs: u32) -> DigitsU64 {
        DigitsU64((self.0 as u32 * rhs) as u8)
    }
}

impl Div<u32> for DigitsU64 {
    type Output = DigitsU64;

    fn div(self, rhs: u32) -> DigitsU64 {
        DigitsU64((self.0 as u32 / rhs) as u8)
    }
}
impl Rem<u32> for DigitsU64 {
    type Output = DigitsU64;

    fn rem(self, rhs: u32) -> DigitsU64 {
        DigitsU64((self.0 as u32 % rhs) as u8)
    }
}

impl From<u32> for DigitsU64 {
    fn from(d: u32) -> DigitsU64 {
        DigitsU64(d as u8)
    }
}

pub fn num_digits(mut n: u64) -> u32 {
    let mut digits = 1;
    while n >= 10 {
        n /= 10;
        digits += 1;
    }
    digits
}

#[inline(always)]
pub fn pow10(d: DigitsU64) -> u64 {
    POW10[d.idx()]
}

#[inline(always)]
pub fn pow10_minus1(d: DigitsU64) -> u64 {
    POW10[d.idx()] - 1
}

#[inline(always)]
pub fn divisors_for(digits: DigitsU64) -> &'static [u32] {
    &PROPER_DIVISORS[digits.idx()]
}

pub fn is_minimal_block(block: u64, block_digits: DigitsU64) -> bool {
    let total_minus1 = pow10_minus1(block_digits);
    for &d in divisors_for(block_digits) {
        let rep = total_minus1 / (pow10_minus1(DigitsU64::new(d).unwrap()));
        if block % rep == 0 {
            return false;
        }
    }
    true
}

pub fn digit_ranges(num1: u64, num2: u64) -> impl Iterator<Item = (u64, u64, DigitsU64)> {
    let start_digits = num_digits(num1);
    let end_digits = num_digits(num2);

    (start_digits..=end_digits).filter_map(move |d| {
        let lower = 10u64.pow(d - 1); // Lower bound for the d-digit numbers
        let upper = 10u64.pow(d) - 1; // Upper bound for the d-digit numbers

        // Adjust the bounds to fit within num1 and num2
        let start = num1.max(lower);
        let end = num2.min(upper);

        // Only add ranges where start <= end
        if start <= end {
            Some((start, end, DigitsU64::new(d).unwrap()))
        } else {
            None
        }
    })
}
