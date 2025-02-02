---
slug: advent-of-code-2023-day-02
date: 2023-12-02T18:00
title: Advent of Code 2023 - Day 02
description: A discussion of my solution to Advent of Code 2023 - Day 02. This post contains spoilers
tags: ['Programming', 'AdventOfCode', 'Rust']
---
[Advent of Code](https://adventofcode.com/) is a yearly programming challenge. See my [previous post](https://zoeaubert.me/blog/advent-of-code-2023-day-01/) to see how the project is set up.

To view my [solutions](https://github.com/GeekyAubergine/advent-of-code/tree/main/2023/day-02) in full, check them out on GitHub. See my [previous posts](https://zoeaubert.me/tags/advent-of-code/) for other solutions.

## Initial solutions

### Part 1

Today saw us calculating if it was possible to draw a "hand" of cubes out of a bag containing a set number of cubes. We were told that the bag contains `12 red cubes, 13 green cubes, and 14 blue cubes`, so drawing 10 red cubes would be valid, but 20 would not.

My initial approach is possibly a little over-engineered, but here we are. I built `structs` (objects) to store the data I needed.

```rust
struct Bag {
    red: u8,
    green: u8,
    blue: u8,
}

struct Hand {
    red: u8,
    green: u8,
    blue: u8,
}

struct Game {
    id: u32,
    hands: Vec<Hand>, // Vec = array
}
```

I decided it would be best to work from backwards and parse the smallest bits of data before tackling the larger stuff, so I set up a test and built a parser for each hand. The input into this parser is `1 red, 2 green, 3 blue`.

```rust
impl Hand {
    fn from_str(input: &str) -> Result<Self> {
        let mut hand = Self {
            red: 0,
            green: 0,
            blue: 0,
        };

        for card in input.split(',') {
            let parts = card.trim().split(' ').collect::<Vec<_>>();

            let count = parts
                .first()
                .ok_or_else(|| Error::CouldNotParseColorCount(card.to_string()))?;
            let color = parts
                .last()
                .ok_or_else(|| Error::CouldNotParseColorCount(card.to_string()))?;

            let count = count
                .parse::<u8>()
                .map_err(|_| Error::CouldNotParseCount(count.to_string()))?;

            match *color {
                "red" => hand.red = count,
                "green" => hand.green = count,
                "blue" => hand.blue = count,
                _ => return Err(Error::UnknownColor(color.to_string())),
            }
        }

        Ok(hand)
    }

    fn is_possible(&self, bag: &Bag) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }
}
```

I also added a function to check if the hand was valid for any given `bag`. There's a fair bit of error handling here, but it's not too messy. It splits the string on `,`, finds the number part, parses it, parses the colour component, and throws suitable errors.

For non-Rust people, `impl` allows you to add functions to a `struct`. If you're familiar with object-oriented programming, then these are similar to class methods. In Rust, we define the data structures separately from the implementation. This might seem weird, but it does have more advantages down the line.

After getting that down, it was then just a process of adding parsers that relied upon previously built parsers.

```rust
impl Game {
    fn from_str(input: &str) -> Result<Self> {
        let id_and_hands = input.split(':').collect::<Vec<_>>();

        let id = id_and_hands
            .first()
            .ok_or_else(|| Error::CouldNotParseGameId(input.to_string()))?
            .trim()
            .split(' ')
            .nth(1)
            .ok_or_else(|| Error::CouldNotParseGameId(input.to_string()))?
            .parse::<u32>()
            .map_err(|_| Error::CouldNotParseGameId(input.to_string()))?;

        let hands = id_and_hands
            .last()
            .ok_or_else(|| Error::CouldNotParseGameHands(input.to_string()))?;

        let hands = hands
            .split(';')
            .map(Hand::from_str)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { id, hands })
    }

    fn is_possible(&self, bag: &Bag) -> bool {
        self.hands.iter().all(|hand| hand.is_possible(bag))
    }
}

pub fn process(input: &str) -> miette::Result<u32> { // Main function
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?;

    let possible_games = games
        .iter()
        .filter(|game| game.is_possible(&bag))
        .map(|game| game.id)
        .sum();

    Ok(possible_games)
}
```

I am leaning quite heavily into iterators here, but hopefully, if you're familiar with using them in other languages or lambda-like functions, these won't seem too weird.  `ok_or_else` transforms an `Option` (a different way of handling `null`) into an error if it's `None/null`.

Overall, this wasn't too complex, and I feel the code is pretty clean.

### Part 2

Part 2 was possibly easier than Part 1. It was very simple to change the logic from "is hand possible" to "work out the max hand". The only real change was to `Game`.

```rust
impl Game {
    fn min_possible_bag(&self) -> Bag {
        let mut bag = Bag {
            red: 0,
            green: 0,
            blue: 0,
        };

        for hand in &self.hands {
            bag.red = bag.red.max(hand.red);
            bag.green = bag.green.max(hand.green);
            bag.blue = bag.blue.max(hand.blue);
        }

        bag
    }

    #[tracing::instrument]
    fn power_set(&self) -> u32 {
        let bag = self.min_possible_bag();

        bag.red as u32 * bag.green as u32 * bag.blue as u32
    }
}
```

With this, it was easy to change the `process` function to respect it.

```rust
pub fn process(input: &str) -> miette::Result<u32> {
    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?;

    let power_sets = games
        .iter()
        .map(|game| game.power_set())
        .collect::<Vec<_>>();

    Ok(power_sets.iter().sum())
}
```

## Optimisation

I had a lot of fun with this.

### Part 1

#### Optimisation Pass 1

While building the initial solution, I spotted a number of areas that could be improved. Mostly parsing more data than necessary. My initial solution parsed everything, and only once it's built a complete picture did it determine the number of possible games. But we know if a game is impossible much more quickly than that. If, at any point, while parsing a hand colour value, we spot a value that is higher than the possible number, we know that nothing else in that game matters as we've broken a rule, so we can exit at that point and declare the game impossible. The first step was parsing the hands:

```rust
fn parse_hand_color(input: &str, bag: &Bag) -> Result<bool> {
    let count_chars = input
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();

    let color_start = count_chars.len() + 1;

    let count = count_chars
        .parse::<u8>()
        .map_err(|_| Error::CouldNotParseCount(input.to_string()))?;

    if count > bag.red || count > bag.green || count > bag.blue {
        return Ok(false);
    }

    let color = input
        .get(color_start..color_start + 1)
        .ok_or_else(|| Error::CouldNotParseColorCount(input.to_string()))?;

    match color {
        "r" => {
            if count > bag.red {
                return Ok(false);
            }
        }
        "g" => {
            if count > bag.green {
                return Ok(false);
            }
        }
        "b" => {
            if count > bag.blue {
                return Ok(false);
            }
        }
        _ => return Err(Error::UnknownColor(color.to_string())),
    }

    Ok(true)
}

fn parse_hand(input: &str, bag: &Bag) -> Result<bool> {
    for card in input.split(',') {
        if !parse_hand_color(card.trim(), bag)? {
            return Ok(false);
        }
    }

    Ok(true)
}
```

The actual parsing here looks similar to the first parser. Notable differences include not splitting the string but instead using what we know about the structure of the input to be able to jump ahead (this will be important later). And, rather than relying on iterators, moving back to traditional loops and exiting as early as possible. 

We also don't return a `Hand` struct anymore, but instead, just return if the hand is possible or not. One additional trick I pulled was rather than checking the whole colour name, I only check the first letter as we know there are no collisions between the colour's first characters.

The `Game` and input processing have changed in a similar way. 

```rust
enum GameResult {
    Possible { game_id: u32 },
    Impossible,
}

fn parse_game(input: &str, bag: &Bag) -> Result<GameResult> {
    let id_chars: String = input
        .chars()
        .skip(5)
        .take_while(|c| c.is_ascii_digit())
        .collect();

    let hands_start = 5 + id_chars.len() + 2;

    let game_id = id_chars
        .parse::<u32>()
        .map_err(|_| Error::CouldNotParseGameId(id_chars))?;

    let hands_text = input[hands_start..].trim();

    for hand in hands_text.split(';') {
        if !parse_hand(hand, bag)? {
            return Ok(GameResult::Impossible);
        }
    }

    Ok(GameResult::Possible { game_id })
}

pub fn process(input: &str) -> miette::Result<u32> {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut possible_game_ids = vec![];

    for line in input.lines() {
        let game_result = parse_game(line.trim(), &bag)?;
        match game_result {
            GameResult::Possible { game_id } => {
                possible_game_ids.push(game_id);
            }
            GameResult::Impossible => {}
        }
    }

    Ok(possible_game_ids.iter().sum())
}
```

I also introduced `GameResult` to pass the minimum amount of data required back to the caller to be able to determine if the game was possible and what ID it had. I dislike tuples so chose not to use them.

These optimisations resulted in a ~2.6x performance increase, but we can go further.

#### Optimisation Pass 2

Building on what I touched on before, there's a lot of performance to be gained by our knowledge of the structure of the input.

```
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
```

We know that the first 5 characters are skippable. We know that after the end of the game ID, there are two characters of dead space, and then we start parsing hands. We then know that after the number of cubes for each colour, there's a space, a colour and then a `,` or a `;`. Turns out we don't care about whether it's a `,` or a `;` as there are no repeated values per hand. So all we actually need to check is each number colour pairing, and we can forget the concept of hands. Additionally, building on the previous, we only need to check the first letter of colour and we automatically know the length of the string and can skip to the next hand without having to parse more data. 

Before I show you the code, this is not what I'd call best practices, while there's nothing `unsafe`, there is potential for index out-of-bounds errors to occur.

```rust
enum HandResult {
    Possible { length: usize },
    Impossible,
}

fn parse_hand_color(input: &str, bag: &Bag) -> Result<HandResult> {
    let mut count_chars: String = String::new();

    for c in input[0..5].chars() {
        if c.is_ascii_digit() {
            count_chars.push(c);
        } else {
            break;
        }
    }
    let color_start = count_chars.len() + 1;

    let count = count_chars
        .parse::<u8>()
        .map_err(|_| Error::CouldNotParseCount(input.to_string()))?;

    let color = input
        .get(color_start..color_start + 1)
        .ok_or_else(|| Error::CouldNotParseColorCount(input.to_string()))?;

    match color {
        "r" => {
            if count > bag.red {
                return Ok(HandResult::Impossible);
            } else {
                return Ok(HandResult::Possible {
                    length: color_start + 3,
                });
            }
        }
        "g" => {
            if count > bag.green {
                return Ok(HandResult::Impossible);
            } else {
                return Ok(HandResult::Possible {
                    length: color_start + 5,
                });
            }
        }
        "b" => {
            if count > bag.blue {
                return Ok(HandResult::Impossible);
            } else {
                return Ok(HandResult::Possible {
                    length: color_start + 4,
                });
            }
        }
        _ => return Err(Error::UnknownColor(color.to_string())),
    }
}
```

Firstly we've gone from using an iterator to a string building for-loop to get the number of cubes in a hand. Previously we were checking for digits on all characters, but we don't need to as we know as soon as we've seen a non-digit character we can stop checking. From there the code is fairly similar to the previous implementation, but this time if it is a possible hand we also return the length of consumed input to allow us to skip to the next hand.

```rust
fn parse_game(input: &str, bag: &Bag) -> Result<GameResult> {
    let mut id_chars: String = String::new();

    for c in input[5..10].chars() {
        if c.is_ascii_digit() {
            id_chars.push(c);
        } else {
            break;
        }
    }

    let hands_start = 5 + id_chars.len() + 2;

    let game_id = id_chars
        .parse::<u32>()
        .map_err(|_| Error::CouldNotParseGameId(id_chars))?;

    let mut index = hands_start;

    while index < input.len() {
        let hand = &input[index..];

        let hand_result = parse_hand_color(hand, bag)?;

        match hand_result {
            HandResult::Possible { length } => {
                index += length + 2;
            }
            HandResult::Impossible { .. } => {
                return Ok(GameResult::Impossible);
            }
        }
    }

    Ok(GameResult::Possible { game_id })
}
```

Using a similar technique to get the game ID, we then use the new hand parser to skip through the list of hands using its returned length. The `process` function remains unchanged.

This final approach has a ~4.2x performance gain over the initial solution and a ~1.6x over the previously optimised solution. Is the new unreadability worth it? Probably not, but it sure is fun.

### Part 2

While there we some changes required to optimise part 2, it's similar to part 1. The `parse_hand_color` function now returns a hand and colour rather than a validity check.

```rust
enum Hand {
    Red { consumed: u8, count: u8 },
    Green { consumed: u8, count: u8 },
    Blue { consumed: u8, count: u8 },
}

fn parse_hand_color(input: &str) -> Result<Hand> {
    let mut count_chars: String = String::new();

    for c in input[0..5].chars() {
        if c.is_ascii_digit() {
            count_chars.push(c);
        } else {
            break;
        }
    }
    let color_start = count_chars.len() + 1;

    let count = count_chars
        .parse::<u8>()
        .map_err(|_| Error::CouldNotParseCount(input.to_string()))?;

    let color = input
        .get(color_start..color_start + 1)
        .ok_or_else(|| Error::CouldNotParseColorCount(input.to_string()))?;

    match color {
        "r" => Ok(Hand::Red {
            consumed: color_start as u8 + 3,
            count,
        }),
        "g" => Ok(Hand::Green {
            consumed: color_start as u8 + 5,
            count,
        }),
        "b" => Ok(Hand::Blue {
            consumed: color_start as u8 + 4,
            count,
        }),
        _ => return Err(Error::UnknownColor(color.to_string())),
    }
}
```

Simialrly, `parse_game` saw some changes but still very similar to the part 1 optimistion.

```rust
fn parse_game(input: &str) -> Result<u32> {
    let input = input.trim();
        
    let mut hands_start = 0;

    for c in input[0..10].chars() {
        if c.eq(&':') {
            break;
        } else {
            hands_start += 1;
        }
    }

    hands_start += 1;

    let mut index = hands_start;

    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;

    while index < input.len() {
        let hand = &input[index..];

        let hand_result = parse_hand_color(hand.trim())?;

        match hand_result {
            Hand::Red { consumed, count } => {
                max_red = max_red.max(count as u32);
                index += consumed as usize;
            }
            Hand::Green { consumed, count } => {
                max_green = max_green.max(count as u32);
                index += consumed as usize;
            }
            Hand::Blue { consumed, count } => {
                max_blue = max_blue.max(count as u32);
                index += consumed as usize;
            }
        }

        index += 2;
    }

    Ok(max_red * max_green * max_blue)
}

pub fn process(input: &str) -> miette::Result<u32> {
    let power_sets = input.lines().map(parse_game).collect::<Result<Vec<_>>>()?;

    Ok(power_sets.iter().sum())
}
```

This approach yielded a ~2.8x performance increase.

## Additional Notes

I did try going further with the optimisation for part 1, but no changes I made improved upon my second attempt. This included some rather unpleasant un-check array reading and unsafe code, but it was still ultimately slower, which surprised me. And again, multi-threading did not help here as the input data was tiny.

## Results

```
day_02         fastest       │ slowest       │ median        │ mean
├─ part1       77.79 µs      │ 197 µs        │ 159.3 µs      │ 142.4 µs
├─ part1_opt   53.41 µs      │ 82.79 µs      │ 60.08 µs      │ 61.02 µs
├─ part1_opt2  36.66 µs      │ 43.2 µs       │ 37.08 µs      │ 37.45 µs
├─ part2       104.3 µs      │ 144.4 µs      │ 112.8 µs      │ 116.2 µs 
╰─ part2_opt   49.2 µs       │ 58.99 µs      │ 49.49 µs      │ 49.96 µs
```