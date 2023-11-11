# Emoji-Word Encoder

A **proof of concept** inefficient multi-list encoder. Do NOT actually use this code to encode anything important.

Encodes 128 bits (16 `u8`s) into 6 pairs of emoji and words.

Running `cargo run --bin ew_encoder` prints a little demo:
```text
If we have a 128-bit key of 
[64, 65, 86, 20, 87, 170, 254, 198, 217, 225, 243, 255, 198, 106, 21, 11]
We first convert it into base-2642247:
[114397, 119199, 503890, 847975, 2387380, 1468640]
Finally, we convert this into 6 emoji-word pairs:
👍 anxieties 🍑 appealed 🍦 conscience 🍦 environments 🐼 territorial 🔥 medicines
```

## Why use lists of exactly 203 emoji and 13016 words?

The entropy of a pair of emoji + word is `log2(203) + log2(13016) == 21.33333`. 

We like that number because multiplying it by 6 (`(log2(203) + log2(13016)) * 6`) gives us a number _just_ above `128` (`128.0000067171`). 

This means that 6 pairs of emoji+word can encode 128 bits with only _a little bit_ of leftover (inefficiency) (hence the phrase **"inefficient multi-list encoder"**).

I contend that this inefficiency is worth it in some use-cases, since finding 203 visually distinct emoji and 13,016 words is feasible.

As you might be able to tell, I didn't pick 203 and 13016 randomly and get lucky. Read and/or run `find_good_list_length_pairs.rs` file to see how I found these two values.

How to run it using Cargo:
```shell
cargo run --bin find_good_list_length_pairs
```

### Results

Now, let's say we wanted to be strict about being efficient (no wasted pairs). Here are some of our options for the two list lengths:

```text
Lists of 128 words and 512 emoji would require 8 pairs to represent 128 bits
Lists of 256 words and 256 emoji would require 8 pairs to represent 128 bits
Lists of 512 words and 128 emoji would require 8 pairs to represent 128 bits
Lists of 1024 words and 64 emoji would require 8 pairs to represent 128 bits
Lists of 2048 words and 32 emoji would require 8 pairs to represent 128 bits
Lists of 4096 words and 16 emoji would require 8 pairs to represent 128 bits
```

8 pairs is a lot! Plus, as far as word lists go, 4,096 isn't that much -- we could go up to 8,000 or even 14,000 and not have to use very strange words. Not very inspiring. But that's just the way the math works out... right? 

But what if we accept length pairs that _over-shot_ our 128-bit goal just a bit. Yes, some possible codes won't be used, but let's just see what these lists would look like (`cargo run --bin find_good_list_length_pairs`). 

```text
Lists of 6775 words and 390 emoji would require 5.999999378003068 pairs to represent 128 bits
Lists of 7046 words and 375 emoji would require 5.999999378003069 pairs to represent 128 bits
Lists of 8130 words and 325 emoji would require 5.999999378003068 pairs to represent 128 bits
Lists of 9750 words and 271 emoji would require 5.999999378003068 pairs to represent 128 bits
Lists of 10569 words and 250 emoji would require 5.999999378003069 pairs to represent 128 bits
Lists of 11389 words and 232 emoji would require 5.999999685133787 pairs to represent 128 bits
Lists of 13016 words and 203 emoji would require 5.999999685133787 pairs to represent 128 bits
Lists of 13550 words and 195 emoji would require 5.999999378003068 pairs to represent 128 bits
Lists of 17615 words and 150 emoji would require 5.999999378003069 pairs to represent 128 bits
Lists of 19009 words and 139 emoji would require 5.999999224437808 pairs to represent 128 bits
```

Now we get (a) option for 6 pairs rather than 8 and (b) the word list length and emoji list lengths are much more reasonable/feasible.

I believe this process is the most interesting part of this project.

For the rest of this project, I picked the 13,016 words + 203 emoji pair.

## A crude measure of success
The fact that when we use this process to encode the highest possible 128-bit value, the first word used is the last word listed on the word list. In other words, we couldn't use fewer words than 13,016 to encode all possible 128-bit values.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
