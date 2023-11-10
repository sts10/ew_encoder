# Emoji-Word Encoder

**Proof of concept**

Encodes 128 bits (16 `u8`s) into 6 pairs of emoji and words.

Running `cargo run --bin ew_encoder` prints a little demo:
```text
If we have a 128-bit key of 
[64, 65, 86, 20, 87, 170, 254, 198, 217, 225, 243, 255, 198, 106, 21, 11]
We first convert it into base-2642247:
[114397, 691184, 2124657, 1576564, 1359289, 156987]
Finally, we convert this into 6 emoji-word pairs:
👍 anxieties 🧢 differences 🎧 servants 🎲 northernmost ⏰ least 🎷 asserting
MAX key in base-2642247 is:
[2642240, 1843709, 2421924, 2104602, 1961804, 2523288]
```

## Why use lists of exactly 203 emoji and 13016 words?

Read and/or run `find_good_list_length_pairs.rs` file for the best answer to this question.

How to run it using Cargo:
```shell
cargo run --bin find_good_list_length_pairs
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
