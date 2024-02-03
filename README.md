# advent-of-code-2023 <!-- omit in toc -->

My solutions to (some of) the
[2023 Advent of Code problems](https://adventofcode.com/2023/),
as solved on [my Twitch stream](https://twitch.tv/NicMcPhee).

This was started on Saturday, 3 February 2024.

- [Day 01](#day-01)
  - [Part 1](#part-1)

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
