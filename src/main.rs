use openssl::sha::sha256;

fn main() {
    let difficulty = 7;

    let mut counter = 0;

    let mut hash;
    let mut hash_str;

    loop {
        if counter % 100_000 == 0 {
            println!("iteration: {}", counter)
        }

        counter = counter + 1;

        hash = sha256(counter.to_string().as_bytes());
        hash_str = hex::encode(hash);

        let is_valid = is_valid(difficulty, &hash_str);

        if is_valid {
            break;
        }
    }

    println!("counter: {}, hash: {}", counter, hash_str);
}

fn is_valid(difficulty: usize, hash: &String) -> bool {
    let mut chars = hash.chars();
    let mut accepted = true;
    for i in 0..difficulty {
        let curr_letter = chars.next().unwrap();

        if curr_letter != '0' {
            accepted = false;
            break;
        }
    }
    accepted
}

#[cfg(test)]
mod tests {
    use crate::is_valid;

    #[test]
    fn test_is_valid() {
        let actual_result = is_valid(2, &String::from("00123"));
        assert_eq!(actual_result, true);
    }
}
