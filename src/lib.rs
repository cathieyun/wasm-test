#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate curve25519_dalek;
extern crate rand;
extern crate ristretto_bulletproofs;
extern crate wasm_bindgen;

use curve25519_dalek::constants;
use curve25519_dalek::scalar::Scalar;
use ristretto_bulletproofs::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[wasm_bindgen]
pub fn get_value(value: String) {
    let n = 32;
    // INSECURE
    let mut rng = rand::ChaChaRng::new_unseeded();
    rng.set_counter(0xbad, 0xbad);

    let v: u64 = value.parse().unwrap();

    let v_blinding = Scalar::random(&mut rng);
    log(&format!("v_blinding: {:?}", v_blinding));

    let generators = Generators::new(PedersenGenerators::default(), n, 1);
    let mut prover_transcript = ProofTranscript::new(b"RangeproofTest");

    let proof = RangeProof::prove_single(
        &generators,
        &mut prover_transcript,
        &mut rng,
        v,
        &v_blinding,
        n,
    ).unwrap();

    log(&format!("made a proof: {:?}", proof));

    let V = generators
        .pedersen_generators
        .commit(Scalar::from_u64(v), v_blinding);
    log(&format!("V: {:?}", V.compress()));

    let mut verifier_transcript = ProofTranscript::new(b"RangeproofTest");
    let result = proof.verify_single(&V, &generators, &mut verifier_transcript, &mut rng, n);
    log(&format!("result: {:?}", result));
    /*
    match result {
    	Ok(_) => log(&format!("Your value verified correctly: {}", value)),
    	Err(_) => log(&format!("Your value did not verify correctly: {}", value)),
    }
    */
}
