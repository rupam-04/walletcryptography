use std::str::FromStr;
use primitive_types::U256;
use hex;
use crate::bytes;

#[derive(Debug, Clone)]

pub struct RD256 {
    pub v: U256
}

#[derive(Debug, PartialEq, Eq)]

pub struct RD256ParseError;

impl FromStr for RD256 {
    type Err = RD256ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match U256::from_str_radix(s, 16) {
            Ok(n) => return Ok(Self { v: n }),
            Err(_) => return Err(RD256ParseError)
        }
    }
}
impl ToString for RD256 {
    fn to_string(&self) -> String {
        let mut bytes: [u8; 32] = [0; 32];
        self.v.to_big_endian(&mut bytes);
        return hex::encode(bytes);
    }
}


impl RD256 {

    pub fn from_bytes(bs: &[u8]) -> Self {
        assert!(bs.len() <= 32, "big-endian");

        return Self {
            v: U256::from_big_endian(bs)
        };
    }
    pub fn to_bytes(&self, r: &mut [u8]) {
        self.v.to_big_endian(r);
    }

    pub fn zero() -> Self {
        return Self::from_str("0x0").unwrap();
    }
    pub fn one() -> Self {
        return Self::from_str("0x1").unwrap();
    }

    pub fn add_mod(&self, b: &RD256, p: &RD256) -> RD256 {
        let x1: U256 = self.v.checked_rem(p.v).expect("modulo");
        let x2: U256 = b.v.checked_rem(p.v).expect("modulo");

        let (mut x3, over) = x1.overflowing_add(x2);

        if over {
            x3 = x3.checked_add(
                U256::MAX.checked_sub(p.v).expect("sub")
                    .checked_add(U256::from_big_endian(&[1])).expect("conversion")
            ).expect("add");
        }
        x3 = x3.checked_rem(p.v).expect("modulo");
        

        return Self { v: x3 };   
    }

    pub fn sub_mod(&self, b: &RD256, p: &RD256) -> RD256 {
        let x1: U256 = self.v.checked_rem(p.v).expect("modulo");
        let x2: U256 = b.v.checked_rem(p.v).expect("modulo");
        
        return Self{ v: x1 }.add_mod(&Self{ v: (p.v - x2) }, p);   
    }

    pub fn mul_mod(&self, b: &RD256, p: &RD256) -> RD256 {
        let x1: RD256 = Self{ v: self.v.checked_rem(p.v).expect("modulo") };
        let x2: RD256 = Self{ v: b.v.checked_rem(p.v).expect("modulo") };

        let mut base: RD256 = Self::zero();

        let seq: Self;
        let adder: Self;

        if x1.v < x2.v {
            seq = x1.clone();
            adder = x2.clone();
        }
        else {
            seq = x2.clone();
            adder = x1.clone();
        }

        let mut seq_bytes: [u8; 32] = [0; 32];
        seq.to_bytes(&mut seq_bytes);

        let mut seq_binaries: Vec<u8> = vec![];
        bytes::bytes_to_binary(&seq_bytes, &mut seq_binaries);

        let mut on: bool = false;
        for d in seq_binaries.into_iter() {
            if on {
                base = base.add_mod(&base, p);
            }
            if d > 0 {
                on = true;
                base = base.add_mod(&adder, p);
            }
        }

        return base;   
    }

    pub fn exp_mod(&self, e: &RD256, p: &RD256) -> RD256 {
        let seq: RD256 = e.clone();
        let multiplier = RD256{ v: self.v.checked_rem(p.v).expect("modulo") };   

        let mut base: RD256 = RD256::one();

        let mut seq_bytes = [0; 32];
        seq.to_bytes(&mut seq_bytes);

        let mut seq_binaries: Vec<u8> = vec![];
        bytes::bytes_to_binary(&seq_bytes, &mut seq_binaries);

        let mut on: bool = false;
        for d in seq_binaries.into_iter() {
            if on {
                base = base.mul_mod(&base, p);
            }
            if d > 0 {
                on = true;
                base = base.mul_mod(&multiplier, p);
            }
        }

        return base;

    }

    pub fn div_mod(&self, b: &RD256, p: &RD256) -> RD256 {
        assert!(p.v - 2 > U256::from_big_endian(&[0]));
        return self.mul_mod(&b.exp_mod(&RD256{ v: p.v - 2 }, p), p);
    }
} 

impl PartialEq for RD256 {
    fn eq(&self, other: &Self) -> bool {
        return self.v == other.v;
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_add_mod() {
        let a = RD256::from_str("0x1").unwrap();
        let b = RD256::from_str("0x2").unwrap();
        let p = RD256::from_str("0x7").unwrap();
        let c = a.add_mod(&b, &p);
        assert_eq!(c.v, U256::from_str("0x3").unwrap());
    }

    #[test]
    fn test_sub_mod() {
        let a = RD256::from_str("0x1").unwrap();
        let b = RD256::from_str("0x2").unwrap();
        let p = RD256::from_str("0x7").unwrap();
        let c = a.sub_mod(&b, &p);
        assert_eq!(c.v, U256::from_str("0x6").unwrap());
    }

    #[test]
    fn test_mul_mod() {
        let a = RD256::from_str("0x2").unwrap();
        let b = RD256::from_str("0x8").unwrap();
        let p = RD256::from_str("0x7").unwrap();
        let c = a.mul_mod(&b, &p);
        assert_eq!(c.v, U256::from_str("0x2").unwrap());
    }

    #[test]
    fn test_exp_mod() {
        let a = RD256::from_str("0x2").unwrap();
        let b = RD256::from_str("0x3").unwrap();
        let p = RD256::from_str("0x7").unwrap();
        let c = a.exp_mod(&b, &p);
        assert_eq!(c.v, U256::from_str("0x1").unwrap());
    }

    #[test]
    fn test_div_mod() {
        let a = RD256::from_str("0x2e").unwrap();
        let b = RD256::from_str("0xa").unwrap();
        let p = RD256::from_str("0xb").unwrap();
        let c = a.div_mod(&b, &p);
        assert_eq!(c.v, U256::from_str("0x9").unwrap());
    }
}