# Advent of Code 2023 (Rust)

## Day 1

Spent way too long debugging why `input.len()..0` is not the reversed version of `0..input.len()` ._.

TODO: use https://github.com/mfussenegger/nvim-dap

## Day 2

No major problems, tried out `nom` for parsing - worked well

## Day 3

I feel like the `sum_parts` turned out way more complex than it needs to be, but at least it works

## Day 4

I tried out `pest` for parsing this time - the grammar is in a separate file and can be reasoned about much more intuitively, but the whole matching of rules and converting the parsed structures to my own types was more cumbersome compared to `nom`.

## Day 5

Didn't enjoy this day's challenge. Chose to do it the brute force way. 

## Day 6

I attempted to do the math properly this time, but ended up brute forcing it again.

## Day 7

There's probably a prettier way to do this than with patterns to determine the type of the deck.

```
[] | [1] | [2] | [3] | [4] | [5] => 7,
[1, 1] | [2, 1] | [3, 1] | [4, 1] => 6,
[3, 2] | [2, 2] => 5,
[3, 1, 1] | [2, 1, 1] | [1, 1, 1] => 4,
[2, 2, 1] => 3,
[2, 1, 1, 1] | [1, 1, 1, 1] => 2,
[1, 1, 1, 1, 1] => 1,
```

## Day 8

This task required just the right amount of maths to be fun.

## Day 9

Part 2 was boring today, you just had to reverse the input.

## Day 10

Only did part 1 today.

