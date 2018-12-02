use base64;
use std::collections::HashMap;
use std::str::from_utf8;

pub fn decode(input: HashMap<String, String>) -> HashMap<String, String> {
    let mut output = HashMap::new();

    for (key, val) in input.iter() {
        let value = base64::decode(val).expect(&format!("Error decoding Base64 in {}", key));
        let value_string =
            from_utf8(&value).expect(&format!("Error representing {} as UTF-8", key));
        output.insert(key.to_string(), value_string.to_string());
    }

    return output;
}

pub fn encode(input: HashMap<String, String>) -> HashMap<String, String> {
    let mut output = HashMap::new();

    for (key, val) in input.iter() {
        let value = base64::encode(val);
        output.insert(key.to_string(), value);
    }

    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    trait FromTuples {
        fn from_tuples(tuples: Vec<(&str, &str)>) -> Self;
    }

    impl FromTuples for HashMap<String, String> {
        fn from_tuples(tuples: Vec<(&str, &str)>) -> HashMap<String, String> {
            return tuples
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
        }
    }

    #[test]
    fn test_encode() {
        let input = HashMap::from_tuples(vec![("username", "admin"), ("password", "Password1!")]);
        let result = encode(input);
        let expected = HashMap::from_tuples(vec![
            ("username", "YWRtaW4="),
            ("password", "UGFzc3dvcmQxIQ=="),
        ]);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_decode() {
        let input = HashMap::from_tuples(vec![
            ("username", "YWRtaW4="),
            ("password", "UGFzc3dvcmQxIQ=="),
        ]);
        let result = decode(input);
        let expected =
            HashMap::from_tuples(vec![("username", "admin"), ("password", "Password1!")]);

        assert_eq!(expected, result);
    }
}
