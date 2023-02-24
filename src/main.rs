use k256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyingKey};
use rand_core::OsRng;

use std::env;

fn main() {
    let mut message = String::from("Hello world!");

    let args: Vec = env::args().collect();
    if args.len() > 1 {
        message = args[1].clone();
    }

    println!("Message: {}", message);

    let mut msg = message.as_bytes();

    // Signing
    let signing_key = SigningKey::random(&mut OsRng);
    let sk = signing_key.to_bytes();
    println!("\nSigning key: {:x?}", hex::encode(sk));

    let signature: Signature = signing_key.sign(msg);
    println!("\nSignature key: {:x?}", hex::encode(signature));

    let verify_key = VerifyingKey::from(&signing_key);
    // Serialize with `::to_encoded_point()`
    let vk = verify_key.to_bytes();
    println!("\nVerifying key: {:x?}", hex::encode(vk));

    let rtn = verify_key.verify(msg, &signature).is_ok();

    if rtn == true {
        println!("\nMessage '{0}' signature correct", message);
    } else {
        println!("\nMessage '{0}' signature incorrect", message);
    }

    msg = "hello".as_bytes();

    let rtn = verify_key.verify(msg, &signature).is_ok();

    if rtn == true {
        println!("\nWith 'hello', message signature correct");
    } else {
        println!("\nWith 'hello', message signature incorrect");
    }
}
