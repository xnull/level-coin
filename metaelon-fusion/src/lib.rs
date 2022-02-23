use ed25519_dalek::PublicKey;
use metaelon_sdk::secret_sharing::shared_secret::{SharedSecret, SharedSecretConfig};

struct SocialSecretConfig {
    units: Vec<PublicKey>,
}

struct SocialSecret {
    config: SocialSecretConfig,
    shared_secret: SharedSecret
}

const HASH_BYTES: usize = 32;

#[derive(Default)]
pub struct Hash([u8; HASH_BYTES]);

#[cfg(test)]
mod tests {
    use ed25519_dalek::Keypair;
    use ed25519_dalek::Signature;
    use sha3::{Keccak256, Sha3_256, Shake128ReaderCore};
    use metaelon_sdk::secret_sharing::shared_secret::{
        SecretMessage, SharedSecret, SharedSecretConfig, SharedSecretDecoder
    };

    use rand_os::OsRng;

    use crate::{Hash, SocialSecretConfig, SocialSecret};

    #[test]
    fn it_works() {
        let mut csprng = OsRng::default();
        //let mut r = StdRng::seed_from_u64(42);
        let keypair_1: Keypair = Keypair::generate(&mut csprng);
        let keypair_2: Keypair = Keypair::generate(&mut csprng);
        let keypair_3: Keypair = Keypair::generate(&mut csprng);

        let config = SharedSecretConfig { count: 5, threshold: 3 };

        let social_secret_config = SocialSecretConfig {
            units: vec![keypair_1.public, keypair_2.public, keypair_3.public],
        };

        let mut message = String::from("test");
        let secret_msg = SecretMessage { secret: message.clone() };

        let mut shared_secret = SharedSecret::new(config, secret_msg);

        let social_secret = SocialSecret {
            config: social_secret_config,
            shared_secret
        };

        //check public keys to restore a secret
        // implement communication protocol here???

        let secret_message = SharedSecretDecoder::restore(social_secret.shared_secret);
        assert_eq!(message, secret_message.secret.clone());
        println!("message: {:?}", secret_message.secret)
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
