---
slug: advent-of-code-2023-day-04
date: 2023-12-04T15:30
title: Advent of Code 2023 - Day 04
description: A discussion of my solution to Advent of Code 2023 - Day 04. This post contains spoilers.
tags: ['Programming', 'AdventOfCode', 'Rust']
---
[Advent of Code](https://adventofcode.com/) is a yearly programming challenge. See my [day 01 post](https://zoeaubert.me/blog/advent-of-code-2023-day-01/) to see how the project is set up.

To view my [solutions](https://github.com/GeekyAubergine/advent-of-code/tree/main/2023/day-04) in full, check them out on GitHub. See my [previous posts](https://zoeaubert.me/tags/advent-of-code/) for other solutions.

## Initial solutions

### Part 1

Today saw us parsing [scratchcards](https://en.wikipedia.org/wiki/Scratchcard). 

```
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
```

Each card has a number, a set of winning numbers on the left of the `|` and the player's numbers on the right. In this case, this is card `1`, the winning numbers are `41 48 83 86 17` and the player's numbers are `83 86 6 31 1 7 9`. If the player has revealed a winning number, the card is worth `1` point, if the player reveals any more winning numbers it doubles for each additional winner. 

I started with a function to parse each line and extract the numbers from it. This is built around splitting and parsing. The code is pretty self-explanatory. The `ok_or` is for error handling to change `Option` (null) to an error.

```rust
pub fn score_line(line: &str) -> Result<u32> {
    let numbers = line
        .split(':')
        .last()
        .ok_or(Error::CannotFindNumbers { line: 0 })?;

    let mut numbers = numbers.split('|');

    let winning_numbers = numbers
        .next()
        .ok_or(Error::CannotFindWinningNumbers { line: 0 })?
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| {
            n.parse::<u32>()
                .map_err(|_| Error::CouldNotParseNumber(n.to_string()))
        })
        .collect::<Result<Vec<_>>>()?;

    let scratch_numbers = numbers
        .last()
        .ok_or(Error::CannotFindScratchedNumbers { line: 0 })?
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| {
            n.parse::<u32>()
                .map_err(|_| Error::CouldNotParseNumber(n.to_string()))
        })
        .collect::<Result<Vec<_>>>()?;
}
```

From here, it's pretty easy to add the scoring part:

```rust
pub fn score_line(line: &str) -> Result<u32> {
    let numbers = line
        .split(':')
        .last()
        .ok_or(Error::CannotFindNumbers { line: 0 })?;

    let mut numbers = numbers.split('|');

    let winning_numbers = numbers
        .next()
        .ok_or(Error::CannotFindWinningNumbers { line: 0 })?
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| {
            n.parse::<u32>()
                .map_err(|_| Error::CouldNotParseNumber(n.to_string()))
        })
        .collect::<Result<Vec<_>>>()?;

    let scratch_numbers = numbers
        .last()
        .ok_or(Error::CannotFindScratchedNumbers { line: 0 })?
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| {
            n.parse::<u32>()
                .map_err(|_| Error::CouldNotParseNumber(n.to_string()))
        })
        .collect::<Result<Vec<_>>>()?;

    let winning_scratched = winning_numbers
        .iter()
        .filter(|n| scratch_numbers.contains(n))
        .count();

    if winning_scratched == 0 {
        return Ok(0);
    }

    Ok(1 << (winning_scratched - 1))
}
```

The "worth one point and then doubling" lent itself nicely to [bit shifting](https://en.wikipedia.org/wiki/Bitwise_operation#Logical_shift). This feels much neater than other solutions and doesn't require an additional loop. From there it was pretty easy to turn this into a full solution.

```rust
pub fn process(input: &str) -> miette::Result<u32> {
    let out = input
        .lines()
        .map(|line| score_line(line.trim()))
        .collect::<Result<Vec<_>>>()
        .map(|v| v.iter().sum())?;

    Ok(out)
}
```

### Part 2

The second part was much more interesting, rather than a card scoring points. For each winning number, it gave you a copy of the next `n` number of winning cards to scratch off. For example, card 1 has four winning numbers, so you get a copy of cards 2, 3, 4, and 5. This process then repeats for all of the player's cards. If you have multiple copies of a card, you play them all. For example, after winning a copy of card 2, you would play it twice, win twice, and get two additional copies of cards 3 and 4.

To keep track of the number of copies of each card the player has, the best solution is a [HashMap](https://en.wikipedia.org/wiki/Hash_table), as it allows quick indexing and modification by an index (card number).

```rust
struct Cards {
    copies: HashMap<u32, u32>,
}

impl Cards {
    fn new() -> Self {
        Self {
            copies: HashMap::new(),
        }
    }

    fn add_card(&mut self, card: u32) {
        *self.copies.entry(card).or_insert(0) += 1;
    }

    fn add_card_copies(&mut self, card: u32, copies: u32) {
        *self.copies.entry(card).or_insert(0) += copies;
    }

    fn get_count(&self, card: u32) -> u32 {
        *self.copies.get(&card).unwrap_or(&0)
    }
}
```

From there the code looks pretty similar bar the new way of scoring

```rust
pub fn score_line(line: &str, mut cards: Cards) -> Result<Cards> {
    let mut card_and_numbers = line.split(':');

    let card_number = card_and_numbers
        .next()
        .ok_or_else(|| Error::CannotFindCardNumber(line.to_owned()))?
        .split(' ')
        .last()
        .ok_or_else(|| Error::CannotFindCardNumber(line.to_owned()))?
        .parse::<u32>()
        .map_err(|_| Error::CouldNotParseCardNumber(line.to_owned()))?;

	// Add self for the original copy of card
    cards.add_card(card_number);

    let numbers = card_and_numbers
        .last()
        .ok_or(Error::CannotFindNumbers { line: 0 })?;

    let mut numbers = numbers.split('|');

    let winning_numbers = numbers
        .next()
        .ok_or(Error::CannotFindWinningNumbers { line: 0 })?
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| {
            n.parse::<u32>()
                .map_err(|_| Error::CouldNotParseNumber(n.to_string()))
        })
        .collect::<Result<Vec<_>>>()?;

    let scratch_numbers = numbers
        .last()
        .ok_or(Error::CannotFindScratchedNumbers { line: 0 })?
        .split(' ')
        .filter(|n| !n.is_empty())
        .map(|n| {
            n.parse::<u32>()
                .map_err(|_| Error::CouldNotParseNumber(n.to_string()))
        })
        .collect::<Result<Vec<_>>>()?;

    let winning_scratched = winning_numbers
        .iter()
        .filter(|n| scratch_numbers.contains(n))
        .count();

    let copies = cards.get_count(card_number);

    for i in 1..=winning_scratched {
        cards.add_card_copies(card_number + i as u32, copies);
    }

    Ok(cards)
}
```

Note that because this is going to be used in a `fold`(reduce) function, this function takes mutable ownership of `Cards` and returns the mutated copy rather than mutating by reference. There are ways to do it by reference, but this is much cleaner.

```rust
pub fn process(input: &str) -> miette::Result<u32> {
    let mut lines = input.lines();

    let cards = lines.try_fold(Cards::new(), |cards, line| score_line(line.trim(), cards))?;

    let card_count = input
        .lines()
        .enumerate()
        .map(|(i, _line)| cards.get_count(i as u32 + 1))
        .sum::<u32>();

    Ok(card_count)
}
```

I'm happy with this implementation as it feels clean and clear.

## Optimisation

After playing around with it for a bit, I only found one optimisation of use. Rather than using my previous technique for parsing numbers, I built a new function to do it:

```rust
fn parse_numbers(input: &str) -> Result<Vec<u32>> {
    let input = input.trim();

    let mut in_number = false;
    let mut numbers = vec![];
    let mut number_start = 0;

    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_digit() {
            if !in_number {
                in_number = true;
                number_start = i;
            }
        } else if in_number {
            numbers.push(
                input[number_start..i]
                    .parse()
                    .map_err(|_| Error::CouldNotParseNumber(input.to_string()))?,
            );
            in_number = false;
        }
    }

    if in_number {
        numbers.push(
            input[number_start..]
                .parse()
                .map_err(|_| Error::CouldNotParseNumber(input.to_string()))?,
        );
    }

    Ok(numbers)
}

fn score_line(line: &str) -> Result<u32> {
    let numbers = line
        .split(':')
        .last()
        .ok_or(Error::CannotFindNumbers { line: 0 })?;

    let mut numbers = numbers.split('|');

    let winning_numbers = numbers
        .next()
        .ok_or(Error::CannotFindWinningNumbers { line: 0 })?;

    let winning_numbers = parse_numbers(winning_numbers)?;

    let scratch_numbers = numbers
        .last()
        .ok_or(Error::CannotFindScratchedNumbers { line: 0 })?;

    let scratch_numbers = parse_numbers(scratch_numbers)?;

    let winning_scratched = winning_numbers
        .iter()
        .filter(|n| scratch_numbers.contains(n))
        .count();

    if winning_scratched == 0 {
        return Ok(0);
    }

    Ok(1 << (winning_scratched - 1))
}
```

Doing this yielded a ~1.4x improvement for both part 1 and part 2.

I did try a few other things, including making `winning_numbers` a HashSet rather than an array, but the upfront cost seemed to contract any gains, which is expected on a data set of this size.

## Results

```
day_04        fastest       │ slowest       │ median        │ mean
├─ part1      157.9 µs      │ 213.4 µs      │ 164.9 µs      │ 167.3 µs
├─ part1_opt  114.4 µs      │ 152.3 µs      │ 114.7 µs      │ 116.7 µs
├─ part2      189.4 µs      │ 227.6 µs      │ 192.8 µs      │ 193.5 µs
╰─ part2_opt  136.8 µs      │ 164.2 µs      │ 140.5 µs      │ 141.8 µs
```