use fleetcore::{BaseInputs, BaseJournal};
use risc0_zkvm::{guest::env, sha::Digest};
use sha2::{Digest as _, Sha256};

fn main() {

    // read the input
    let mut input: BaseInputs = env::read();
    // TODO: do something with the input
    let output= none;

    env::commit(&output);
}