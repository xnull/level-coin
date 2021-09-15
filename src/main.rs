use openssl::sha;
use std::borrow::Borrow;

fn main() {
    use openssl::sha::sha256;

    let mut counter = 0;
    let mut hash;
    loop {
        counter = counter + 1;

        hash = sha256(counter.to_string().as_bytes());
        let hash_str = hex::encode(hash);
        //println!("{}", hash_str);

        let first_letter = hash_str.chars().next().unwrap();
        //println!("{}", first_letter);

        if first_letter == '0' {
            break;
        }
    }
    println!("hash = {}", hex::encode(hash));


}
