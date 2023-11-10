use include_lines::include_lines;

/// Encode an array of 16 `u8`s into a String of 6 emoji-and-word pairs
pub fn key_to_emoji_and_words(key: [u8; 16]) -> String {
    let mut visual_fingerprint: String = String::new();

    let emoji_list = &include_lines!("emoji_list.txt");
    let word_list = &include_lines!("word_list.txt");
    // It's important that these lists have these exact lengths
    // assert!(emoji_list.len() == 203);
    // assert!(word_list.len() == 13016);

    let encoded_key = to_custom_base(key);
    for d in encoded_key {
        let (x, y) = get_x_and_y_from_part_of_key(d);
        visual_fingerprint = visual_fingerprint + " " + emoji_list[x] + " " + word_list[y];
    }
    visual_fingerprint.trim().to_string()
}

/// Takes a base-2655468 "digit" of the 128-bit key and converts it into x,y  coordinates
/// Where the x can range from 0 to 202 and the y can range from 0 to 13,015 (May be off by one?)
/// I picked this idea up from a paper on the Drunken Bishop
/// http://dirk-loss.de/sshvis/drunken_bishop.pdf
pub fn get_x_and_y_from_part_of_key(part_of_key: usize) -> (usize, usize) {
    let x = part_of_key % 204;
    let y = (part_of_key - x) / 204;
    (x, y)
}

/// Encodes a 128 u8 array into 6 "digits" of base 2655468.
/// 203 + 204 * 13016 == 2_655_457
pub fn to_custom_base(value: [u8; 16]) -> Vec<usize> {
    let mut value = u128::from_ne_bytes(value);
    let base = 2655468;
    let mut digits_vec: Vec<usize> = vec![];
    while value > 0 {
        let digit = value % base; // digit in [0..base)
        digits_vec.push(digit.try_into().unwrap());
        value /= base; // integer div
    }
    digits_vec.reverse();
    while digits_vec.len() < 6 {
        digits_vec.push(0);
    }
    digits_vec
}

#[test]
fn can_convert_between_x_y_pair_and_key() {
    // a random x y pair
    let xy = (199, 9444);
    fn get_key_from_x_y_pair(x: usize, y: usize) -> usize {
        x + 204 * y
    }

    assert_eq!(
        get_x_and_y_from_part_of_key(get_key_from_x_y_pair(xy.0, xy.1)),
        xy
    );
}

use std::collections::HashSet;
#[test]
fn can_convert_from_x_y_pair_to_partial_key_for_all_possible_values() {
    let mut all_possible_pairs: HashSet<(usize, usize)> = HashSet::new();
    for partial_key in 0..2655468 {
        let (this_x, this_y) = get_x_and_y_from_part_of_key(partial_key);

        assert!(!all_possible_pairs.contains(&(this_x, this_y)));
        all_possible_pairs.insert((this_x, this_y));
    }
    assert_eq!(all_possible_pairs.len(), 2655468);
}

#[test]
fn can_encode_a_sample_key() {
    let sample_key: [u8; 16] = [
        64, 65, 86, 20, 87, 170, 254, 198, 217, 225, 243, 255, 198, 106, 21, 11,
    ];
    let visual_fingerprint = key_to_emoji_and_words(sample_key);

    assert_eq!(
        visual_fingerprint,
        "🚂 anthropologist 🦢 presenting 🍀 transaction 📝 trend 🐼 radiation 🧵 understandings"
    );
}

#[test]
fn can_encode_max_u128() {
    let max_key_possible = u128::MAX.to_ne_bytes();
    let visual_fingerprint = key_to_emoji_and_words(max_key_possible);
    assert_eq!(
        visual_fingerprint,
        "🪵 volcanic 💾 proceeds 🛁 defect 🦢 collaborating 🚂 seventies 🍼 currency"
    );
}

#[test]
fn can_encode_zero() {
    let visual_fingerprint = key_to_emoji_and_words(0u128.to_ne_bytes());
    assert_eq!(
        visual_fingerprint,
        "⌛ abandon ⌛ abandon ⌛ abandon ⌛ abandon ⌛ abandon ⌛ abandon"
    );
}
