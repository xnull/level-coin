use level_coin::pow::validator;

fn main() {
    validator::find(7, "yay")
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
