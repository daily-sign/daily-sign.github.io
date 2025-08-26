use ed25519::Signature;
use ed25519::signature::{Signer, Verifier};
use ed25519_dalek::pkcs8::EncodePublicKey;
use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;
use ed25519_dalek::{SECRET_KEY_LENGTH, SigningKey};

use base64ct::{Base64, Encoding};

/*
fn main() {
    let secret_key_bytes: [u8; SECRET_KEY_LENGTH] = [
        157, 097, 177, 157, 239, 253, 090, 096, 186, 132, 074, 244, 146, 236, 044, 196, 068, 073,
        187, 105, 123, 050, 105, 025, 112, 059, 172, 003, 028, 174, 127, 096,
    ];

    let signing_key: SigningKey = SigningKey::from_bytes(&secret_key_bytes);

    let message: &[u8] = b"This is a test of the tsunami alert system.";
    let signature: Signature = signing_key.sign(message);

    let verifying_key = signing_key.verifying_key();

    print!("{}\n", Base64::encode_string(verifying_key.as_bytes()));

    print!("{}\n", Base64::encode_string(&signature.to_bytes()));
}
*/
