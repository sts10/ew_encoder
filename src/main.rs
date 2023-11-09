use include_lines::include_lines;

fn main() {
    let sample_key: [u8; 16] = [
        64, 65, 86, 20, 87, 170, 254, 198, 217, 225, 243, 255, 198, 106, 21, 11,
    ];
    println!(
        "Encoded sample key {:?} as {:?}",
        sample_key,
        encode(sample_key)
    );

    let visual_fingerprint = key_to_emoji_and_words(sample_key);
    println!("Visual fingerprint:\n{}", visual_fingerprint);
}

fn key_to_emoji_and_words(key: [u8; 16]) -> String {
    let mut visual_fingerprint: String = String::new();

    let emoji_list = &include_lines!("emoji_list.txt");
    let word_list = &include_lines!("word_list.txt");

    let encoded_key = encode(key);
    for d in encoded_key {
        let (x, y) = get_x_and_y_from_part_of_key(d);
        visual_fingerprint = visual_fingerprint + " " + emoji_list[x] + " " + word_list[y];
    }
    visual_fingerprint.trim().to_string()
}

fn get_x_and_y_from_part_of_key(part_of_key: usize) -> (usize, usize) {
    let x = part_of_key % 204;
    let y = (part_of_key - x) / 204;
    (x, y)
}

fn encode(value: [u8; 16]) -> Vec<usize> {
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
