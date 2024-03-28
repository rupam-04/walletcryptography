use walletcryptography::rd256::RD256;
use walletcryptography::secp256k1::*;
use walletcryptography::base16;
use std::str::FromStr;

use secp256k1::{Secp256k1, Message, SecretKey, PublicKey};
use rand::prelude::*;

#[test]

fn ecc() {
    let mut rng = rand::thread_rng();
    let bytes = [0; 32];
    let random_bytes: Vec<u8> = bytes.into_iter().map(|_| rng.gen_range(0..=255)).collect::<Vec<u8>>();
    let pr_n = hex::encode(random_bytes);

    let pub_key1: Point = SECP256K1::pr_to_pub(&RD256::from_str(&pr_n).unwrap());
    let pub_key_str1 = pub_key1.to_hex_string();

    let secp = Secp256k1::new();
    let pr_key = SecretKey::from_str(&pr_n).expect("private-key");
    let pub_key2 = PublicKey::from_secret_key(&secp, &pr_key);
    let pub_key_str2 = base16::encode_bytes(&pub_key2.serialize_uncompressed());
    assert_eq!(pub_key_str1, pub_key_str2);
}