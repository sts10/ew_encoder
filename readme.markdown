# Inefficient Multi-List Encoder

A **proof of concept** emoji-word encoder.

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

The entropy of a pair of emoji + word is `log2(203) + log2(13016) == 21.33333`. The key is that `(log2(203) + log2(13016)) * 6` is just above `128` (`128.0000067171`). This means that 6 pairs of emoji+word can encode 128 bits with a little bit of leftover (inefficiency). I contend that this inefficiency is worth it in some use-cases, since finding 203 visually distinct emoji and 13,016 words is feasible.

As you might be able to tell, I didn't pick 203 and 13016 randomly. Read and/or run `find_good_list_length_pairs.rs` file to see how I found these two values.

How to run it using Cargo:
```shell
cargo run --bin find_good_list_length_pairs
```

I believe this process is the most interesting part of this project.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
