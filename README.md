# advent-of-code-2023 <!-- omit in toc -->

My solutions to (some of) the
[2023 Advent of Code problems](https://adventofcode.com/2023/),
as solved on [my Twitch stream](https://twitch.tv/NicMcPhee).

This was started on Saturday, 3 February 2024.

- [Day 01](#day-01)
  - [Part 1](#part-1)
  - [Part 2](#part-2)
- [Day 02](#day-02)
- [Day 03](#day-03)

---

## Day 01

Interestingly, when I was setting things up, GitHub CoPilot went
straight to `include_str!()` instead of the "standard" file reading
stuff. So I just rolled with that, since people had suggested it
as a way of speeding things up.

### Part 1

The first part was pretty straightforward. I initially overcomplicated
things by using `filter` with a bunch of unwrapping instead of
`filter_map`, but folks got me on the right track. There was also
some question about how to best get the last digit, but it turns
out that `DoubleEndedIterator`s have a `.next_back()` method that
does that nicely.

### Part 2

The second part is, in some ways, a substantial "jump" from the first
part, although I was surprised to find that my structure for Part 1
was clean enough that I really just had extract out and modify a
 `get_digits()` method.

I spent a while flailing on how to get the consecutive "windows" onto
a given `line` as it turned out that there wasn't a particularly good
"built-in" for that. In the end I went with a suggestion from
@JustusFluegel to use `char_indices` to get all the consecutive slices.
Then we just used `filter_map()` again with a new `to_digits()` method.

Using `match` on `to_digits()` was quite nice, and cleaner than I had
anticipated. I used `s.starts_with()` as suggested by @JustusFluegel, but @MizardX proposed matching against byte arrays, like:

```rust
    match s.as_bytes() {
        &[b'o',b'n',b'e',..]=> ...
    }
```

That might be faster because we don't have to do the UTF-8
checking that working with `String`s requires, although we didn't
do a test to find out. I stuck with `starts_with()` just because I
found it a lot more readable than the `[b'o', b'n', ...]`
business.

@MizardX built a home-brew state machine in his solution,
so they never had to do any full string matches. [Their solution](https://github.com/MizardX/AdventOfCode_2023/blob/main/src/day01/mod.rs#L159) has the form:

```rust
fn match_forward(line: &[u8]) -> Option<u8> {
    let mut state = State::Start;
    for ch in line {
        state = match (state, ch) {
            (_, b @ b'0'..=b'9') => return Some(b - b'0'),
            (State::O | State::Fo, b'n') => State::On,
            (State::On, b'e') => return Some(1),
            (State::T, b'w') => State::Tw,
            (State::T, b'h') => State::Th,
            (State::Tw, b'o') => return Some(2),
            (State::Th, b'r') => State::Thr,
            ...
```

I found it interesting that they restart their state machine after
each digit is recognized. I wonder if it would be more efficient
to stay in the same state machine throughout, tracking the first
and last digits as you go. Not sure I'll ever actually _try_ that,
but it would be an interesting experiment.

## Day 02

At @JustusFluegel's suggestion, we're going to try using the
[Pest parser](https://pest.rs/). I was flailing some at the start, but
after we got the grammar written it was pretty nice:

```pest
game = { "Game" ~ #game_number = int ~ ":" ~ reveal ~ (";" ~ reveal)* }
    int = { ("+" | "-")? ~ ASCII_DIGIT+ }
    reveal = { cubeCount ~ ("," ~ cubeCount)* }
    cubeCount = { int ~ color }
    color = { "red" | "green" | "blue" }

WHITESPACE = _{ " " }
```

I feel like these grammars might be a lot easier to re-use than
the parsers we built with `nom` last time? That said, there are
some downsides:

- I'm not a huge fan of the `reveal ~ (";" ~ reveal)*` syntax. It's
  a bummer that you have to repeat `reveal`, and it's nice in `nom` that
  they have separated lists combinators that avoid that.
- In Pest you get this big syntax tree back, and we'll have to take
  it apart, mapping and combining the various pieces. With `nom` you
  are able to map elements to their desired values as part of the
  parsing process, so what comes out of the parsing process is the
  value you wanted.

I haven't yet processed the value returned by the parser, though,
so it might be less of an issue than I'm thinking.

## Day 03

I decided that I liked Pest, so I chose to use it again for Day 03.

We started with this Pest grammar:

```pest
input = _{ row ~ ("\n" ~ row)* }

row = { cell* }

cell = { number | symbol }

number = @{ ASCII_DIGIT+ }

symbol = { !(WHITESPACE | number) ~ ANY }

WHITESPACE = _{ "." }
```

but ended up changing to:

```pest
input = { WHITESPACE* ~ cell+ }

cell = { number | symbol }

number = @{ ASCII_DIGIT+ }

symbol = { !(WHITESPACE | ASCII_DIGIT) ~ ANY }

WHITESPACE = _{ "." | NEWLINE}
```

We had some trailing newlines (thanks to @MizardX for noticing that), so we added them to `WHITESPACE`. That then allowed us to just remove the `row` item from the grammar. When we went from the test input to the entire input, we found that it starts with a bunch of dots (whitespace), so the parser failed to find anything. since `cell*` and `cell+` only have whitespace _between_ elements. So we had to add `WHITESPACE*` to the front of the `input`.

Pest's span fields were great here, and made it easy to keep track of where things were in the input. We were able to fairly easily parse a part to a `Part` `struct` with the value of the part number, the line it was one, and the start and end positions of the digit sequence for that part. Similarly, a `Symbol` could have the character, and its line and column.

We came up (with lots of help from @JustusFluegel and @MizardX ) with a nice iterator over the adjacent locations of a `Part`, using `chain` in nice ways. We could then check each of those to see if it had a `Symbol` at that location. There was some discussion about whether we should loop over `Parts` and look up `Symbols`, or loop over `Symbols` and just do math related to the `Parts`. We went with the former (partly because hashing made looking up symbols by position easy and fast), but now that we've seen Part 2 of Day 03, I'm thinking we should have done the latter since now we'll need to loop over `Symbols` for this part. Oh, well.

I had thought we'd have to worry about subtractions going negative when we computed
the positions around a part, but since Pest starts its line and column counting at 1
instead of 0, it's safe to subtract one from those even though they are unsigned values.
When I still thought we'd have to worry about that problem, @MizardX suggested
`saturated_sub()` (which I've never used), which totally would have been perfect
if we had in fact needed it.
