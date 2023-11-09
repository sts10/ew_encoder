//! This bin file prints a list of list length that are suitable for encoding 128-bit strings or
//! keys.
//! Notably, it includes length pairs that are slightly inefficient for encoding 128 bits.
fn main() {
    assert_eq!(calc_pairs_needed(256, 256), 8.0);
    for word_length in 128..=66000 {
        for emoji_length in 16..=512 {
            let pairs = calc_pairs_needed(word_length, emoji_length);
            if is_just_less_than_natural(pairs) {
                println!(
                    "Lists of {} words and {} emoji would require {} pairs to represent 128 bits",
                    word_length, emoji_length, pairs
                );
                break;
            }
        }
    }
    println!("Done.");
}

fn calc_pairs_needed(words: usize, emoji: usize) -> f64 {
    128 as f64 / (log_base_2(words as f64) + log_base_2(emoji as f64))
}

fn log_base_2(n: f64) -> f64 {
    n.ln() / 2_f64.ln()
}

/// Here, we set the tolerance for encoding ineffiency
/// 0.000001 seems about right, though 0.00001 adds some more
/// interesting list-length-pair options
fn is_just_less_than_natural(n: f64) -> bool {
    n.ceil() - n < 0.000001
}
