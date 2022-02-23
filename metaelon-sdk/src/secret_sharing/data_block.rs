/**
 * 64 byte chunk of the chunk of data that will be split by shamir secret sharing method
 */
pub struct DataBlock {
    block: Vec<u8>,
}

impl DataBlock {
    pub fn new(block: Vec<u8>) -> Self {
        if block.len() > 63 {
            panic!("invalid data");
        }

        DataBlock { block }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data_block: Vec<u8> = vec![0; 64];
        data_block[0] = self.block.len() as u8;

        let mut i = 1;
        for byte in &self.block {
            data_block[i] = *byte;
            i = i + 1;
        }

        data_block
    }
}

#[cfg(test)]
mod test {
    use crate::secret_sharing::data_block::DataBlock;

    #[test]
    fn test_to_bytes() {
        let block = DataBlock::new(vec![42; 63]);

        let mut expected: Vec<u8> = vec![];
        expected.append(&mut vec![63]);
        expected.append(&mut vec![42; 63]);

        assert_eq!(expected, block.to_bytes());
    }

    #[test]
    fn test_to_bytes_with_zeros() {
        let block = DataBlock::new(vec![42; 1]);

        let mut expected: Vec<u8> = vec![];
        expected.append(&mut vec![1]);
        expected.append(&mut vec![42]);
        expected.append(&mut vec![0; 62]);

        assert_eq!(expected, block.to_bytes());

        println!("{:?}", block.to_bytes())
    }
}