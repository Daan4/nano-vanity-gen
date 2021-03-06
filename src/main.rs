use bitvec::prelude::*;
use blake2b_simd::{Hash, Params};
use byteorder::{BigEndian, WriteBytesExt};
use ed25519_dalek::{PublicKey, SecretKey};
use once_cell::sync::Lazy;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use regex::Regex;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Instant;

// --SETTINGS--

// The text the vanity address should start with, empty to ignore
const STARTS_WITH: &str = "";

// The text the vanity address should end with, empty to ignore
const ENDS_WITH: &str = "";

// The text the vanity address should contain, empty to ignore
const CONTAINS: &str = "daan";

// Number of threads, set to number of logical processors
const NUM_THREADS: u32 = 4;

fn main() {
    // benchmark();

    let re = Regex::new(r"[13456789abcdefghijkmnopqrstuwxyz]{0,59}$").unwrap();
    if STARTS_WITH != "" && !re.is_match(STARTS_WITH) {
        println!("Invalid STARTS_WITH setting");
        return;
    }
    if ENDS_WITH != "" && !re.is_match(ENDS_WITH) {
        println!("Invalid ENDS_WITH setting");
        return;
    }
    if CONTAINS != "" && !re.is_match(CONTAINS) {
        println!("Invalid CONTAINS setting");
        return;
    }

    let mut ts: Vec<thread::JoinHandle<()>> = vec![];
    let (tx, rx): (Sender<()>, Receiver<()>) = channel();

    for _ in 0..NUM_THREADS {
        let tx_clone = tx.clone();
        ts.push(thread::spawn(|| {
            let rng = &mut ChaCha20Rng::from_entropy();
            inner(&mut rng.clone(), tx_clone);
        }));
    }

    rx.recv().unwrap();
}

fn inner(rng: &mut ChaCha20Rng, tx: Sender<()>) {
    loop {
        let seed = generate_random_seed(rng);
        let private_key = derive_private_key(seed, 0);
        let public_key = derive_public_key(private_key);
        let address = derive_address(public_key);
        if address[1..].starts_with(STARTS_WITH)
            && address.ends_with(ENDS_WITH)
            && address.contains(CONTAINS)
        {
            println!("nano_{}\n{}", address, bytes_to_hexstring(&seed));
            tx.send(()).unwrap();
            break;
        }
    }
}

fn benchmark() {
    let rng = &mut ChaCha20Rng::from_entropy();

    let mut count = 0;
    let runs = 20000;
    let now = Instant::now();
    while count < runs {
        let seed = generate_random_seed(rng);
        let private_key = derive_private_key(seed, 0);
        let public_key = derive_public_key(private_key);
        derive_address(public_key);
        count += 1;
    }
    println!(
        "{} runs: {}s",
        runs,
        now.elapsed().as_millis() as f32 / 1000.0
    );
}

/// Generate random seed
fn generate_random_seed(rng: &mut ChaCha20Rng) -> [u8; 32] {
    let mut seed = [0; 32];
    for i in 0..32 {
        seed[i] = rng.gen_range(0..16) << 4 | rng.gen_range(0..16);
    }
    seed
}

/// Derive private key from seed and index
fn derive_private_key(seed: [u8; 32], index: u32) -> Hash {
    let mut wtr = vec![];
    wtr.write_u32::<BigEndian>(index).unwrap();
    Params::new()
        .hash_length(32)
        .to_state()
        .update(&seed)
        .update(&wtr)
        .finalize()
}

/// Derive public key from private key
fn derive_public_key(private_key: Hash) -> PublicKey {
    PublicKey::from(&SecretKey::from_bytes(private_key.as_bytes()).unwrap())
}

/// Derive address from public key
fn derive_address(public_key: PublicKey) -> String {
    // Code based on Feeless project implementation
    let mut address = String::with_capacity(65);

    const PKP_CAPACITY: usize = 4 + 8 * 32 + 4;
    let mut bits: BitVec<Msb0, u8> = BitVec::with_capacity(PKP_CAPACITY);
    let pad: BitVec<Msb0, u8> = bitvec![Msb0, u8; 0; 4];
    bits.extend_from_bitslice(&pad);
    bits.extend_from_raw_slice(public_key.as_bytes());
    let public_key_part = encode_nano_base_32(&bits);
    address.push_str(&public_key_part);

    let result = Params::new()
        .hash_length(5)
        .to_state()
        .update(public_key.as_bytes())
        .finalize();
    let bits: BitVec<Msb0, u8> = BitVec::from_iter(result.as_bytes().iter().rev());
    let checksum = encode_nano_base_32(&bits);
    address.push_str(&checksum);
    address
}

// Function based on Feeless project implementation
const ALPHABET: &str = "13456789abcdefghijkmnopqrstuwxyz";
static ALPHABET_VEC: Lazy<Vec<char>> = Lazy::new(|| ALPHABET.chars().collect());
const ENCODING_BITS: usize = 5;

fn encode_nano_base_32(bits: &BitSlice<Msb0, u8>) -> String {
    let mut s = String::new();
    for idx in (0..bits.len()).step_by(ENCODING_BITS) {
        let chunk: &BitSlice<Msb0, u8> = &bits[idx..idx + ENCODING_BITS];
        let value: u8 = chunk.load_be();
        let char = ALPHABET_VEC[value as usize];
        s.push(char);
    }
    s
}

const HEX: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f",
];

fn bytes_to_hexstring(bytes: &[u8]) -> String {
    let mut buf = String::new();
    for x in bytes.iter() {
        buf += HEX[(*x >> 4) as usize];
        buf += HEX[0x0F & *x as usize];
    }
    buf
}
