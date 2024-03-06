use solana_program::pubkey::Pubkey;

/// Returns an array that represents a map from Base58 encoding character to number.
/// For example:
/// ```no_run
/// use const_str_to_pubkey::get_base58ch_to_number_map;
///
/// let map = get_base58ch_to_number_map();
/// assert!(map['1' as usize] == 0);
/// assert!(map['2' as usize] == 1);
/// assert!(map['A' as usize] == 9);
/// assert!(map['B' as usize] == 10);
/// assert!(map['a' as usize] == 33);
/// // Invalid characters (like uppercase 'O') are mapped to 0xFF
/// assert!(map['O' as usize] == 0xFF);
/// ```
pub const fn get_base58ch_to_number_map() -> [u8; 128] {
    let mut map = [0xFF; 128];
    let mut number = 0;

    let mut i = '1' as usize;
    while i <= '9' as usize {
        map[i] = number;
        number += 1;
        i += 1;
    }

    i = 'A' as usize;
    while i <= 'Z' as usize {
        if i != 'I' as usize && i != 'O' as usize {
            map[i] = number;
            number += 1;
        }
        i += 1;
    }

    i = 'a' as usize;
    while i <= 'z' as usize {
        if i != 'l' as usize {
            map[i] = number;
            number += 1;
        }
        i += 1;
    }

    map
}

/// Converts a `&'static str` to [`Pubkey`](https://docs.rs/solana-program/latest/solana_program/pubkey/struct.Pubkey.html).
///
/// This is sometimes useful, because the macro [`pubkey!`](https://docs.rs/solana-program/latest/solana_program/macro.pubkey.html)
/// only works with string literals. When we have a constant public key string
/// (e.g., from [`env!`](https://doc.rust-lang.org/core/macro.env.html)) instead of a string literal, we can derive a
/// constant `Pubkey` with this function. For example:
///
/// ```ignore
/// use const_str_to_pubkey::str_to_pubkey;
/// const ADMIN_PUBKEY: Pubkey = str_to_pubkey(env!("ADMIN_PUBKEY"));
/// ```
pub const fn str_to_pubkey(s: &'static str) -> Pubkey {
    let s = s.as_bytes();
    assert!(
        s.len() <= 44,
        "Public key string length should be no more than 44"
    );

    let map = get_base58ch_to_number_map();
    let mut bytes = [0u8; 32];
    let mut i = 0;
    let mut index = 0;

    while i < s.len() {
        assert!(s[i] <= 127, "Invalid Base58 character found");

        let mut val = map[s[i] as usize] as usize;
        assert!(val != 0xFF, "Invalid Base58 character found");

        let mut j = 0;
        while j < index {
            val += (bytes[j] as usize) * 58;
            bytes[j] = (val & 0xFF) as u8;
            val >>= 8;
            j += 1;
        }

        while val > 0 {
            bytes[index] = (val & 0xFF) as u8;
            index += 1;
            val >>= 8;
        }

        i += 1;
    }

    i = 0;
    while i < s.len() && s[i] == '1' as u8 {
        bytes[index] = 0;
        index += 1;
    }

    i = 0;
    while i < 16 {
        (bytes[i], bytes[31 - i]) = (bytes[31 - i], bytes[i]);
        i += 1;
    }

    Pubkey::new_from_array(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const MAP: [u8; 128] = get_base58ch_to_number_map();
    const PUBKEY_STR: &'static str = "CBNbUAykYgopeby9QC9x9pvpvoRrbmf5FrPLFZ8rGB4Y";
    const PUBKEY: Pubkey = str_to_pubkey(PUBKEY_STR);

    #[test]
    fn test_base58ch_to_number_map() {
        assert_eq!(MAP['1' as usize], 0);
        assert_eq!(MAP['2' as usize], 1);
        assert_eq!(MAP['A' as usize], 9);
        assert_eq!(MAP['a' as usize], 33);

        // Invalid characters are mapped to 0xFF
        assert_eq!(MAP['0' as usize], 0xFF);
        assert_eq!(MAP['I' as usize], 0xFF);
        assert_eq!(MAP['O' as usize], 0xFF);
        assert_eq!(MAP['l' as usize], 0xFF);
        assert_eq!(MAP['+' as usize], 0xFF);
    }

    #[test]
    fn test_str_to_pubkey() {
        let gt_pubkey = Pubkey::from_str(PUBKEY_STR).unwrap();
        assert_eq!(PUBKEY, gt_pubkey);
    }
}
