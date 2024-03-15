---
slug: advent-of-code-2023-day-01
date: 2023-12-01T13:00
title: Advent of Code 2023 - Day 01
description: A discussion of my solution to Advent of Code 2023 - Day 01. This post contains spoilers
tags: ['Programming', 'AdventOfCode', 'Rust']
---
[Advent of Code](https://adventofcode.com/) is a yearly programming challenge. I've followed other people doing it before and thought I'd take a crack at it this year. Let's see how far I get. It goes without saying that this post will contain spoilers.

To view my [solutions](https://github.com/GeekyAubergine/advent-of-code/tree/main/2023/day-01) in full, check them out on GitHub. See my [previous posts](https://zoeaubert.me/tags/advent-of-code/) for other solutions.

## Setup and structure

Unsurprisingly I'm going to use [Rust](https://www.rust-lang.org/) for this. I had an idea about how I wanted to structure the project but then [Chris Biscardi](https://www.youtube.com/@chrisbiscardi) posted a great [video](https://www.youtube.com/watch?v=fEQv-cqzbPg) outlining their project structure. I've ~~borrowed~~ _yoinked_ their approach and made some small modifications. It's a little confusing at first but if you've used Rust before it's probably fine. The video explains it well.

You'll notice I have `part1.rs` and `part1_opt.rs`. I thought it would be to separately store my first pass approach, and a more optimised approach and compare the two. It helps that I have a benchmarking tool that allows me to directly compare my solutions. The [benchmarks](https://github.com/GeekyAubergine/advent-of-code/blob/main/2023/benchmarks/all.txt) are stored here.

## Initial solutions

### Part 1

This was a nice easy start. Given a set of strings, extract the first and last digit from each string, and join them to make a 2 digit number, then sum all the digits. For example, `m9qvkqlgfhtwo3seven4seven` becomes 94.

To get the digits I first extracted the digits from the string:

```rust
fn extract_digits(input: &str) -> Vec<u64> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .collect()
}
```

This is pleasantly easy in Rust. I then took the digits, got the first and last, and build a new string to then parse:

```rust
fn number_for_line(line: &str) -> Result<u64> {
    let digits = extract_digits(line);
    let first = digits.first().ok_or_else(|| Error::NoFirstDigitInLine)?;
    let last = digits.last().ok_or_else(|| Error::NoLastDigitInLine)?;
    let string = format!("{}{}", first, last);
    Ok(string.parse::<u64>()?)
}
```

(If you're new to Rust you can ignore the `ok_or_else` part, it's just error handling and does not change the way it works). 

From there it again, very simple to sum up the value from each line:

```rust
pub fn process(input: &str) -> miette::Result<u64> {
    Ok(input
        .lines()
        .map(number_for_line)
        .collect::<Result<Vec<u64>>>()
        .map(|v| v.iter().sum())?)
}
```

### Part 2

Now we get the fun twist. Rather than just parsing digits, we also need to parse the numbers as words. For example `eight33` becomes `83`.

I took a potentially cursed approach to this but I didn't think RegEx was the right tool for the job, so instead I did this:

```rust
fn parse_digit(input: &str) -> Result<u64> {
    let first_char = input
        .chars()
        .next()
        .ok_or_else(|| Error::NoFirstDigitInLine)?;

    if let Some(digit) = first_char.to_digit(10) {
        return Ok(digit as u64);
    }

    if input.starts_with("zero") {
        return Ok(0);
    }

    if input.starts_with("one") {
        return Ok(1);
    }

    if input.starts_with("two") {
        return Ok(2);
    }

    if input.starts_with("three") {
        return Ok(3);
    }

    if input.starts_with("four") {
        return Ok(4);
    }

    if input.starts_with("five") {
        return Ok(5);
    }

    if input.starts_with("six") {
        return Ok(6);
    }

    if input.starts_with("seven") {
        return Ok(7);
    }

    if input.starts_with("eight") {
        return Ok(8);
    }

    if input.starts_with("nine") {
        return Ok(9);
    }

    Err(Error::ParseBasicIntError())
}
```

Cursed, I know, but it worked and that's kind of what I'm striving for. Wanting to do everything perfectly all the time is a massive problem when it comes to me actually finishing projects. I then use this new parser in a new `extract_digits` function:

```rust
fn extract_digits(input: &str) -> Result<Vec<u64>> {
    let mut digits = Vec::new();
    for i in 0..input.len() {
        match parse_digit(&input[i..]) {
            Ok(d) => digits.push(d),
            Err(_) => continue,
        }
    }

    Ok(digits)
}
```

From there everything is the same as part 1.

## Optimisation

Now on to the fun stuff, let's make this go further.

### Part 1

The first step was no to try and parse the whole string for all digits, this is wasteful. Instead I built two new functions that move inwards from either end of the string and return as soon as they've found a digit.

```rust
fn extract_first_digit(input: &str) -> Result<u8> {
    for c in input.chars() {
        if let Some(digit) = c.to_digit(10) {
            return Ok(digit as u8);
        }
    }

    Err(Error::NoFirstDigitInLine)
}

fn extract_last_digit(input: &str) -> Result<u8> {
    for c in input.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            return Ok(digit as u8);
        }
    }

    Err(Error::NoLastDigitInLine)
}
```

The next tiny improvement was to also not build a new string to parse, but instead, use "maths" . This now leaves the `number_for_line` function as:

```rust
fn number_for_line(line: &str) -> Result<u32> {
    let first = extract_first_digit(line)?;
    let last = extract_last_digit(line)?;
    Ok((first * 10 + last) as u32)
}
```

With everything else remaining the same I got a ~2.4 time performance increase. For the full results scroll to the bottom of this post.

### Part 2

After applying the same optimisations as I did in part 1 I got stuck. There was little I could see to make it better. I did try some replacement things, but there are some well thought out gotchas that tripped me up. For example `eightwothree`, depending on the order of your replacements, `two` will get replaced first leaving `eigh2three`, of which `2` is now the first digit, which is wrong. 

Interestingly this resulted in a ~3x performance gain vs part 1. I suspect this is because the parsing is more expensive so doing less of it is just better (who'd've guessed 🤣).

### Additional Notes

I also tried multithreading, and as I suspected, on a problem this small the overhead is too high to make it worth it, single-threaded is the way to go.

I never properly explored the RegEx solution, maybe it was better, but I would be surprised as I suspect you'd have to do the `starts_with` or `===` either way. However it wouldn't likely suffer from my issue where the string might be smaller than 5 characters and needs to be padded to prevent an index out-of-bounds error.

## Results

```
day_01        fastest       │ slowest       │ median        │ mean
├─ part1      104.9 µs      │ 126.7 µs      │ 106.2 µs      │ 108.8 µs
├─ part1_opt  43.83 µs      │ 60.29 µs      │ 44.08 µs      │ 45.45 µs
├─ part2      214.2 µs      │ 246.9 µs      │ 217.1 µs      │ 220 µs
╰─ part2_opt  71.24 µs      │ 97.7 µs       │ 71.83 µs      │ 74.24 µs
```