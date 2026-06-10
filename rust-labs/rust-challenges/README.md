# Lessons learned during the labs

## Reverse String

### Lessons from the refactor

The first version worked, but it did more work than necessary: it built intermediate collections, checked for palindromes separately, and allocated more than once.

The refactor keeps the Unicode-aware behavior while making the flow smaller and cheaper:

- Use `graphemes(true)` instead of raw `chars()` so composed characters and emoji stay intact.
- Use `.rev()` directly because `Graphemes` implements `DoubleEndedIterator`.
- Avoid collecting into `Vec<&str>` when the final result is a `String`.
- Skip the palindrome check because reversing the iterator already covers all cases cleanly.
- Preallocate with `String::with_capacity(input.len())` to reserve exactly enough bytes for the result.
- Build the output with `fold` and `push_str`, resulting in one intentional allocation.

Why `fold` instead of collecting into a vector first?

```rust
let letters = input.graphemes(true).rev().collect::<Vec<&str>>();
letters.concat()
```

This works, but it creates an intermediate `Vec<&str>` and then creates the final `String`. That means extra allocation and an extra pass over the graphemes.

Using `fold` writes each reversed grapheme directly into the final `String`:

```rust
input.graphemes(true).rev().fold(
    String::with_capacity(input.len()),
    |mut reversed, grapheme| {
        reversed.push_str(grapheme);
        reversed
    },
)
```

This avoids the temporary vector, avoids repeated string growth, and keeps the function focused on the result it actually needs to return.

## XOR (LC389 - Find the difference)

XOR ("exclusive or") is a bitwise operation on two bits. The rule: output 1 if the bits differ, 0 if
they're the same.

```text
0 ^ 0 = 0
0 ^ 1 = 1
1 ^ 0 = 1
1 ^ 1 = 0
```

Another way to read it: "is the number of 1s odd?".

**The three properties that make it powerful**

1. Self-inverse: x ^ x == 0 — anything XOR'd with itself cancels to zero.
2. Identity: x ^ 0 == x — XOR with zero changes nothing.
3. Commutative + associative: a ^ b == b ^ a, and you can regroup freely: (a ^ b) ^ c == a ^ (b ^ 
c).

Property 3 is the subtle, important one: order doesn't matter. That's exactly why a shuffle of s
doesn't bother us — XOR-ing a pile of values gives the same result no matter what order they come
in.

```text
TO REFINE

Why it solves your problem

  Your t is every char of s, plus one extra. If you XOR everything together (all of s and all of t),
  every char that appears in both lists shows up an even number of times and cancels itself out via
  property 1. The one extra char appears an odd number of times — it's the lone survivor. Property 2
  mops up the zeros.

  Concretely, with s = "abc", t = "cabd":

  a ^ b ^ c   ^   c ^ a ^ b ^ d
  = (a^a) ^ (b^b) ^ (c^c) ^ d
  = 0 ^ 0 ^ 0 ^ d
  = d

  The shuffle is irrelevant — that's the commutativity paying off.

  XOR in Rust

  The operator is ^, and there's a compound-assign form ^=:

  let x = 5 ^ 3;        // 6
  let mut acc = 0u8;
  acc ^= b'a';          // acc = acc ^ b'a'

  A few Rust-specific things that matter for this problem:

  1. Chars vs bytes. Rust's char is a 4-byte Unicode scalar and does not implement ^ directly. You XOR
  integers. Since these problems are ASCII (lowercase English letters), iterate over .bytes() to get
  u8 values, XOR those, then convert the result back:

  let b: u8 = b'a';          // byte literal, type u8
  let c: char = b as char;   // u8 -> char is a safe cast

  (If you tried to XOR char values you'd get a compile error — a good one to hit on purpose once so
  you recognize it.)

  2. fold is the idiomatic accumulator. Coming from Java you might reach for a for loop with a mut
  variable. That's fine and clear. But the Rust-idiomatic way to "reduce a sequence to one value" is
  fold:

  // chain both strings into one stream of bytes, XOR them all
  let result = s.bytes().chain(t.bytes()).fold(0u8, |acc, byte| acc ^ byte);

  - chain glues the two byte-iterators into one — no allocation, no temporary string.
  - fold(0u8, ...) starts the accumulator at 0 (our identity element — property 2!) and folds each
  byte in with ^.
  - This is lazy and single-pass: O(n) time, O(1) space.

  3. Watch the integer type. 0u8 pins the accumulator to u8 so it matches the bytes. XOR never
  overflows (the result of u8 ^ u8 always fits in u8), so unlike the sum trick you don't have to worry
  about wraparound here. That's a quiet advantage of XOR over the sum approach.

  Where else you'll see this

  - "Single Number" (LeetCode 136): every element appears twice except one — same exact survivor
  logic.
  - Swap without a temp: a ^= b; b ^= a; a ^= b; (cute, rarely worth it in real code).
  - Finding a missing number in 0..n.
  - Toggling a flag/bit: flags ^= MASK flips just the masked bits.

  ---
  Now go write it. I'd suggest you try the fold version, but if it feels alien, write the plain for
  loop first — both are legitimate Rust, and I'd rather you understand it than copy an idiom. Paste
  what you come up with and I'll check it for idiomaticity and any type gotchas.

```
