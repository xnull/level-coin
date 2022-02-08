use openssl::sha::sha256;

pub fn is_valid(difficulty: usize, hash: &String) -> bool {
    let mut chars = hash.chars();
    let mut accepted = true;
    for _i in 0..difficulty {
        let curr_letter = chars.next().unwrap();

        if curr_letter != '0' {
            accepted = false;
            break;
        }
    }
    accepted
}

pub fn find(difficulty: usize, tx_prefix: &str) -> PowStat {
    let mut counter = 0;

    let mut hash;
    let mut hash_str;

    loop {
        //if counter % 100_000 == 0 {
        //println!("iteration: {}", counter)
        //}

        counter = counter + 1;

        let mut tx_str = String::new();
        tx_str.push_str(tx_prefix);
        tx_str.push_str(counter.to_string().as_str());

        hash = sha256(tx_str.as_bytes());
        hash_str = hex::encode(hash);

        let is_valid = is_valid(difficulty, &hash_str);

        if is_valid {
            break;
        }
    }

    return PowStat { counter, hash: hash_str };
    //println!("counter: {}, hash: {}", counter, hash_str);
}

#[derive(Debug)]
pub struct PowStat {
    counter: usize,
    hash: String,
}
