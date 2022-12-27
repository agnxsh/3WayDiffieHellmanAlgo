#![allow(unused)]
extern crate bn;

extern crate rand;
use bn::{G1,Fr};
use crate::bn::Group;

extern crate bincode;
extern crate rustc_serialize;
use bincode::SizeLimit::Infinite;
use bincode::rustc_serialize::{encode};
use rustc_serialize::{Encodable};
use rustc_serialize::hex::{ ToHex};


//need to figure out a template class
pub fn into_hex(obj: S) -> Option<str>{
   encode(&obj, Infinite).ok().map(|e| e.to_hex())
}



fn main() {
   print!("Base point {:?}", into_hex(G1::one()).unwrap());

    let rng = &mut rand::thread_rng();

    // Construct private keys
    let a = Fr::random(rng);
    let b = Fr::random(rng);
    let c = Fr::random(rng);

    print!("\n\nAlice priv: {:?}\n\nBob priv {:?}\n\nCarol priv {:?}",  into_hex(Fr::a).unwrap(), into_hex(Fr::b).unwrap(), into_hex(Fr::c).unwrap());


// Public keys
    let alice_pk = G1::one() * a;
    let bob_pk = G1::one() * b;
    let carol_pk = G1::one() * c;

  
    print!("\n\nAlice pub: {:?}\n\nBob pub {:?}\n\nCarol pub {:?}",  into_hex(G1::alice_pk).unwrap(), into_hex(G1::bob_pk).unwrap(), into_hex(G1::carol_pk).unwrap());


    // Round 1
    let alice_1 = bob_pk * c; 
    let bob_1 = carol_pk * a;
    let carol_1 = alice_pk * b;

    // Round 2
    let alice_share =alice_1 * a;
    let bob_share = bob_1 * b;
    let carol_share = carol_1 * c;

    print!("\n\nAlice share: {:?}\n\nBob share {:?}\n\nCarol share {:?}",  into_hex(alice_share).unwrap(), into_hex(bob_share).unwrap(), into_hex(carol_share).unwrap());

 
}
