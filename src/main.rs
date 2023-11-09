use ew_encoder::encode;
use ew_encoder::key_to_emoji_and_words;

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
