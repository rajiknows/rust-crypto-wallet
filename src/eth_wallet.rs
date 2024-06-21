use anyhow::Result;
use core::result::Result::Ok;
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter},
};
use tiny_keccak::keccak256;
use web3::types::Address;

pub fn generate_key_pairs() -> (SecretKey, PublicKey) {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(1111);
    secp.generate_keypair(&mut rng)
}

pub fn public_key_address(publickey: &PublicKey) -> Address {
    let publickey = publickey.serialize_uncompressed();

    let hash = keccak256(&publickey[1..]);

    Address::from_slice(&hash[12..])
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub secret_key: String,
    pub public_key: String,
    pub address: String,
}

impl Wallet {
    pub fn new(secret_key: &SecretKey, publickey: &PublicKey) -> Self {
        let addr: Address = public_key_address(&publickey);
        Wallet {
            secret_key: format!("{:?}", secret_key),
            public_key: format!("{:?}", publickey),
            address: format!("{:?}", addr),
        }
    }

    pub fn save_to_file(&self, filepath: &str) -> Result<()> {
        let file = OpenOptions::new().write(true).create(true).open(filepath)?;
        let buf_writer = BufWriter::new(file);

        serde_json::to_writer_pretty(buf_writer, &self)?;
        Ok(())
    }

    pub fn export_from_file(filepath: &str) -> Result<Self> {
        let file = OpenOptions::new().read(true).open(&filepath);

        match file {
            Ok(file) => {
                let buffreader = BufReader::new(file);
                let wallet: Wallet = serde_json::from_reader(buffreader)?;
                Ok(wallet)
            }
            Err(e) => {
                println!("cant find a file with this name");
                Err(e.into())
            }
        }
    }
}
