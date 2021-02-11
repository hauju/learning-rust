


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password_length() {
        let text = "2012-03-14, 2013-01-01 and 2014-07-05";
        assert_eq!(text.len(), 37);
    }
}