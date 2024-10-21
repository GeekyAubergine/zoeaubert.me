---
slug: advent-of-code-2023-day-09
date: 2023-12-09T11:00
title: Advent of Code 2023 - Day 09
description: A discussion of my solution to Advent of Code 2023 - Day 09. This post contains spoilers.
tags: ['Programming', 'AdventOfCode', 'Rust']
---
[Advent of Code](https://adventofcode.com/) is a yearly programming challenge. See my [day 01 post](https://zoeaubert.me/blog/advent-of-code-2023-day-01/) to see how the project is set up.

To view my [solutions](https://github.com/GeekyAubergine/advent-of-code/tree/main/2023/day-09) in full, check them out on GitHub. See my [previous posts](https://zoeaubert.me/tags/advent-of-code/) for other solutions.

## Initial solutions

### Part 1

Today saw us extrapolating numbers using a specific technique. I don't think I can explain the [problem](https://adventofcode.com/2023/day/9) any better than it already is.

The first step was taking a first series of numbers `0 3 6 9 12 15` and producing the differences between each of them so that you get a result like this:

```
0   3   6   9  12  15
  3   3   3   3   3
```

```rust
fn calculate_differences(values: &[i32]) -> Vec<i32> {
    let mut differences = Vec::new();
    for i in 0..values.len() - 1 {
        differences.push(values[i + 1] - values[i]);
    }
    differences
}
```

The only slightly complex part was repeating this step until you reached a row of all `0` and then working up the right edge to get the extrapolations.

```rust
fn extrapolate_value(input: &[i32]) -> Result<i32> {
    let mut values = vec![input.to_vec()];

    loop {
        let bottom = values
            .last()
            .ok_or_else(|| Error::CouldNotGetBottomRowOfValues)?;

        if bottom.iter().all(|n| *n == 0) {
            break;
        }

        values.push(calculate_differences(bottom));
    }

    for row_index in (0..values.len() - 1).rev() {
        let row_last_value = values[row_index]
            .last()
            .ok_or_else(|| Error::CouldNotGetLastValueOfRow(row_index))?;

        let row_below_last_value = values[row_index + 1]
            .last()
            .ok_or_else(|| Error::CouldNotGetLastValueOfRow(row_index + 1))?;

        let next_value = row_last_value + row_below_last_value;

        values[row_index].push(next_value);
    }

    Ok(values[0][values[0].len() - 1])
}

pub fn process(input: &str) -> Result<i32> {
    let input = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| {
                    n.parse::<i32>()
                        .map_err(Error::CouldNotParseNumber)
                })  
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    let extrapolations = input
        .iter()
        .map(|row| extrapolate_value(row))
        .collect::<Result<Vec<_>>>()?;

    let sum = extrapolations.iter().sum();

    Ok(sum)
}
```

### Part 2

I half expected part 2 to be another maths thing like [yesterday](https://zoeaubert.me/blog/advent-of-code-2023-day-08/), but no, this was a nice simple twist. Rather than extrapolating forwards, we need to extrapolate backwards. The only change needed was to `extrapolate_value` which operated on the first rather than last elements in the arrays and inserted the value at the start rather than the end.

```rust
fn extrapolate_value(input: &[i32]) -> Result<i32> {
    let mut values = vec![input.to_vec()];

    loop {
        let bottom = values
            .last()
            .ok_or_else(|| Error::CouldNotGetBottomRowOfValues)?;

        if bottom.iter().all(|n| *n == 0) {
            break;
        }

        values.push(calculate_differences(bottom));
    }

    for row_index in (0..values.len() - 1).rev() {
        let row_last_value = values[row_index]
            .first()
            .ok_or_else(|| Error::CouldNotGetFirstValueOfRow(row_index))?;

        let row_below_last_value = values[row_index + 1]
            .first()
            .ok_or_else(|| Error::CouldNotGetFirstValueOfRow(row_index + 1))?;

        let next_value = row_last_value - row_below_last_value;

        values[row_index].insert(0, next_value);
    }

    Ok(values[0][0])
}
```

## Optimisation

There might be some neat maths trick I don't know about, but I can't see any algorithmic thing I can do to optimise this in any meaningful way.

## Thoughts

Today was a nice easy puzzle compared to some of the other days this week. It's nice to have an easy solution to an easy problem. Though it makes me worry about tomorrow 🤣

## Results

```
day_09    fastest       │ slowest       │ median        │ mean
├─ part1  311.9 µs      │ 405.2 µs      │ 313.6 µs      │ 316.4 µs
╰─ part2  319.9 µs      │ 338.9 µs      │ 321 µs        │ 322.8 µs 
```