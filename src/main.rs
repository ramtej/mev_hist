use num::bigint::BigUint;
use rand::RngCore;
use sha2::{Digest, Sha256 as Sha2_256};
use sha3::Sha3_256;
use std::{collections::BTreeMap, io::Result};

fn hist<D: Digest>(rolls: usize) -> Result<BTreeMap<u64, u64>> {
    let mut histogram = BTreeMap::new();
    let mut rng = rand::thread_rng();

    for _ in 0..rolls {
        let mut tx = [0u8; 1000];
        rng.fill_bytes(&mut tx);
        let h_tx = D::digest(tx);
        let h = BigUint::from_bytes_le(h_tx.as_slice());

        *histogram.entry(h.bits()).or_insert(0) += 1;
    }

    Ok(histogram)
}

fn main() -> Result<()> {
    let dice_rolls = 1 << 20;
    let sha2_256_hist = hist::<Sha2_256>(dice_rolls)?;
    let sha3_256_hist = hist::<Sha3_256>(dice_rolls)?;

    println!("SHA2_256(tx) Histogram, [bits->count]: ");
    for (bits, count) in &sha2_256_hist {
        println!("{}->{}", bits, count);
    }

    println!(" ");
    println!("SHA3_256(tx) Histogram [bits->count]: ");
    for (bits, count) in &sha3_256_hist {
        println!("{}->{}", bits, count);
    }

    Ok(())
}
