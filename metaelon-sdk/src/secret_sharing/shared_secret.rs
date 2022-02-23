use std::borrow::{Borrow, BorrowMut};
use shamirsecretsharing as sss;

use crate::secret_sharing::data_block::DataBlock;
use crate::secret_sharing::secret_block::SecretBlock;

#[derive(Clone, Copy, Debug)]
pub struct SharedSecretConfig {
    pub count: u8,
    pub threshold: u8,
}

/// A shard of a secret kept by a user (consists of blocks, each block is 113 bytes)
#[derive(Debug)]
pub struct PersonalSecret {
    pub blocks: Vec<SecretBlock>,
}

#[derive(Debug)]
pub struct SharedSecret {
    config: SharedSecretConfig,
    secrets: Vec<PersonalSecret>,
}

pub struct SecretMessage {
    pub secret: String,
}

pub struct SharedSecretDecoder;

impl SharedSecretDecoder {
    pub fn restore(shared_secret: SharedSecret) -> SecretMessage {
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

/// https://mitxela.com/projects/shamirs_password_store
impl SharedSecret {
    pub fn new(config: SharedSecretConfig, secret_message: SecretMessage) -> Self {
        let mut secrets: Vec<PersonalSecret> = vec![];
        for _index in 0..config.count {
            let empty_secret = PersonalSecret { blocks: vec![] };
            secrets.push(empty_secret);
        }

        let mut shared_secret = SharedSecret { config, secrets };

        let chunks: Vec<Vec<u8>> = secret_message.secret.into_bytes()
            .chunks(63)
            .map(|s| s.into())
            .collect();

        for chunk in chunks {
            let block = DataBlock::new(chunk.to_vec());
            let shares: Vec<Vec<u8>> = Self::create_shares(config, block);

            for i in 0..config.count {
                let index = i as usize;
                let share: &[u8] = shares[index].borrow();
                let secret_block = SecretBlock::new(share.to_vec());

                let secrets: &mut [PersonalSecret] = shared_secret.secrets.borrow_mut();
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
    use std::borrow::{Borrow, BorrowMut};

    use crate::secret_sharing::data_block::DataBlock;
    use crate::secret_sharing::shared_secret::{
        SecretMessage, SharedSecret, SharedSecretConfig, SharedSecretDecoder,
    };

    #[test]
    fn split_and_restore_secret() {
        let mut message = String::new();
        for i in 0..100 {
            message.push_str(i.to_string().as_str());
            message.push_str("-")
        }
        //let message = String::from("1 -2-3-4-5-6-7-8-9-10-11-12-13-14-15-16-17-18-19-20-21-22-23-24-25-26-27-28-29-30-31-32-33");

        let config = SharedSecretConfig { count: 5, threshold: 3 };
        let secret_msg = SecretMessage { secret: message.clone() };

        let mut shared_secret = SharedSecret::new(config, secret_msg);
        shared_secret.secrets.remove(0);

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