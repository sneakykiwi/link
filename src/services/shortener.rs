use sha2::{Sha256, Digest};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use std::time::{SystemTime, UNIX_EPOCH};

const BASE62_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn generate_short_code(url: &str, salt: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    hasher.update(salt);
    let result = hasher.finalize();
    URL_SAFE_NO_PAD.encode(&result[..6])
}

pub fn generate_short_code_with_timestamp(url: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    hasher.update(&timestamp.to_be_bytes());
    let result = hasher.finalize();
    URL_SAFE_NO_PAD.encode(&result[..6])
}

pub fn generate_short_code_base62(url: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let result = hasher.finalize();
    
    let num = u64::from_be_bytes([
        result[0], result[1], result[2], result[3],
        result[4], result[5], result[6], result[7],
    ]);
    
    encode_base62(num)
}

fn encode_base62(mut num: u64) -> String {
    if num == 0 {
        return "0".to_string();
    }
    
    let mut result = String::new();
    while num > 0 {
        result.push(BASE62_CHARS[num as usize % 62] as char);
        num /= 62;
    }
    
    result.chars().rev().collect()
}

pub fn is_valid_custom_code(code: &str) -> bool {
    !code.is_empty() && 
    code.len() <= 20 && 
    code.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') &&
    !code.contains(' ')
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_generate_short_code_deterministic() {
        let url = "https://example.com";
        let salt = b"test_salt";
        
        let code1 = generate_short_code(url, salt);
        let code2 = generate_short_code(url, salt);
        
        assert_eq!(code1, code2);
        assert_eq!(code1.len(), 8);
    }

    #[test]
    fn test_generate_short_code_different_salts() {
        let url = "https://example.com";
        let salt1 = b"salt1";
        let salt2 = b"salt2";
        
        let code1 = generate_short_code(url, salt1);
        let code2 = generate_short_code(url, salt2);
        
        assert_ne!(code1, code2);
    }

    #[rstest]
    #[case("https://example.com")]
    #[case("https://github.com/rust-lang/rust")]
    #[case("https://very-long-domain-name.example.com/path")]
    fn test_generate_short_code_with_timestamp_different_each_time(#[case] url: &str) {
        let code1 = generate_short_code_with_timestamp(url);
        std::thread::sleep(std::time::Duration::from_millis(1));
        let code2 = generate_short_code_with_timestamp(url);
        
        assert_ne!(code1, code2);
        assert_eq!(code1.len(), 8);
        assert_eq!(code2.len(), 8);
    }

    #[test]
    fn test_generate_short_code_base62() {
        let url = "https://example.com";
        let code = generate_short_code_base62(url);
        
        assert!(!code.is_empty());
        assert!(code.chars().all(|c| BASE62_CHARS.contains(&(c as u8))));
    }

    #[test]
    fn test_encode_base62() {
        assert_eq!(encode_base62(0), "0");
        assert_eq!(encode_base62(61), "z");
        assert_eq!(encode_base62(62), "10");
    }

    #[rstest]
    #[case("valid_code", true)]
    #[case("valid-code", true)]
    #[case("ValidCode123", true)]
    #[case("_valid_code_", true)]
    #[case("", false)]
    #[case("invalid code", false)]
    #[case("invalid.code", false)]
    #[case("invalid@code", false)]
    fn test_is_valid_custom_code(#[case] code: &str, #[case] expected: bool) {
        assert_eq!(is_valid_custom_code(code), expected);
    }

    #[test]
    fn test_is_valid_custom_code_max_length() {
        let long_code = "a".repeat(21);
        assert_eq!(is_valid_custom_code(&long_code), false);
        
        let max_code = "a".repeat(20);
        assert_eq!(is_valid_custom_code(&max_code), true);
    }

    #[test]
    fn test_url_safe_encoding() {
        let test_urls = vec![
            "https://example.com",
            "https://example.com/path?query=value",
            "https://example.com/path/with/special/chars",
        ];
        
        for url in test_urls {
            let salt = b"test";
            let code = generate_short_code(url, salt);
            
            assert!(!code.contains('+'));
            assert!(!code.contains('/'));
            assert!(!code.contains('='));
        }
    }
} 