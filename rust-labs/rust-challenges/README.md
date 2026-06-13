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

**Why it solves the problem**

Your t is every char of s, plus one extra. If you XOR everything together (all of s and all of t),
every char that appears in both lists shows up an even number of times and cancels itself out via
property 1. The one extra char appears an odd number of times — it's the lone survivor. Property 2
mops up the zeros.

Concretely, with s = "abc", t = "cabd":

```text
a ^ b ^ c   ^   c ^ a ^ b ^ d
    = (a^a) ^ (b^b) ^ (c^c) ^ d
    = 0 ^ 0 ^ 0 ^ d
    = d
```

**XOR in Rust**

The operator is ^, and there's a compound-assign form ^=:

```rust
let x = 5 ^ 3;        // 6
let mut acc = 0u8;
acc ^= b'a';          // acc = acc ^ b'a'
```

**A few Rust-specific things that matter for this problem:**

1. Chars vs bytes. Rust's char is a 4-byte Unicode scalar and does not implement ^ directly. You XOR
integers. Since these problems are ASCII (lowercase English letters), iterate over .bytes() to get
u8 values, XOR those, then convert the result back:

2. fold is the idiomatic accumulator. Coming from Java you might reach for a for loop with a mut
variable. That's fine and clear. But the Rust-idiomatic way to "reduce a sequence to one value" is
fold:

```rust
// chain both strings into one stream of bytes, XOR them all
let result = s.bytes().chain(t.bytes()).fold(0u8, |acc, byte| acc ^ byte);
```

2.1. s.bytes() — iterate over the string as raw bytes

s.bytes() gives you an iterator of u8 values, one per byte of the string. Since LeetCode guarantees the input is lowercase ASCII letters, each byte is one character ('a' = 97, 'b' = 98, etc.). This is cheaper than chars(), which yields 4-byte char values and has to do UTF-8 decoding.

For s = "abc", s.bytes() yields: 97, 98, 99.

2.2. .chain(t.bytes()) — glue the two iterators together

chain produces a single iterator that yields everything from the first iterator, then everything from the second. No allocation, no copying — it just switches sources when the first runs dry.

For s = "abc", t = "abcd":

97, 98, 99, 97, 98, 99, 100
└── s ────┘ └────── t ─────┘

This works because for this problem we don't care where each character came from — only how many times each value appears in total.

3.2. .fold(0u8, |acc, byte| acc ^ byte) — reduce everything to one value

fold walks the iterator, carrying an accumulator:

- 0u8 is the starting value (typed as u8 so the compiler knows the accumulator type).
- For each byte, it computes acc ^ byte and that becomes the new acc.
- When the iterator is exhausted, fold returns the final acc.

So it's literally: ((((((0 ^ 97) ^ 98) ^ 99) ^ 97) ^ 98) ^ 99) ^ 100.

3. Watch the integer type. 0u8 pins the accumulator to u8 so it matches the bytes. XOR never
overflows (the result of u8 ^ u8 always fits in u8), so unlike the sum trick you don't have to worry
about wraparound here. That's a quiet advantage of XOR over the sum approach.

**Where else you'll see this**

- "Single Number" (LeetCode 136): every element appears twice except one — same exact survivor
  logic.
- Swap without a temp: a ^= b; b ^= a; a ^= b; (cute, rarely worth it in real code).
- Finding a missing number in 0..n.
- Toggling a flag/bit: flags ^= MASK flips just the masked bits.

## `windows` in Rust for LC28 Find Index of the First Occurrence in a String

`windows(k)` slides a fixed-width window of size k across a slice, yielding every contiguous sub-slice of that window, **overlapping**,
advancing one element at a time.

```rust
let v = [1, 2, 3,  4];
// v.windows(2) yields: [1,2], [2,3], [3,4]
// v.windows(3) yields: [1,2,3], [2,3,4]
```

For a slice of length n and window k, you get exactly `n - k + 1` windows (or zero
if `k > n`). Each yielded item is a `&[T]` - a borrow into the original slice, not
a copy.

The signature:

```rust
pub fn windows(&self, size: usize) -> Windows<'_, T>
```

**How it's actually built**

```rust
pub struct Windows<'a, T: 'a> {
    v: &'a [T],          // the remaining slice still to scan
    size: NonZeroUsize,  // window width
}

impl<'a, T> Iterator for Windows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<&'a [T]> {
        if self.size.get() > self.v.len() {
            None
        } else {
            let ret = &self.v[..self.size.get()]; // front window
            self.v = &self.v[1..];                // slide right by ONE
            Some(ret)
        }
    }
}
```

1. `&self.v[..size]` — take a view of the first size elements. This is the current window.
2. `self.v = &self.v[1..]` — drop the first element from the working slice, so 
next time the window starts one position later.

That single `[1..]` re-slice is why windows overlap. Compare with its sibling 
chunks, whose next does `self.v = &self.v[size..]` — advancing by the full width, 
giving non-overlapping blocks. The only difference between "overlapping windows" 
and "disjoint chunks" is whether you step by 1 or by `size`. That's a nice thing 
to internalize.

**Why it's efficient**

*No allocation, ever.* This is the big one coming from Java. There, `haystack.substring(i, i + k)` allocates a new `String` and copies the chars — do that across the haystack and you've allocated O(n) strings totaling O(n·k) bytes. Rust's `&self.v[..size]` allocates nothing. A `&[T]` is a *fat pointer*: a pair of `(pointer, length)` that lives in a register or on the stack. Slicing just computes a new pointer and length — pointer arithmetic, no copy, no heap touch. Every window is two machine words.

*O(1) per `next`.* Each step is a length comparison, a sub-slice, and a pointer bump. No work proportional to k happens just to *produce* a window. (The cost of *comparing* a window to the needle is separate — that's the `==` you write, a `memcmp` of up to k bytes.)

*Lazy and short-circuiting.* Because it's an iterator, windows are produced on demand. When you chain `position(|w| w == needle)`, the moment a window matches, iteration stops — the remaining windows are never generated. You get early-exit for free, without writing a `break`.

*Bounds-check elision.* The naive worry is "doesn't `&self.v[..size]` do a bounds check every iteration?" It does in the source, but the `if self.size.get() > self.v.len()` guard above it *proves* to LLVM that the index is in range, so the optimizer deletes the redundant check. The generated assembly is essentially what you'd hand-write with raw pointers. This guard-then-slice pattern is idiomatic precisely because it makes bounds checks provably dead.

*The `NonZeroUsize` trick.* Notice `size` isn't a plain `usize`. Storing it as `NonZeroUsize` does two things: it bakes the "size must be > 0" invariant into the type (the panic happens once, up front, in `windows()` itself), and it lets the compiler use the niche — a `usize` that can never be 0 has a spare bit-pattern, so `Option<NonZeroUsize>` is the same size as `usize`.

**Lifetimes — the detail that trips up Java folks**

Look at the item type: `type Item = &'a [T]`. The `'a` is the lifetime of the *original slice*, not of the iterator. The windows borrow from the source data, so:

- The original slice must stay alive and **unmodified** for as long as you hold the iterator (the borrow checker enforces this — you can't mutate the haystack while iterating windows over it).
- A yielded `&[T]` can outlive a given `next()` call; it's tied to the source, not the iterator step.

This is why there's no allocation to clean up and no use-after-advance hazard — the type system guarantees the backing data is pinned in place.

**Bonus traits it implements**

`Windows` is more than a forward iterator:

- `ExactSizeIterator` → `.len()` is O(1) (`n - k + 1`), because it can compute the count without walking.
- `DoubleEndedIterator` → you can `.rev()` or `.next_back()` to scan from the right. (For "first occurrence" you want left-to-right, but it's there.)

**Two gotchas to design around**

1. `windows(0)` panics. `NonZeroUsize::new(0)` fails and `windows()` does `.expect("window size must be non-zero")`. So before you ever call `windows(needle.len())`, handle the empty-needle case — which, conveniently, is also the case with the special return value. One guard kills two birds.

2. What you compare. `windows` over `&[u8]` yields `&[u8]`. To test a window against the needle you compare `&[u8] == &[u8]`, which Rust does element-wise (a length check then `memcmp`). Mixing a `&[u8]` window with a `&str` won't compile — the compiler keeping you honest.

3. The output type. `position` hands you `Option<usize>` (the window's *start index*, which is exactly the byte index of the match). The function wants `i32` with `-1` for "not found". Collapse "Some(i) → transform, None → default" with one combinator — see below.

## Collapsing `Option` with `map_or`

`map_or`.

```rust
let result = opt.map_or(default, |i| transform(i));
```

Its signature is `map_or<U>(self, default: U, f: F) -> U where F: FnOnce(T) -> U`. The argument order is the part worth burning into memory: **default comes first**, the closure second. So you read it as "map this, or use that" — slightly backwards from how the match reads top-to-bottom.

```rust
match opt {
    Some(i) => transform(i),
    None    => default,
}
```

One gotcha tied to the ownership concepts you've been working through: `default` is evaluated eagerly, regardless of whether the `Option` is `Some` or `None`. If that default is expensive to produce, or has side effects, or moves a value you'd rather not give up when it isn't needed, reach for `map_or_else` instead, which takes a closure for the default too:

```rust
let result = opt.map_or_else(|| compute_default(), |i| transform(i));
```

So the rule of thumb: `map_or` for a cheap, already-computed default; `map_or_else` when the default itself needs lazy evaluation.



