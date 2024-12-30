---
slug: advent-of-code-2023-day-11
date: 2023-12-11T23:00
title: Advent of Code 2023 - Day 11
description: A discussion of my solution to Advent of Code 2023 - Day 11. Today was a day of mistakes. This post contains spoilers.
tags: ['Programming', 'AdventOfCode', 'Rust']
---
[Advent of Code](https://adventofcode.com/) is a yearly programming challenge. See my [day 01 post](https://zoeaubert.me/blog/advent-of-code-2023-day-01/) to see how the project is set up.

To view my [solutions](https://github.com/GeekyAubergine/advent-of-code/tree/main/2023/day-11) in full, check them out on GitHub. See my [previous posts](https://zoeaubert.me/tags/advent-of-code/) for other solutions.

## Part 1

Today saw us finding the distances between galaxies with some interesting twists. Read the [problem statement](https://adventofcode.com/2023/day/11), as it explains it pretty well. I went down many mistaken paths today trying to predict what would come, and it didn't work out 🤣.

### Mistake 1

I started with a recurring feature of these challenges, and created a`Input` struct.

```rust
struct Input {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

impl Input {
    fn new(input: &str) -> Self {
        let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();
        let width = lines[0].len();

        // Expand "empty" rows to two "empty" rows
        let rows = lines
            .iter()
            .flat_map(|row| {
                if row.chars().all(|c| c == '.') {
                    vec![row, row]
                } else {
                    vec![row]
                }
            })
            .collect::<Vec<_>>();

        let height = rows.len();

        let mut cols = vec![];

        for x in 0..width {
            let mut col = vec![];
            for y in 0..height {
                col.push(rows[y].chars().nth(x).unwrap());
            }
            cols.push(col);
        }

        let cols = cols
            .iter()
            .flat_map(|col| {
                if col.iter().all(|&c| c == '.') {
                    vec![col.clone(), col.clone()]
                } else {
                    vec![col.clone()]
                }
            })
            .collect::<Vec<_>>();

        let width = cols.len();

        let mut chars = vec![];

        for y in 0..height {
            for x in 0..width {
                chars.push(cols[x][y]);
            }
        }

        Self {
            chars,
            width,
            height,
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(self.chars[y * self.width + x]);
            }
            s.push('\n');
        }
        s
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.chars[y * self.width + x])
        }
    }
}
```

In my infinite wisdom, I chose to deal with the "empty row/column counting as two rows/columns" by actually expanding the input and adding additional characters to it. In hindsight, it was absolute madness, but in my barely awake state this morning, seemed like a genius move. Everything else in it is pretty normal.

### Mistake 2

```rust
struct Galaxy {
    id: u16,
    x: f32,
    y: f32,
}

impl Galaxy {
    fn new(id: u16, x: f32, y: f32) -> Self {
        Self { id, x, y }
    }

    fn distance(&self, other: &Self) -> f32 {
        let mut distance = 0.0;

        let mut x = self.x;
        let mut y = self.y;

        while x != other.x || y != other.y {
            let dx = other.x - x;
            let dy = other.y - y;

            if dx.abs() >= dy.abs() {
                x += dx.signum();
            } else {
                y += dy.signum();
            }

            distance += 1.0;
        }

        distance
    }
}
```

Next, I built myself a `Galaxy` struct. Here lies my second of many follies. When working out the distance between two galaxies, I first turned to what I thought was the genius move of using a [rasterisation](https://en.wikipedia.org/wiki/Rasterisation) style approach to working out what "grid" slots the shortest distance span across, and using that to work out the "grid distance". 

This initially worked for the first few tests, giving me a false sense of hope. But, later on (after building the rest of the program), I would discover it was wrong for more test cases. After abandoning that approach, I turned to what seemed like a safer bet. Work out the delta between the two points, and then walk in that direction one cell at a time in the direction of the greatest delta. And sure enough, it worked, but boi, was it slow. I couldn't see any optimisation for it, so I gave it no more thought at this point.

### Mistake 3

I chanced a glance into the future as to what part 2 might contain, and suspected some more difficult pathfinding would come, something like "shortest path between all points", or "shortest spanning tree between all points". With that in mind, I decided it would probably be useful to cache the distance between any two given galaxies.

```rust
fn galaxy_distance_hash_id(galaxy_a: u16, galaxy_b: u16) -> u32 {
    let min = galaxy_a.min(galaxy_b);
    let max = galaxy_a.max(galaxy_b);

    (min as u32) << 16 | (max as u32)
}

struct GalaxyMap {
    galaxies: HashMap<u16, Galaxy>,
    galaxy_distances: HashMap<u32, u32>,
}

impl GalaxyMap {
    fn new() -> Self {
        Self {
            galaxies: HashMap::new(),
            galaxy_distances: HashMap::new(),
        }
    }

    fn from_input(input: &Input) -> Self {
        let mut map = Self::new();

        let mut id = 1;

        for y in 0..input.height {
            for x in 0..input.width {
                if input.get(x, y) == Some('#') {
                    map.add(Galaxy::new(id, x as f32, y as f32));
                    id += 1;
                }
            }
        }

        map
    }

    fn add(&mut self, galaxy: Galaxy) {
        self.galaxies.insert(galaxy.id, galaxy);
    }

    fn distance(&mut self, a: u16, b: u16) -> u32 {
        let key = galaxy_distance_hash_id(a, b);
        if let Some(distance) = self.galaxy_distances.get(&key) {
            *distance
        } else {
            let distance = self.galaxies[&a].distance(&self.galaxies[&b]) as u32;
            self.galaxy_distances.insert(key, distance);
            distance
        }
    }

    fn galaxy_ids(&self) -> Vec<u16> {
        self.galaxies.keys().copied().collect::<Vec<_>>()
    }
}
```

From there, it was easy to process the necessary data.

```rust
pub fn process(input: &str) -> Result<u32> {
    let input = Input::new(input);

    let mut map = GalaxyMap::from_input(&input);

    let galaxy_ids = map.galaxy_ids();

    let mut total_distance = 0;

    for a in 0..galaxy_ids.len() {
        for b in a + 1..galaxy_ids.len() {
            let distance = map.distance(galaxy_ids[a], galaxy_ids[b]);
            total_distance += distance;
        }
    }

    Ok(total_distance)
}
```

Cool. I was feeling good, I felt like I knew what twist was coming and was ready for it.

I then saw part 2. Each empty row/column isn't one additional empty row/column but is actually 1 million more rows/columns. This was not a twist I foresaw, and I knew immediately that my code would not support it.

## Optimisation (aka fixing past mistakes)

I decided part 2 was impossible without fixing my previous mistakes, so I did this first, which is different to my usual approach.

### Fix 1

Growing the input to be several million rows and columns is not feasible. So that had to change.

```rust
struct Input {
    chars: Vec<char>,
    width: usize,
    height: usize,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Input {
    fn new(input: &str) -> Self {
        let lines = input.lines().map(|l| l.trim()).collect::<Vec<_>>();

        let width = lines[0].len();
        let height = lines.len();

        let chars = lines.iter().flat_map(|l| l.chars()).collect::<Vec<_>>();

        let empty_rows = (0..height)
            .filter(|y| {
                for x in 0..width {
                    if chars[*y * width + x] == '#' {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<_>>();

        let empty_cols = (0..width)
            .filter(|x| {
                for y in 0..height {
                    if chars[y * width + *x] == '#' {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<_>>();

        Self {
            chars,
            width,
            height,
            empty_rows,
            empty_cols,
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(self.chars[y * self.width + x]);
            }
            s.push('\n');
        }
        s
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.chars[y * self.width + x])
        }
    }

    fn is_row_empty(&self, y: usize) -> bool {
        self.empty_rows.contains(&y)
    }

    fn is_col_empty(&self, x: usize) -> bool {
        self.empty_cols.contains(&x)
    }
}
```

Long gone is transforming the input. The only semi-clever thing going on here is pre-computing all the empty rows/columns before I need them. This is still quite messy, or at least I think it is. There are probably a few things you could do to fix it. The empty checks feel particularly scuffed.

### Fix 2

This was more a stroke of luck than a stroke of genius. In trying to optimise the distance calculation I tried a great many things, including the Pythagorean theorem. While this turned out to be useless, I did notice that the x and y delta just happened to be the same as the distance. This greatly simplifies the calculation.

```rust
impl Galaxy {
    #[tracing::instrument]
    fn new(id: u16, x: f32, y: f32) -> Self {
        Self { id, x, y }
    }

    #[tracing::instrument]
    fn distance(&self, other: &Self) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        dx.abs() + dy.abs()
    }
}
```

I feel like with the number of grid-based games I've created over the years I'd have known this. Useful to know now though.

### Fix 3

The `GalaxyMap` was the next thing on the chopping block. As we're not doing anything clever with the input to create the additional empty rows/columns, we need to do it here. Rather than just looking at galaxies, we need to go row by row and add a running offset for each empty row. Similar for columns.

The second thing is removing the cache. We're only checking each pairing once, so we're never going to look it up.

```rust
struct GalaxyMap {
    galaxies: HashMap<u16, Galaxy>,
}

impl GalaxyMap {
    fn new() -> Self {
        Self {
            galaxies: HashMap::new(),
        }
    }

    fn from_input(input: &Input) -> Self {
        let mut map = Self::new();

        let mut id = 1;

        let mut y_offset = 0;

        for y in 0..input.height {
            if input.is_row_empty(y) {
                y_offset += 1;
            }
            let mut x_offset = 0;
            for x in 0..input.width {
                if input.is_col_empty(x) {
                    x_offset += 1;
                }
                if input.get(x, y) == Some('#') {
                    map.add(Galaxy::new(
                        id,
                        (x + x_offset) as f32,
                        (y + y_offset) as f32,
                    ));
                    id += 1;
                }
            }
        }

        map
    }

    fn add(&mut self, galaxy: Galaxy) {
        self.galaxies.insert(galaxy.id, galaxy);
    }

    fn distance(&self, a: u16, b: u16) -> u32 {
        self.galaxies[&a].distance(&self.galaxies[&b]) as u32
    }

    fn galaxy_ids(&self) -> Vec<u16> {
        self.galaxies.keys().copied().collect::<Vec<_>>()
    }
}
```

With those optimisations, I got the speed boost I needed. It went from 44ms to 730µs. I'd usually chalk that up to a genius performance increase, but I think it just highlights how bad my first attempt was 🤣.

## Part 2

After the optimisation, part 2 was easy and only required 2 changes. Changing all the values from `u32` -> `u64` and `f32` -> `f64` (possibly not needed), and changing the offsets to `999,999`.

```rust
impl GalaxyMap {
    fn from_input(input: &Input) -> Self {
        let mut map = Self::new();

        let mut id = 1;

        let mut y_offset = 0;

        for y in 0..input.height {
            if input.is_row_empty(y) {
                y_offset += 999_999;
            }
            let mut x_offset = 0;
            for x in 0..input.width {
                if input.is_col_empty(x) {
                    x_offset += 999_999;
                }
                if input.get(x, y) == Some('#') {
                    map.add(Galaxy::new(
                        id,
                        (x + x_offset) as f64,
                        (y + y_offset) as f64,
                    ));
                    id += 1;
                }
            }
        }

        map
    }
    }
```

## Thoughts

Today wasn't actually _that_ difficult, I just made it harder than it needed to be by making a litany of errors.

I've also written this in a different style than usual. Rather than presenting the correct solution and why it worked, I thought it would be more entertaining to explore the issues of my own creation and how I fixed them. Hopefully, you enjoyed it.

## Results

```
day_11        fastest       │ slowest       │ median        │ mean
├─ part1      43 ms         │ 49.85 ms      │ 44.31 ms      │ 44.43 ms
├─ part1_opt  621.2 µs      │ 1.266 ms      │ 721 µs        │ 731.8 µs
╰─ part2      612.7 µs      │ 1.052 ms      │ 718.6 µs      │ 733.2 µs
```