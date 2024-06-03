//! A program that takes a number `n` as input, and writes if `n` is prime as an output.
use sp1_sdk::{utils, ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup a tracer for logging.
    utils::setup_logger();

    let mut stdin = SP1Stdin::new();

    // Generate and verify the proof
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, stdin).unwrap();

    let sha256_hw_hash = proof.public_values.read::<[u8; 32]>();
    println!("Sha256 \"hello world\" {:x?}", sha256_hw_hash);
    let keccak256_hw_hash = proof.public_values.read::<[u8; 32]>();
    println!("Keccak256 \"hello world\" {:x?}", keccak256_hw_hash);

    client.verify(&proof, &vk).expect("verification failed");

    // Save the proof
    proof
        .save("proof-wrong-hash.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
