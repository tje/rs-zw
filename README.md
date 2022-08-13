# ZW

Utility for encoding and decoding text using zero-width characters.

## How it works

Subject text is first converted to its binary representation (e.g. "foo" -> "011001100110111101101111"), then each digit is replaced with a zero-width character (specifically: `U+200B` and `U+200C`). Decoding is simply the inverse of the same flow of operations.

## Usage

As a module:

```rust
use zw;
// ...
let encoded = zw::encode("Hello");
let decoded = zw::decode(&encoded);
```

As a CLI tool:

```sh
zw [-e|--encode] [-d|--decode] [input]
```

If both `-e` and `-d` flags are omitted then conversion direction is guessed based on the first interpreted character.

Reads from `stdin` if an input string isn't provided in its arguments.

```sh
# Shell-ish
zw "Hello" > encoded.txt
cat encoded.txt | zw > decoded.txt

# MacOS clipboard encoding
pbpaste | zw | pbcopy
```
