---
slug: advent-of-code-2023-day-06
date: 2023-12-06T11:00
title: Advent of Code 2023 - Day 06
description: A discussion of my solution to Advent of Code 2023 - Day 06. I optimised part 2 down to ~420ns. This post contains spoilers.
tags: ['Programming', 'AdventOfCode', 'Rust']
---
[Advent of Code](https://adventofcode.com/) is a yearly programming challenge. See my [day 01 post](https://zoeaubert.me/blog/advent-of-code-2023-day-01/) to see how the project is set up.

To view my [solutions](https://github.com/GeekyAubergine/advent-of-code/tree/main/2023/day-06) in full, check them out on GitHub. See my [previous posts](https://zoeaubert.me/tags/advent-of-code/) for other solutions.

## Initial solutions

### Part 1

Today was much nicer than yesterday's. We were tasked with working out the optimal strategy for boat racing. A race lasts an amount of milliseconds (very short races) and has a record distance. For your boat to move, you must charge it. For every millisecond you charge it the boat goes 1 mm/s faster (it never slows down). Then, for any given race, you need to work out how many milliseconds to charge for in order to beat the record before the race ends.

The input was a series of time and distance pairs, each representing a race.

```
Time:      7  15   30
Distance:  9  40  200
```

Parsing this was pretty simple.

```rust
struct Race {
    time: u64,
    distance: u64,
}

fn numbers_from_line(input: &str) -> Result<Vec<u64>> {
    let colon_split: Vec<&str> = input.split(": ").collect();

    colon_split[1]
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().map_err(Error::CouldNotParseNumber))
        .collect()
}

fn input_to_races(input: &str) -> Result<Vec<Race>> {
    let mut races = vec![];

    let lines: Vec<&str> = input.split('\n').map(|l| l.trim()).collect();

    let times = numbers_from_line(lines[0])?;
    let distances = numbers_from_line(lines[1])?;

    for (i, time) in times.iter().enumerate() {
        match distances.get(i) {
            Some(distance) => races.push(Race {
                time: *time,
                distance: *distance,
            }),
            None => return Err(Error::MissingDistance(i)),
        }
    }

    Ok(races)
}
```

The puzzle is to determine how many ways you can beat each race and then multiply them together.

```rust
fn calculate_max_distance_for_time(press_down_time: u64, max_time: u64) -> u64 {
    let time_remaining = max_time - press_down_time;
    time_remaining * press_down_time
}

fn number_of_ways_to_beat_race(race: &Race) -> u64 {
    (0..race.time)
        .map(|t| calculate_max_distance_for_time(t, race.time))
        .filter(|t| t > &race.distance)
        .count() as u64
}

pub fn process(input: &str) -> miette::Result<u64> {
    let races = input_to_races(input)?;

    Ok(races.iter().map(number_of_ways_to_beat_race).product())
}
```

And that's it. Compared to previous challenges, this is nice, small and very clean.

### Part 2

Based on [yesterday](https://zoeaubert.me/blog/advent-of-code-2023-day-05/) I had a funny feeling that they'd make the numbers very big. And they did. Rather than the input being multiple races, it's a single race and you need to ignore the spaces. The new race is:

```
Time:      71530
Distance:  940200
```

This number's still pretty small, let's see if it works without optimising.

```rust
pub fn process(input: &str) -> miette::Result<u64> {
    let races = input_to_races(input)?;

    Ok(races.iter().map(number_of_ways_to_beat_race).product())
}
```

It worked, and it didn't take the lifetime of the universe. Nice. But I know we can do better.

## Optimisation

There's one obvious optimisation here, and I _think_ they're even trying to hint at it in the statement.

> Since the current record for this race is `9` millimetres, there are actually `_4_` different ways you could win: you could hold the button for `2`, `3`, `4`, or `5` milliseconds at the start of the race.

> In the second race, you could hold the button for at least `4` milliseconds and at most `11` milliseconds and beat the record, a total of `_8_` different ways to win.

For each race, the only number that matters is the first and last charging time that'd beat the record. You know everything outside that range will lose, and anything inside that range will win.

You could search from each end to find the values, but I think [bisectional search](https://en.wikipedia.org/wiki/Binary_search_algorithm) lends itself much better to this problem, especially knowing how large the input is. Thanks to the way I've built it, it only requires changing `number_of_ways_to_beat_race`.

```rust
fn find_first_winning_number(race: &Race) -> u64 {
    let mut low = 0;
    let mut high = race.time;

    loop {
        let index = (low + high) / 2;
        let left = index - 1;

        let distance = calculate_max_distance_for_time(index, race.time);
        let left_distance = calculate_max_distance_for_time(left, race.time);

        if distance > race.distance && left_distance <= race.distance {
            return index;
        }

        if distance <= race.distance {
            low = index;
        } else {
            high = index;
        }
    }
}

fn find_last_winning_number(race: &Race) -> u64 {
    let mut low = 0;
    let mut high = race.time;

    loop {
        let index = (low + high) / 2;
        let right = index + 1;

        let distance = calculate_max_distance_for_time(index, race.time);
        let right_distance = calculate_max_distance_for_time(right, race.time);

        if distance > race.distance && right_distance <= race.distance {
            return index;
        }

        if distance > race.distance {
            low = index;
        } else {
            high = index;
        }
    }
}

fn number_of_ways_to_beat_race(race: &Race) -> u64 {
    find_last_winning_number(race) - find_first_winning_number(race)
}
```

Yes, I'm doing two separate searches for each edge of the winners. You could probably do it in one. I tried it, but it was far too messy, and I don't think it would be that much faster.

I applied this to both parts and got a ~1.4x improvement on part 1 and a ~124,324x improvement on part 2. That bisectional search is really doing work on that second part.

For those who are curious or, like me, found the part 2 improvement suspicious, the tests are running with 41ns precision, and part 2 had 100 samples with 1600 iterations. The full [benchmarking output](https://github.com/GeekyAubergine/advent-of-code/blob/main/2023/benchmarks/all.txt) is available on GitHub.

## Thoughts

I'm very pleased with how quickly I saw the optimisation and how well it performed. Makes learning and practising the implementation almost worth it 🤣

## Results

```
day_06        fastest       │ slowest       │ median        │ mean
├─ part1      790.7 ns      │ 5.04 µs       │ 791.7 ns      │ 852.1 ns
├─ part1_opt  541.3 ns      │ 1.786 µs      │ 551.7 ns      │ 584.3 ns
├─ part2      48.88 ms      │ 75.92 ms      │ 50.14 ms      │ 50.93 ms
╰─ part2_opt  387.7 ns      │ 424.1 ns      │ 403.3 ns      │ 403.6 ns
```