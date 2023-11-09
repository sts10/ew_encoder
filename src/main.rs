use include_lines::include_lines;

fn main() {
    let key: u128 = 14732711889470983911895342650328760640; // = rand::random::<u128>();
    println!("Encoded {} as {:?}", key, encode(key));

    let visual_fingerprint = key_to_emoji_and_words(key);
    println!("Visual fingerprint:\n{}", visual_fingerprint);
}

fn key_to_emoji_and_words(key: u128) -> String {
    let mut visual_fingerprint: String = String::new();

    let emoji_list = &include_lines!("emoji_list.txt");
    let word_list = &include_lines!("word_list.txt");

    let encoded_key = encode(key);
    for d in encoded_key {
        let (x, y) = get_x_and_y_from_key(d);
        println!("x is {}; y is {}", x, y);
        visual_fingerprint = visual_fingerprint + " " + emoji_list[x] + " " + word_list[y];
    }
    visual_fingerprint.trim().to_string()
}

fn get_x_and_y_from_key(key: usize) -> (usize, usize) {
    let x = key % 204;
    let y = (key - x) / 204;
    (x, y)
}

fn encode(value: u128) -> Vec<usize> {
    let base = 2655468;
    let mut value = value;
    let mut digits_vec: Vec<usize> = vec![];
    while value > 0 {
        let digit = value % base; // digit in [0..base)
        digits_vec.push(digit.try_into().unwrap());
        value /= base; // integer div
    }
    digits_vec.reverse();
    digits_vec
}

#[test]
fn can_convert_between_x_y_pair_and_key() {
    // a random x y pair
    let xy = (199, 9444);
    fn get_key_from_x_y_pair(x: usize, y: usize) -> usize {
        x + 204 * y
    }

    assert_eq!(get_x_and_y_from_key(get_key_from_x_y_pair(xy.0, xy.1)), xy);
}
