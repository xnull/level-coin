use level_coin::pow::validator;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let stat = validator::find(4, "yay");

    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
    println!("{:?}", stat)
}


#[cfg(test)]
mod tests {
    use level_coin::pow::validator;

    #[test]
    fn test_is_valid() {
        let actual_result = validator::is_valid(2, &String::from("00123"));
        assert_eq!(actual_result, true);
    }
}
