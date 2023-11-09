# Emoji-Word Encoder

Encodes 128 bits (16 `u8`s) into 6 pairs of emoji and words.

```text
If we have a 128-bit key of 
[64, 65, 86, 20, 87, 170, 254, 198, 217, 225, 243, 255, 198, 106, 21, 11]
We first convert it into base-2655468:
[111577, 1810868, 2455782, 2472407, 1900768, 2506320]
Finally, we convert this into 6 emoji-word pairs:
🚂 anthropologist 🦢 presenting 🍀 transaction 📝 trend 🐼 radiation 🧵 understandings
```
