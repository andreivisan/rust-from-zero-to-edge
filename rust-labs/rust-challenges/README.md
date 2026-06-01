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
