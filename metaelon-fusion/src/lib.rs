use ed25519_dalek::PublicKey;

struct SecretMetaData {
    id: Hash,
    units: Vec<PublicKey>,
}

const HASH_BYTES: usize = 32;

#[derive(Default)]
pub struct Hash([u8; HASH_BYTES]);

#[cfg(test)]
mod tests {
    use ed25519_dalek::Keypair;
    use ed25519_dalek::Signature;
    use sha3::{Keccak256, Sha3_256, Shake128ReaderCore};
    use sha3::digest::core_api::XofReaderCoreWrapper;

    use rand_os::OsRng;

    use crate::{Hash, SecretMetaData};

    #[test]
    fn it_works() {
        let mut csprng = OsRng::default();
        //let mut r = StdRng::seed_from_u64(42);
        let keypair_1: Keypair = Keypair::generate(&mut csprng);
        let keypair_2: Keypair = Keypair::generate(&mut csprng);
        let keypair_3: Keypair = Keypair::generate(&mut csprng);

        let meta_data = SecretMetaData {
            id: Hash::default(),
            units: vec![keypair_1.public, keypair_2.public, keypair_3.public]
        };

        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn yay() {
        use sha3::{Digest, Sha3_256};

        use hex_literal::hex;

        let mut hasher = Sha3_256::new();
        hasher.update(b"abc");
        let mut res1 = hasher.finalize();
        //let hash = Hash(res1);
        let expected = "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532";
        assert_eq!(hex::encode(res1), expected);
    }
}
