use k256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyingKey};
use rand_core::OsRng;

pub fn generate_sign_verify(message: String) -> bool {
    let msg = message.as_bytes();
    let signing_key = SigningKey::random(&mut OsRng);
    let signature: Signature = signing_key.sign(msg);
    let verify_key = VerifyingKey::from(&signing_key);
    let rtn = verify_key.verify(msg, &signature).is_ok();
    rtn
}
