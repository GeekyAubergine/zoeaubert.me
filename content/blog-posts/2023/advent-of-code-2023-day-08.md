---
slug: advent-of-code-2023-day-08
date: 2023-12-08T23:20
title: Advent of Code 2023 - Day 08
description: A discussion of my solution to Advent of Code 2023 - Day 08. This post contains spoilers.
tags: ['Programming', 'AdventOfCode', 'Rust']
---
[Advent of Code](https://adventofcode.com/) is a yearly programming challenge. See my [day 01 post](https://zoeaubert.me/blog/advent-of-code-2023-day-01/) to see how the project is set up.

To view my [solutions](https://github.com/GeekyAubergine/advent-of-code/tree/main/2023/day-08) in full, check them out on GitHub. See my [previous posts](https://zoeaubert.me/tags/advent-of-code/) for other solutions.

## Initial solutions

### Part 1

Today saw us taking a series of `Node`s with left and right children, and based on a series of instructions, take either the left or right children until we reach a goal node. The nodes were formatted like this:

```
AAA = (BBB, CCC)
BBB = (DDD, EEE)
```

Using this it was relatively easy to parse the `Node`s and build up a map of `Node`s.

```rust
struct Node {
    id: u32,
    left: u32,
    right: u32,
}

impl Node {
    fn new(id: u32, left: u32, right: u32) -> Self {
        Self { id, left, right }
    }

    fn from_str(input: &str) -> Result<Self> {
        let id = input
            .get(0..=2)
            .ok_or_else(|| Error::CouldNotFindIdForInstruction(input.to_string()))?;

        let left = input
            .get(7..=9)
            .ok_or_else(|| Error::CouldNotFindLeftInstruction(input.to_string()))?;

        let right = input
            .get(12..=14)
            .ok_or_else(|| Error::CouldNotFindRightInstruction(input.to_string()))?;

        Ok(Self::new(
            letters_to_id(id)?,
            letters_to_id(left)?,
            letters_to_id(right)?,
        ))
    }
}

struct Map {
    nodes: HashMap<u32, Node>,
}

impl Map {
    fn new(nodes: Vec<Node>) -> Self {
        let mut map = Self {
            nodes: HashMap::new(),
        };

        for node in nodes {
            map.nodes.insert(node.id, node);
        }

        map
    }

    fn from_str(input: &str) -> Result<Self> {
        let mut nodes = Vec::new();

        for line in input.lines() {
            nodes.push(Node::from_str(line)?);
        }

        Ok(Self::new(nodes))
    }
    
    fn get_node(&self, id: u32) -> Result<&Node> {
        self.nodes
            .get(&id)
            .ok_or_else(|| Error::CouldNotInspectionForId(id_to_letters(id)))
    }
}
```

I've opted to use a [HashMap](https://en.wikipedia.org/wiki/Hash_table) as it's the quickest way to look up `Node`s. You will notice that I've used `HashMap<u32, Node>` rather than `HashMap<String, Node>`. I don't know if this is still true but a fixed-size key is supposed to be a lot better than a String. As a result, I did some trickery.

```rust
const ZZZ_ID: u32 = 0x005A5A5A;

fn letters_to_id(letters: &str) -> Result<u32> {
    if letters.len() != 3 {
        return Err(Error::InvalidNumberOfLettersForId(letters.to_string()));
    }

    let mut id: u32 = 0;

    for (i, letter) in letters.chars().rev().enumerate() {
        id |= (letter as u32) << (i * 8);
    }

    Ok(id)
}

fn id_to_letters(id: u32) -> String {
    let mut letters = String::new();

    let letter_1 = ((id & 0x00FF0000) >> 16) as u8 as char;
    let letter_2 = ((id & 0x0000FF00) >> 8) as u8 as char;
    let letter_3 = (id & 0x000000FF) as u8 as char;

    letters.push(letter_1);
    letters.push(letter_2);
    letters.push(letter_3);

    letters
}
```

These functions convert the `Node` to and from a `u32`. Because I know that the `Node` IDs are always three-letter [ASCII](https://en.wikipedia.org/wiki/ASCII) characters, and I know that an ASCII character takes up 1 byte, I can store all three in a `u32` without much difficulty. For example, the `Node` ID `ABC` will be encoded as `0x00414243` in [Hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal). 

From here it's a simple task to follow the instructions and traverse the `Node` `Map` until we find the `Node` we want. The instructions are given as a list of `L` and `R` (for each child `Node`). If you reach the end of the instructions and have not reached your goal `Node`, you repeat the instructions as required.

```rust
pub fn process(input: &str) -> Result<u32> {
    let mut lines = input.lines().map(|l| l.trim());

    let instructions = lines.next().ok_or_else(|| Error::NoInstructionsFound)?;

    lines.next();

    let remaining = lines.collect::<Vec<_>>().join("\n");

    let map = Map::from_str(&remaining)?;

    let mut steps = 0;
    let mut current_node = map.get_node(letters_to_id("AAA")?)?;

    loop {
        for direction in instructions.chars() {
            if current_node.id == ZZZ_ID {
                return Ok(steps);
            }

            match direction {
                'L' => current_node = map.get_node(current_node.left)?,
                'R' => current_node = map.get_node(current_node.right)?,
                _ => return Err(Error::UnexpectedInstruction(direction.to_string())),
            }

            steps += 1;
        }
    }
}
```

### Part 2

Part 2 saw the fun introduction of instead of starting at the `Node` with the ID `AAA`, we're required to start at any `Node` ending with `A` and finish at any `Node` finishing with `Z`. For all the start `Node`s, we must work out what is the minimum number of steps required before all "walks" (the word I'm going to use to represent the journey from a start to end `Node` when following the instructions) are on a `Node` ending with `Z`.

Oh, boi. This really complicates things. Working out when two "walks" both hit an end `Node` at the same time is kinda hard. I think [the problem](https://adventofcode.com/2023/day/8) describes it pretty well.

```
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
```

```
Here, there are two starting nodes, `11A` and `22A` (because they both end with `A`). As you follow each left/right instruction, use that instruction to _simultaneously_ navigate away from both nodes you're currently on. Repeat this process until _all_ of the nodes you're currently on end with `Z`. (If only some of the nodes you're on end with `Z`, they act like any other node and you continue as normal.) In this example, you would proceed as follows:

- Step 0: You are at `11A` and `22A`.
- Step 1: You choose all of the _left_ paths, leading you to `11B` and `22B`.
- Step 2: You choose all of the _right_ paths, leading you to `_11Z_` and `22C`.
- Step 3: You choose all of the _left_ paths, leading you to `11B` and `_22Z_`.
- Step 4: You choose all of the _right_ paths, leading you to `_11Z_` and `22B`.
- Step 5: You choose all of the _left_ paths, leading you to `11B` and `22C`.
- Step 6: You choose all of the _right_ paths, leading you to `_11Z_` and `_22Z_`.

So, in this example, you end up entirely on nodes that end in `Z` after `_6_` steps.
```

It was at this point I suspected that cycles would be important. I was ultimately right, but not before I went down a weird path.

I noticed in this example that the path the first walk takes loops every 2 steps. But this felt like a trap. What if there was a walk that didn't directly repeat, but instead oscillated between two valid end `Node`s? Thinking that this was likely to be a gotcha, I set about solving it with this problem in mind.

I've recently been watching a bunch of videos on [ray marching](https://en.wikipedia.org/wiki/Ray_marching) and thought I could borrow the approach. Essentially, it works out the longest "safe" distance a ray can progress before it _might_ encounter an object. Using similar logic. I built a function that took a `Node`, the current input, and returned how many steps it had before it reached the next end `Node`. If I did this for all active walks, I would be able to skip forward to the next ending spot before progressing.

I thought this approach was clever. I thought I'd got one up on the inevitable gotcha. But I was wrong. After letting it run for a long time and not getting a result of use, I investigated more. I logged the length of each walk to work out their cycle lengths and noticed that even when crossing their looping point, the lengths did not change. This disproved my theory and meant I needed to rethink my approach.

Introducing the [least/lowest common multiple](https://en.wikipedia.org/wiki/Least_common_multiple). I've seen a fair bit of discussion talking about how people don't understand why it works. I'm going to give it my best shot of explaining it.

#### Why LCM works

Given two numbers, the LCM will tell you the first common multiple of two (or more) numbers. A "common multiple" is a number in which two other numbers can be multiplied by both arrive at. Let's take 2 and 3 and have a look at their multiples.

```
2: 02, 04, 06, 08, 10, 12, 14, 16, 18, 20
3: 03, 06, 09, 12, 15, 18, 21, 24, 27, 30
```

When we look at this list of multiples, we can see that `06` is the first number that appears in both lists. It is _common_ to both lists of _multiples_. And because it is first, it is the _least_ common multiple.

We can apply this to our problem using the lengths of our cycles. If we consider the first walk, it hits an "end" `Node` every n steps equal to the cycle length. Which in this case is 2. If we listed the step/iteration count at the end of each of these cycles, it would look like `02, 04, 06, ...`, which is the same as our list of multiples from before. The same will be true with the other cycle with a length of 3.

So, if each of these "multiples" is equivalent to a cycle, we need to know when both cycles have ended at the same time. Or, another way of putting, that they have a step in common. 

In this case, it's trivial, but in the actual input, the cycle lengths are:
```
[
    17621,
    13939,
    12361,
    19199,
    15517,
    20777,
]
```

If we take just the first two numbers. When would they both have a step in common? If you were to list every single multiple of `17,621` and `13, 939` below 1 million, you wouldn't find a common/shared multiple until `933,913`. And as you can imagine, this is much easier to compute than actually checking all the numbers before that.

This also gets out of hand pretty quickly. If you take the first three walks, the LCM is `43,893,911`. Adding that one number has made the search space increase by 47x. All of them together lead to a 14-digit number, which is crazy high and would take forever to check by hand. But this technique is much faster.

#### Implementing LCM

As part of my misstep earlier, I built a lot of helper functions and structures.

```rust
struct Input {
    input: String,
    cursor: usize,
}

impl Input {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            cursor: 0,
        }
    }

    fn skip(&mut self, n: usize) {
        self.cursor += n;
        self.cursor %= self.input.len();
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.cursor)
    }

    fn next(&mut self) -> Option<char> {
        let next = self.peek();
        self.cursor += 1;
        self.cursor %= self.input.len();
        next
    }
}

fn get_next_node(map: &Map, node: u32, mut input: Input) -> Result<(u32, Input)> {
    let node = map.get_node(node)?;

    match input.next() {
        Some('L') => Ok((map.get_node(node.left)?.id, input)),
        Some('R') => Ok((map.get_node(node.right)?.id, input)),
        Some(c) => Err(Error::UnexpectedInstruction(c.to_string())),
        None => Err(Error::UnexpectedEndOfInstructions),
    }
}

fn steps_to_next_ending_in_z(map: &Map, node: u32, mut input: Input) -> Result<u64> {
    let mut steps = 0;
    let mut current_node = node;

    loop {
        if id_ends_with_z(current_node) {
            return Ok(steps);
        }

        let (next_node, next_input) = get_next_node(map, current_node, input)?;

        steps += 1;

        current_node = next_node;
        input = next_input;
    }
}

fn lcm(numbers: &[u64]) -> u64 {
    let mut result = numbers[0];

    for &number in numbers.iter().skip(1) {
        result = result * number / result.gcd(number);
    }

    result
}
```

The `Input` struct is definitely over-engineered. But it did make the "repeat the instructions if not at the end until you are" part much easier to work with.

With those it was pretty easy to calculate the cycle length (distance to first "end" `Node`) and then get the LCM of them

```rust
pub fn process(input: &str) -> Result<u64> {
    let mut lines = input.lines().map(|l| l.trim());

    let instructions = lines.next().ok_or_else(|| Error::NoInstructionsFound)?;

    let input = Input::new(instructions);

    lines.next();

    let remaining = lines.collect::<Vec<_>>().join("\n");

    let map = Map::from_str(&remaining)?;

    let current_nodes = map
        .get_starting_nodes()
        .iter()
        .map(|n| n.id)
        .collect::<Vec<_>>();

    let distances_to_next_z = current_nodes
        .par_iter()
        .map(|n| steps_to_next_ending_in_z(&map, *n, input.clone()))
        .collect::<Result<Vec<_>>>()?;

    let lcm: u64 = lcm(&distances_to_next_z);

    Ok(lcm)
}
```

## Optimisation

I don't think there's anything you could optimise out here. Maybe there is though and it'd be fun to be corrected.

## Thoughts

I was not a fan of today. Not because it wasn't a good puzzle but because it required knowing and understanding how to apply a maths concept, rather than a computer science concept. I don't mind it when [Project Euler](https://projecteuler.net/) does it, because that's a maths sort of problem set, but I disliked it here. I like using programming challenges to teach and many people are (rightly) put off by the heavy maths stuff so I have to point them to other places. I hoped this would be more maths-light.

## Results

```
day_08    fastest       │ slowest       │ median        │ mean
├─ part1  351.7 µs      │ 1.896 ms      │ 381.9 µs      │ 441.9 µs
╰─ part2  1.096 ms      │ 8.691 ms      │ 1.169 ms      │ 1.355 ms 
```