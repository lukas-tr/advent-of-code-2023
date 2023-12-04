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

