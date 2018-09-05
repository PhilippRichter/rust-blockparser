extern crate bitcoin;
extern crate secp256k1;

use bitcoin::util::address::Address;
use bitcoin::network::constants::Network;
use secp256k1::key::PublicKey;
use secp256k1::key::SecretKey;
use secp256k1::Secp256k1;

fn main() {
    let secp = Secp256k1::new();
    let secret = SecretKey::from_slice(
        &Secp256k1::new(),
        b"11111111111111111111111111111111"
    ).unwrap();

    let pk = PublicKey::from_secret_key(
        &secp,
        &secret
    );

    let address = Address::p2pkh(
        &pk,
        Network::Bitcoin
    );
    println!("{:?}", address);
}
