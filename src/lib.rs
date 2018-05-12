#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate curve25519_dalek;
extern crate rand;
extern crate ristretto_bulletproofs;
extern crate wasm_bindgen;

use curve25519_dalek::scalar::Scalar;
use ristretto_bulletproofs::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_value(value: String) {
    let n = 64;
    // INSECURE
    let mut rng = rand::ChaChaRng::new_unseeded();
    rng.set_counter(0xbad, 0xbad);

    let v: u64 = value.parse().unwrap();

    let v_blinding = Scalar::random(&mut rng);
    alert(&format!("v_blinding: {:?}", v_blinding));

    // let generators = Generators::new(PedersenGenerators::default(), n, 1);
    // let mut transcript = ProofTranscript::new(b"RangeproofTest");
    // let proof = RangeProof::generate_proof(
    //     generators.share(0),
    //     &mut transcript,
    //     &mut rng,
    //     n,
    //     v,
    //     &v_blinding,
    // );

    alert(&format!("made a proof with value: {:?}", value));

    // let commit_v = PedersenGenerators::default().commit(Scalar::from_u64(v), v_blinding);
    // let result = proof.verify(&commit_v, generators.share(0), &mut transcript, &mut rng, n);
    // match result {
    // 	Ok(_) => alert(&format!("Your value verified correctly: {}", value)),
    // 	Err(_) => alert(&format!("Your value did not verify correctly: {}", value)),
    // }
}
