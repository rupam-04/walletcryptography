use crate::rd256::RD256;
use crate::bytes;
use std::str::FromStr;

#[derive(Debug, Clone)]

pub struct Point {
    pub x: RD256,
    pub y: RD256
}

impl Point {
    pub fn from_hex_coordinates(x: &str, y: &str) -> Self {
        return Point {
            x: RD256::from_str(x).unwrap(),
            y: RD256::from_str(y).unwrap()
        };
    }
    pub fn to_hex_string(&self) -> String {
        return format!("{} {}", self.x.to_string(), self.y.to_string());
    }
    pub fn is_zero_point(&self) -> bool {
        return self.x == RD256::from_str("0x0").unwrap() && self.y == RD256::from_str("0x0").unwrap();
    }
}

pub struct SECP256K1;

impl SECP256K1 {
    pub fn p() -> RD256 {
        return RD256::from_str("0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f").unwrap();
    }
    pub fn g() -> Point {
        return Point {
            x: RD256::from_str("0x79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798").unwrap(),
            y: RD256::from_str("0x483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8").unwrap()
        };
    }
    pub fn n() -> RD256 {
        return RD256::from_str("0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141").unwrap();
    }
}
