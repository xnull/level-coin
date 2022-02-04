use std::str::FromStr;

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::{Address, Network};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tiny_hderive::bip32::ExtendedPrivKey;

//bitcoin rust sdk https://github.com/rust-bitcoin/rust-bitcoin/blob/master/src/util/address.rs
//

fn main() {
    let seed = generate_seed();

    // get the HD wallet seed as raw bytes
    let seed_bytes: &[u8] = seed.as_bytes();

    let secp = Secp256k1::new();

    let secret_key = SecretKey::from_slice(&[0xcd; 32])
        .expect("32 bytes, within curve order");

    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    let key = bitcoin::util::ecdsa::PublicKey::from_str(public_key.to_string().as_str())
        .expect("32 bytes, within curve order");

    let addr = Address::p2pkh(&key, Network::Bitcoin);
    //assert_eq!(&addr.to_string(), "1QJVDzdqb1VpbDK7uDeyVXy9mR27CJiyhY");

    println!("{:?}", &addr.to_string());

    generate_qr_code(&addr.to_string());
}

fn generate_qr_code(data: &str) {
    use qrcode_generator::QrCodeEcc;

    qrcode_generator::to_png_to_file(
        data,
        QrCodeEcc::High,
        1024,
        "first_qr.png").unwrap();
}

fn generate_seed() -> Seed {
    /// create a new randomly generated mnemonic phrase
    //let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let entropy = &[42; 16];
    let mnemonic = Mnemonic::from_entropy(entropy, Language::English).unwrap();

    /// get the phrase
    let phrase: &str = mnemonic.phrase();
    println!("phrase: {}", phrase);

    /// get the HD wallet seed
    let seed = Seed::new(&mnemonic, "");
    seed
}