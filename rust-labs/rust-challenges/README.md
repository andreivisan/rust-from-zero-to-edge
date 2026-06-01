## Reverse Strint

### Comments at code review

Hello Andrei,

Good to meet you here. Interesting solution! I have a couple of observations/suggestions, maybe you'll find them helpful.

First, for the potential reversal of letters (lines 8 to 14) you could use the (optimized) slice method slice.reverse:

```rust
if is_palindrome(&letters) { return word; }
letters.reverse();
letters.concat()
```
}
Then: in the worst case you are allocating three times on the heap:

word
letters
return String
Imho, those are the parts that drive the performance of reverse, the rest is rather cheap. So I'd suggest to check if you could get rid of some of them. As a first step I'd suggest to drop word and only build it once you really need it, like:

pub fn reverse(input: &str) -> String {
    if input.len() < 2 { return input.to_string(); }
    let mut letters = input.graphemes(true).collect::<Vec<&str>>();
    if is_palindrome(&letters) { return input.to_string(); }
    ...
(input.len() is exactly the same as word.len().) Then I'd like to point out that the iterator Graphemes has implemented the DoubleEndedIterator trait (see here). That means you can reverse the iterator with the .rev() adapter and therefore could just collect reversed into letters:

pub fn reverse(input: &str) -> String {
    if input.len() < 2 { return input.to_string(); }
    let letters = input.graphemes(true).rev().collect::<Vec<&str>>();
    letters.concat()
}
This actually doesn't cost more than iterating the other way around, you can compare the implementations here and here. The palindrome check would now be moot. But at that point you could also just directly collect into a String:

pub fn reverse(input: &str) -> String {
    if input.len() < 2 { return input.to_string(); }
    input.graphemes(true).rev().collect()
}
Or, I like that better, because .collect() isn't optimal w.r.t. avoiding reallocations:

pub fn reverse(input: &str) -> String {
    if input.len() < 2 { return input.to_string(); }
    input.graphemes(true).rev().fold(
        String::with_capacity(input.len()),
        |mut reversed, grapheme| {
            reversed.push_str(grapheme);
            reversed
        }
    )
}
This way there's only one allocation, no growing pains, because the String is set up with String::with_capacity with the exact capacity needed.

I hope that's helpful. Let me know if not or if you have follow-up questions!

Cheers, Timus
