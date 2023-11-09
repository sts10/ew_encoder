# Emoji-Word Encoder

**Proof of concept**

Encodes 128 bits (16 `u8`s) into 6 pairs of emoji and words.

Running `cargo run` prints a little demo:
```text
If we have a 128-bit key of 
[64, 65, 86, 20, 87, 170, 254, 198, 217, 225, 243, 255, 198, 106, 21, 11]
We first convert it into base-2655468:
[111577, 1810868, 2455782, 2472407, 1900768, 2506320]
Finally, we convert this into 6 emoji-word pairs:
🚂 anthropologist 🦢 presenting 🍀 transaction 📝 trend 🐼 radiation 🧵 understandings
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
