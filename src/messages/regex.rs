use regex::Regex;
use once_cell::sync::Lazy;
pub static ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z]{3,3}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static ANY_BICDEC2014_IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static BICFIDEC2014_IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static COUNTRY_CODE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z]{2,2}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static EXACT2_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9]{2}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static EXACT4_ALPHA_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9]{4}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static IBAN2007_IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static LEIIDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z0-9]{18,18}[0-9]{2,2}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static PHONE_NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\+[0-9]{1,3}-[0-9()+\-]{1,30}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static UUIDV4_IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static ACTIVE_CURRENCY_CODE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z]{3,3}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static MAX15_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9]{1,15}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static EXACT1_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9]$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static EXACT3_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9]{3}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static ISINOCT2015_IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static ISO2_ALANGUAGE_CODE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z]{2,2}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static MAX15_PLUS_SIGNED_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[\+]{0,1}[0-9]{1,15}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static MAX3_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9]{1,3}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static MAX5_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9]{1,5}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static MIN2_MAX3_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9]{2,3}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static MIN3_MAX4_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9]{3,4}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static MIN8_MAX28_NUMERIC_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9]{8,28}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static SHA256_SIGNATURE_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([0-9A-F][0-9A-F]){32}$")
        .expect("Invalid regex pattern generated from XSD")
});
pub static HEX_BINARY_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-9a-fA-F]+$")
        .expect("Invalid regex pattern generated from XSD")
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iban_regex_accepts_valid_shapes() {
        assert!(IBAN2007_IDENTIFIER_REGEX.is_match("BE68539007547034"));
        assert!(IBAN2007_IDENTIFIER_REGEX.is_match("GB29NWBK60161331926819"));
        assert!(IBAN2007_IDENTIFIER_REGEX.is_match("DE12X")); // minimum: 2 letters + 2 check digits + 1 alnum
    }

    #[test]
    fn iban_regex_rejects_invalid_shapes() {
        assert!(!IBAN2007_IDENTIFIER_REGEX.is_match("")); // empty
        assert!(!IBAN2007_IDENTIFIER_REGEX.is_match("BE")); // no check digits, no BBAN
        assert!(!IBAN2007_IDENTIFIER_REGEX.is_match("68539007547034")); // no country code
        assert!(!IBAN2007_IDENTIFIER_REGEX.is_match("belgium68539")); // lowercase country code
        assert!(!IBAN2007_IDENTIFIER_REGEX.is_match(&format!("BE68{}", "1".repeat(31)))); // 31 BBAN chars, max is 30
        assert!(!IBAN2007_IDENTIFIER_REGEX.is_match("BE68 5390 0754 7034")); // spaces not allowed
    }

    #[test]
    fn bic_regex_accepts_8_and_11_char_forms() {
        assert!(ANY_BICDEC2014_IDENTIFIER_REGEX.is_match("DEUTDEFF")); // 8-char
        assert!(ANY_BICDEC2014_IDENTIFIER_REGEX.is_match("DEUTDEFF500")); // 11-char
        assert!(BICFIDEC2014_IDENTIFIER_REGEX.is_match("DEUTDEFF"));
        assert!(BICFIDEC2014_IDENTIFIER_REGEX.is_match("DEUTDEFF500"));
    }

    #[test]
    fn bic_regex_rejects_wrong_length_and_lowercase() {
        assert!(!ANY_BICDEC2014_IDENTIFIER_REGEX.is_match("DEUTDEF")); // 7 chars
        assert!(!ANY_BICDEC2014_IDENTIFIER_REGEX.is_match("DEUTDEFF50")); // 10 chars (must be 8 or 11)
        assert!(!ANY_BICDEC2014_IDENTIFIER_REGEX.is_match("deutdeff")); // lowercase
        assert!(!ANY_BICDEC2014_IDENTIFIER_REGEX.is_match(""));
    }

    #[test]
    fn country_code_regex_requires_exactly_two_uppercase_letters() {
        assert!(COUNTRY_CODE_REGEX.is_match("BE"));
        assert!(!COUNTRY_CODE_REGEX.is_match("B"));
        assert!(!COUNTRY_CODE_REGEX.is_match("BEL"));
        assert!(!COUNTRY_CODE_REGEX.is_match("be"));
    }

    #[test]
    fn currency_code_regex_requires_exactly_three_uppercase_letters() {
        assert!(ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX.is_match("EUR"));
        assert!(ACTIVE_CURRENCY_CODE_REGEX.is_match("USD"));
        assert!(!ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX.is_match("EU"));
        assert!(!ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX.is_match("EURO"));
        assert!(!ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX.is_match("eur"));
        assert!(!ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX.is_match("EU1"));
    }

    #[test]
    fn uuidv4_regex_accepts_valid_v4_uuid() {
        assert!(UUIDV4_IDENTIFIER_REGEX.is_match("f47ac10b-58cc-4372-a567-0e02b2c3d479"));
    }

    #[test]
    fn uuidv4_regex_rejects_wrong_version_and_variant_nibbles() {
        // Version nibble must be '4'.
        assert!(!UUIDV4_IDENTIFIER_REGEX.is_match("f47ac10b-58cc-1372-a567-0e02b2c3d479"));
        // Variant nibble must be 8/9/a/b.
        assert!(!UUIDV4_IDENTIFIER_REGEX.is_match("f47ac10b-58cc-4372-1567-0e02b2c3d479"));
        // Uppercase hex not accepted (regex only allows a-f0-9).
        assert!(!UUIDV4_IDENTIFIER_REGEX.is_match("F47AC10B-58CC-4372-A567-0E02B2C3D479"));
        assert!(!UUIDV4_IDENTIFIER_REGEX.is_match("not-a-uuid"));
    }

    #[test]
    fn lei_regex_requires_18_alnum_plus_2_check_digits() {
        assert!(LEIIDENTIFIER_REGEX.is_match("529900T8BM49AURSDO55")); // real-shaped LEI, 20 chars
        assert!(!LEIIDENTIFIER_REGEX.is_match("529900T8BM49AURSDO5")); // 19 chars
        assert!(!LEIIDENTIFIER_REGEX.is_match(""));
    }

    #[test]
    fn phone_number_regex_requires_leading_plus_and_country_code() {
        assert!(PHONE_NUMBER_REGEX.is_match("+32-478123456"));
        assert!(!PHONE_NUMBER_REGEX.is_match("32-478123456")); // missing '+'
        assert!(!PHONE_NUMBER_REGEX.is_match("+32478123456")); // missing '-' separator
    }

    #[test]
    fn sha256_signature_regex_requires_64_uppercase_hex_chars() {
        let valid = "AB".repeat(32);
        assert_eq!(valid.len(), 64);
        assert!(SHA256_SIGNATURE_TEXT_REGEX.is_match(&valid));
        assert!(!SHA256_SIGNATURE_TEXT_REGEX.is_match(&valid.to_lowercase()));
        assert!(!SHA256_SIGNATURE_TEXT_REGEX.is_match(&valid[..62])); // too short
    }

    #[test]
    fn hex_binary_regex_accepts_mixed_case_and_rejects_non_hex() {
        assert!(HEX_BINARY_TEXT_REGEX.is_match("deadBEEF01"));
        assert!(!HEX_BINARY_TEXT_REGEX.is_match("not-hex!"));
        assert!(!HEX_BINARY_TEXT_REGEX.is_match("")); // requires at least one char
    }

    #[test]
    fn numeric_text_regexes_enforce_exact_and_bounded_lengths() {
        assert!(EXACT1_NUMERIC_TEXT_REGEX.is_match("5"));
        assert!(!EXACT1_NUMERIC_TEXT_REGEX.is_match("55"));
        assert!(EXACT2_NUMERIC_TEXT_REGEX.is_match("42"));
        assert!(!EXACT2_NUMERIC_TEXT_REGEX.is_match("4"));
        assert!(MAX3_NUMERIC_TEXT_REGEX.is_match("1"));
        assert!(MAX3_NUMERIC_TEXT_REGEX.is_match("123"));
        assert!(!MAX3_NUMERIC_TEXT_REGEX.is_match("1234"));
        assert!(MIN2_MAX3_NUMERIC_TEXT_REGEX.is_match("12"));
        assert!(!MIN2_MAX3_NUMERIC_TEXT_REGEX.is_match("1"));
        assert!(!MIN2_MAX3_NUMERIC_TEXT_REGEX.is_match("1234"));
    }
}
