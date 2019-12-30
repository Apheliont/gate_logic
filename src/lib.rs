#![doc(html_logo_url = "https://d30y9cdsu7xlg0.cloudfront.net/png/411962-200.png")]
/*!
 * This library implements simple logic gate operators
 * AND and XOR.
 */

/**
 * AND is a boolean AND operator
 * # Example
 * 0 AND 1 -> 0
 * 1 AND 0 -> 0
 * 0 AND 0 -> 0
 * 1 AND 1 -> 1
 */
pub fn and(a: u8, b: u8) -> Option<u8> {
    match (a, b) {
        (1, 1) => Some(1),
        (1, 0) | (0, 1) | (0, 0) => Some(0),
        _ => None,
    }
}

/** 
 * XOR is a boolean XOR operator
 * # Example
 * 0 XOR 1 -> 1
 * 1 XOR 0 -> 1
 * 0 XOR 0 -> 0
 * 1 XOR 1 -> 0
 */
pub fn xor(a: u8, b: u8) -> Option<u8> {
    match (a, b) {
        (0, 0) | (1, 1) => Some(0),
        (1, 0) | (0, 1) => Some(1),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{and, xor};
    #[test]
    fn test_and() {
        assert_eq!(and(1, 1), Some(1));
        assert_eq!(and(0, 1), Some(0));
        assert_eq!(and(1, 0), Some(0));
        assert_eq!(and(0, 0), Some(0));
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(1, 1), Some(0));
        assert_eq!(xor(0, 1), Some(1));
        assert_eq!(xor(1, 0), Some(1));
        assert_eq!(xor(0, 0), Some(0));
    }
}
