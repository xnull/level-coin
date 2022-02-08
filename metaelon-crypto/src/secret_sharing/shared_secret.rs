use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;

use shamirsecretsharing as sss;

use crate::data_block::DataBlock;
use crate::secret_block::SecretBlock;

#[derive(Clone, Copy, Debug)]
struct SharedSecretConfig {
    count: u8,
    threshold: u8,
}

/// A shard of a secret kept by a user (consists of blocks 113 bytes)
#[derive(Debug)]
pub struct Secret {
    pub blocks: Vec<SecretBlock>,
}

#[derive(Debug)]
struct SharedSecret {
    config: SharedSecretConfig,
    secrets: Vec<Secret>,
}

struct SecretMessage {
    secret: String,
}

struct SharedSecretDecoder;

impl SharedSecretDecoder {
    fn restore(shared_secret: SharedSecret) -> SecretMessage {
        let mut secret_message = String::new();

        let size = shared_secret.secrets[0].blocks.len();
        for i in 0..size {
            let shares: Vec<Vec<u8>> = shared_secret.secrets
                .iter()
                .map(|secret| secret.blocks[i].block.to_vec())
                .collect();

            let mut restored: Vec<u8> = shamirsecretsharing::combine_shares(&shares)
                .unwrap().unwrap();

            let length = restored.remove(0);
            let restored: &[u8] = restored.split_at(length as usize).0;

            let restored_str = String::from_utf8(restored.to_vec()).unwrap();
            secret_message.push_str(restored_str.as_str())
        }

        SecretMessage { secret: secret_message }
    }
}

impl SharedSecret {
    fn new(config: SharedSecretConfig, secret_message: SecretMessage) -> Self {
        let mut secrets: Vec<Secret> = vec![];
        for _index in 0..config.count {
            let empty_secret = Secret { blocks: vec![] };
            secrets.push(empty_secret);
        }

        let mut shared_secret = SharedSecret { config, secrets };

        let chunks: Vec<Vec<u8>> = secret_message.secret.into_bytes()
            .chunks(63)
            .map(|s| s.into())
            .collect();

        for chunk in chunks {
            let mut block = DataBlock::new(chunk.to_vec());
            let mut shares: Vec<Vec<u8>> = Self::create_shares(config, block);

            for i in 0..config.count {
                let index = i as usize;
                let share: &[u8] = shares[index].borrow();
                let secret_block = SecretBlock::new(share.to_vec());

                let secrets: &mut [Secret] = shared_secret.secrets.borrow_mut();
                //let blocks: &mut [SecretBlock] = secrets[index].blocks.borrow_mut();// secret.blocks.borrow_mut();
                //blocks.push(secret_block);
                secrets[index].blocks.push(secret_block);
            }
        }

        shared_secret
    }

    fn create_shares(config: SharedSecretConfig, block: DataBlock) -> Vec<Vec<u8>> {
        let bytes: Vec<u8> = block.to_bytes();
        sss::create_shares(&bytes, config.count, config.threshold)
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;

    use crate::data_block::DataBlock;
    use crate::shared_secret::{SecretMessage, SharedSecret, SharedSecretConfig, SharedSecretDecoder};

    #[test]
    fn split_and_restore_secret() {
        let mut message = String::new();
        for i in 0..10 {
            message.push_str(i.to_string().as_str());
        }
        //let message = String::from("1 -2-3-4-5-6-7-8-9-10-11-12-13-14-15-16-17-18-19-20-21-22-23-24-25-26-27-28-29-30-31-32-33");

        let config = SharedSecretConfig { count: 5, threshold: 3 };
        let shared_secret = SharedSecret::new(config, SecretMessage { secret: message.clone() });
        let secret_message = SharedSecretDecoder::restore(shared_secret);
        assert_eq!(message, secret_message.secret.clone());
        println!("message: {:?}", secret_message.secret)
    }

    #[test]
    fn shamir_test() {
        let block = DataBlock::new(vec![42; 63]);
        let data: Vec<u8> = vec![63, 104, 101, 121, 95, 104, 101, 121, 95, 104, 101, 121, 95, 104, 101, 121, 95, 104, 101, 121, 95, 104, 101, 121, 95, 104, 101, 121, 95, 104, 101, 121, 95, 104, 101, 121, 95, 104, 101, 121, 95, 104, 101, 121, 95, 104, 101, 121, 95, 121, 97, 121, 95, 104, 101, 121, 95, 104, 101, 121, 95, 104, 101, 121];//block.to_bytes();

        let count = 5;
        let threshold = 3;
        let mut shares: Vec<Vec<u8>> = shamirsecretsharing::create_shares(&data, count, threshold)
            .unwrap();

        for share in &shares {
            println!("share size: {:?}", share.len());
        }

        // Lose a share (for demonstration purposes)
        shares.remove(0);

        // We still have 4 shares, so we should still be able to restore the secret
        let restored = shamirsecretsharing::combine_shares(&shares).unwrap();
        assert_eq!(restored, Some(data));

        // Lose a share (for demonstration purposes)
        shares.remove(0);

        // If we lose another share the secret is lost
        shares.remove(0);
        let restored2 = shamirsecretsharing::combine_shares(&shares).unwrap();
        assert_eq!(restored2, None);
    }
}