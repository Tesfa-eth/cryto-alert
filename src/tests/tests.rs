#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::block_on;

    #[test]
    fn test_validate_u64_input() {
        assert!(validate_u64_input("5").is_ok());
        assert!(validate_u64_input("five").is_err());
    }

    #[test]
    fn test_parse_price() {
        let response_data = r#"{
            "price": "123.45"
        }"#;
        assert_eq!(parse_price(response_data).unwrap(), "123.45");
    }

    #[test]
    fn test_send_get_request() {
        let fake_url = "https://httpbin.org/get";
        let response = block_on(send_get_request(fake_url));
        assert!(response.is_ok());
    }
}