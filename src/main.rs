use ew_encoder::get_x_and_y_from_part_of_key;
use ew_encoder::key_to_emoji_and_words;
use ew_encoder::to_custom_base;

fn main() {
    let sample_key: [u8; 16] = [
        64, 65, 86, 20, 87, 170, 254, 198, 217, 225, 243, 255, 198, 106, 21, 11,
    ];
    println!("If we have a 128-bit key of \n{:?}", sample_key);
    println!(
        "We first convert it into base-2655468:\n{:?}",
        to_custom_base(sample_key)
    );

    let visual_fingerprint = key_to_emoji_and_words(sample_key);
    println!(
        "Finally, we convert this into 6 emoji-word pairs:\n{}",
        visual_fingerprint
    );

    println!(
        "Pair for last key partial (2655467) is {:?}",
        get_x_and_y_from_part_of_key(2655467)
    );
    let max_key: [u8; 16] = [
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
        u8::MAX,
    ];
    println!("MAX key in base-2655468 is:\n{:?}", to_custom_base(max_key));
}
