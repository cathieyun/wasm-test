#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
extern crate ristretto_bulletproofs;
extern crate rand;
extern crate curve25519_dalek;

use wasm_bindgen::prelude::*;
use ristretto_bulletproofs::*;
use curve25519_dalek::scalar::Scalar;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_value(value: String) {
	let n = 64;
	let mut rng = rand::thread_rng();
	let v: u64 = value.parse().unwrap();
	// let v_blinding = Scalar::random(&mut rng);
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

	alert(&format!("made a proof with value: {:?}", value))

	// let commit_v = PedersenGenerators::default().commit(Scalar::from_u64(v), v_blinding);
	// let result = proof.verify(&commit_v, generators.share(0), &mut transcript, &mut rng, n);
	// match result {
	// 	Ok(_) => alert(&format!("Your value verified correctly: {}", value)),
	// 	Err(_) => alert(&format!("Your value did not verify correctly: {}", value)),
	// }
}