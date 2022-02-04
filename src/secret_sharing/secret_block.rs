//block of data which contains a chunk of data converted to a shared secret
#[derive(Clone, Debug)]
pub struct SecretBlock {
    pub block: Vec<u8>,
}

impl SecretBlock {
    pub fn new(secret_vec: Vec<u8>) -> Self {
        if secret_vec.len() != 113 {
            panic!("Invalid data")
        }

        SecretBlock { block: secret_vec }
    }
}