---
slug: advent-of-code-2023-day-05
date: 2023-12-05T14:45
title: Advent of Code 2023 - Day 05
description: A discussion of my solution to Advent of Code 2023 - Day 05. This post contains spoilers.
tags: ['Programming', 'AdventOfCode', 'Rust']
---
[Advent of Code](https://adventofcode.com/) is a yearly programming challenge. See my [day 01 post](https://zoeaubert.me/blog/advent-of-code-2023-day-01/) to see how the project is set up.

To view my [solutions](https://github.com/GeekyAubergine/advent-of-code/tree/main/2023/day-05) in full, check them out on GitHub. See my [previous posts](https://zoeaubert.me/tags/advent-of-code/) for other solutions.

## Initial solutions

### Part 1

I don't really know how to explain today's challenge. I would highly recommend reading the [problem](https://adventofcode.com/2023/day/5) yourself. It saw us taking seeds and working out where to plant them. Seeds were given as an array.

```
seeds: 79 14 55 13
```

From there, you were given a series of `map`s to process the seeds. 

```
seed-to-soil map:
50 98 2
52 50 48
```

These are essentially transformers that act on the seeds' value. After applying a bunch of these you then took the minimum remaining value.

As this was going to require some funky parsing I built a custom input and cursor, much easier than traversing lines in an iterator.

```rust
struct Input {
    lines: Vec<String>,
    cursor: usize,
}

impl Input {
    fn from_str(input: &str) -> Result<Input> {
        let lines = input
            .lines()
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        Ok(Input { lines, cursor: 0 })
    }

    fn peak(&self) -> Option<&String> {
        self.lines.get(self.cursor)
    }

    fn next(&mut self) -> Result<&String> {
        let next = self
            .lines
            .get(self.cursor)
            .ok_or_else(|| Error::CannotFindNextLine(self.cursor));
        self.cursor += 1;
        next
    }

    fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}
```

From there, it was a simple case of parsing the data.

```rust
type ParserOutput<T> = (T, Input);

struct Seeds {
    seeds: Vec<u32>,
}

impl Seeds {
    fn from_input(mut input: Input) -> Result<ParserOutput<Seeds>> {
        let first_line = input.next().map_err(|_| Error::CannotFindSeedsHeader)?;

        if !first_line.starts_with("seeds:") {
            return Err(Error::CannotFindSeedsHeader);
        }

        let seeds = first_line
            .split(':')
            .last()
            .ok_or_else(|| Error::CannotFindSeedsHeader)?
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<u32>().map_err(Error::CouldNotParseNumber))
            .collect::<Result<Vec<_>>>()?;

        Ok((Seeds { seeds }, input))
    }
}
```

```rust
struct Map {
    mapped_values: HashMap<u32, u32>,
}

impl Map {
    fn from_input(mut input: Input) -> Result<ParserOutput<Map>> {
        let mut mapped_values = HashMap::new();

        if !input.next()?.ends_with("map:") {
            return Err(Error::CannotFindMapHeader);
        }

        while let Some(line) = input.peak() {
            if line.is_empty() {
                break;
            }

            let line = input.next()?;

            let numbers = line
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<u32>().map_err(Error::CouldNotParseNumber))
                .collect::<Result<Vec<_>>>()?;

            if numbers.len() != 3 {
                return Err(Error::UnexpectedNumberOfValuesForMap(line.to_string()));
            }

            let destination_start = numbers[0];
            let source_start = numbers[1];
            let range = numbers[2];

            for i in 0..range {
                let source = source_start + i;
                let destination = destination_start + i;

                mapped_values.insert(source, destination);
            }
        }

        Ok((Map { mapped_values }, input))
    }

    fn get_mapped_value(&self, value: u32) -> u32 {
        *self.mapped_values.get(&value).unwrap_or(&value)
    }
}
```

There is a mistake here in `Map`, if you've already spotted it, nice, you probably suffered in a similar way, if not, stay tuned.

```rust
struct Data {
    seeds: Seeds,
    seed_to_soil_map: Map,
    soil_to_fertilizer_map: Map,
    fertilizer_to_water_map: Map,
    water_to_light_map: Map,
    light_to_temperature_map: Map,
    temparure_to_humity_map: Map,
    humidity_to_location_map: Map,
}

impl Data {
    fn from_input(input: Input) -> Result<Data> {
        let (seeds, mut input) = Seeds::from_input(input)?;
        
        input.next()?;

        let (seed_to_soil_map, mut input) = Map::from_input(input)?;

        input.next()?;

        let (soil_to_fertilizer_map, mut input) = Map::from_input(input)?;

        input.next()?;

        let (fertilizer_to_water_map, mut input) = Map::from_input(input)?;

        input.next()?;

        let (water_to_light_map, mut input) = Map::from_input(input)?;

        input.next()?;

        let (light_to_temperature_map, mut input) = Map::from_input(input)?;

        input.next()?;

        let (temparure_to_humity_map, mut input) = Map::from_input(input)?;

        input.next()?;

        let (humidity_to_location_map, _) = Map::from_input(input)?;

        Ok(Data {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temparure_to_humity_map,
            humidity_to_location_map,
        })
    }

    fn seeds(&self) -> &Seeds {
        &self.seeds
    }

    fn map_seed(&self, seed: u32) -> u32 {
        let soil = self.seed_to_soil_map.get_mapped_value(seed);
        let fertilizer = self.soil_to_fertilizer_map.get_mapped_value(soil);
        let water = self.fertilizer_to_water_map.get_mapped_value(fertilizer);
        let light = self.water_to_light_map.get_mapped_value(water);
        let temperature = self.light_to_temperature_map.get_mapped_value(light);
        let humidity = self.temparure_to_humity_map.get_mapped_value(temperature);
        let location = self.humidity_to_location_map.get_mapped_value(humidity);

        location
    }
}

pub fn process(input: &str) -> miette::Result<u32> {
    let input = Input::from_str(input)?;

    let data = Data::from_input(input)?;

    let min_location = data
        .seeds()
        .seeds
        .iter()
        .map(|seed| data.map_seed(*seed))
        .min()
        .ok_or(Error::NoMinValue)?;

    Ok(min_location)
}
```

I'm processing all the `Map`s and then passing in a single seed at a time, passing it through each `Map` and then getting its new value.

So, at this point, I was feeling good. I very quickly arrived at the point of having the example input processed and correct. So I swapped it to use the actual input, and, well, nothing happened. My computer sat there and processed. I added some logging and saw that it had only processed one line of the first `Map` after a minute.

It was at this point I looked properly at the actual input and noticed the size of the numbers. This is where my folly was. The first mapping range is:

```
2328388605 1716277852 240111965
```

Yes, that's right, it has 240,111,965 numbers. And that's just the first of many. Looking back through the code I noticed my mistake. In `Map`, I was building a [HashMap](https://en.wikipedia.org/wiki/Hash_table) of value -> mapped value. This is not good on something this size. Thankfully, because I'd over-engineered it, the only thing that needed changing was `Map`.

```rust
struct MapRange {
    destination_start: u64,
    source_start: u64,
    range: u64,
}

impl MapRange {
    fn new(destination_start: u64, source_start: u64, range: u64) -> Result<MapRange> {
        Ok(MapRange {
            destination_start,
            source_start,
            range,
        })
    }

    fn contains_value(&self, value: u64) -> bool {
        value >= self.source_start && value < self.source_start + self.range
    }

    fn map_value(&self, value: u64) -> u64 {
        if !self.contains_value(value) {
            return value;
        }

        let offset = value - self.source_start;
        let destination = self.destination_start + offset;

        destination
    }
}

struct Map {
    mapped_ranges: Vec<MapRange>,
}

impl Map {
    fn from_input(mut input: Input) -> Result<ParserOutput<Map>> {
        let mut mapped_ranges = Vec::new();

        if !input.next()?.ends_with("map:") {
            return Err(Error::CannotFindMapHeader);
        }

        while let Some(line) = input.peak() {
            if line.is_empty() {
                break;
            }

            let line = input.next()?;

            let numbers = line
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<u64>().map_err(Error::CouldNotParseNumber))
                .collect::<Result<Vec<_>>>()?;

            if numbers.len() != 3 {
                return Err(Error::UnexpectedNumberOfValuesForMap(line.to_string()));
            }

            let destination_start = numbers[0];
            let source_start = numbers[1];
            let range = numbers[2];

            let map_range = MapRange::new(destination_start, source_start, range)?;

            mapped_ranges.push(map_range);            
        }

        Ok((Map { mapped_ranges }, input))
    }

    fn get_mapped_value(&self, value: u64) -> u64 {
        self.mapped_ranges
            .iter()
            .find(|map_range| map_range.contains_value(value))
            .map(|map_range| map_range.map_value(value))
            .unwrap_or(value)
    }
}
```

Rather than precomputing all the mappings (silly in hindsight), I now keep track of the ranges, and when asked for a value, I check if it's in the range, map it as necessary, or return the original value.

This small change fixed everything. The process now not only ran, but it was fast. For a change, my over-engineering saved me. Lets hope part 2 doesn't ruin all that.

### Part 2

It ruined all that.

Part two changed the game. Instead of the `seeds` input being a list of seeds, it's now a list of pairs describing the starting seed number and then x number of following seeds.

```
seeds: 2906961955 52237479 1600322402 372221628
```

So, this went from 4 seeds to 424,459,107. And that's not the whole input. Oh boi.

I thought about it initially but decided I couldn't eyeball and optimise without a full rewrite, so I did the only logical thing. I brute forced it.

Introducing [Rayon](https://github.com/rayon-rs/rayon), my beloved. tldr; it lets you parallelise iterators with almost zero changes. 

```rust
pub fn process(input: &str) -> miette::Result<u64> {
    let input = Input::from_str(input)?;

    let data = Data::from_input(input)?;

    let min_location = data
        .seeds()
        .seeds
        .par_iter()
        .map(|seed| data.map_seed(*seed))
        .min()
        .ok_or(Error::NoMinValue)?;

    Ok(min_location)
}
```

The only change here is `iter` -> `par_iter`, (Yes, this is one of the many reasons I love Rust and this library). I then changed the `Seeds` component to handle the pairings.

```rust
impl Seeds {
    #[tracing::instrument]
    fn from_input(mut input: Input) -> Result<ParserOutput<Seeds>> {
        let first_line = input.next().map_err(|_| Error::CannotFindSeedsHeader)?;

        if !first_line.starts_with("seeds:") {
            return Err(Error::CannotFindSeedsHeader);
        }

        let seed_pairs = first_line
            .split(':')
            .last()
            .ok_or_else(|| Error::CannotFindSeedsHeader)?
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<u64>().map_err(Error::CouldNotParseNumber))
            .collect::<Result<Vec<_>>>()?;

        let mut seeds = Vec::new();

        for seed_pair in seed_pairs.chunks(2) {
            let seed = seed_pair[0];
            let count = seed_pair[1];

            for i in 0..count {
                seeds.push(seed + i);
            }
        }

        Ok((Seeds { seeds }, input))
    }
}
```

And with that, we were off. It took my Apple M2 Max 7 minutes to compute, and used just shy of 16GB of RAM. Damn.

This is clearly not the right solution, but it worked so I won't lament it. Part of my taking part in this is to accept "good enough" solutions. At least before I optimise them

## Optimisation

I don't see any need to optimise part 1, or anywhere where I could make significant inroads.

Part 2 needs revisiting. I've tried a few things but not finished them. If I do I will update this post. Things I've considered trying:
- Working from the bottom up. We know the minimum locations, what if we worked backwards from that to get an input seed range?
- Operate only on ranges. Change the seeds to ranges and apply the mappings that way.

## Thoughts

This was good. Evil but good. Everyone I know was caught off guard by the sudden explosion in size either causing insane run-times and or out-of-memory errors everywhere. If this is a sign of things to come and it's only day five, I'm a little worried 🤣.

## Results

```
day_05    fastest       │ slowest       │ median        │ mean
├─ part1  57.2 µs       │ 370.5 µs      │ 62.06 µs      │ 79.74 µs
╰─ part2 7 minutes
```