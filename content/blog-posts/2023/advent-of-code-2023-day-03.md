---
slug: advent-of-code-2023-day-03
date: 2023-12-03T14:55
title: Advent of Code 2023 - Day 03
description: A discussion of my solution to Advent of Code 2023 - Day 03. This post contains spoilers. I also have a test case that might help you if you're stuck like I was.
tags: ['Programming', 'AdventOfCode', 'Rust']
---
[Advent of Code](https://adventofcode.com/) is a yearly programming challenge. See my [day 01 post](https://zoeaubert.me/blog/advent-of-code-2023-day-01/) to see how the project is set up.

To view my [solutions](https://github.com/GeekyAubergine/advent-of-code/tree/main/2023/day-03) in full, check them out on GitHub. See my [previous posts](https://zoeaubert.me/tags/advent-of-code/) for other solutions.

> Updated 07:40 2023-12-04 with part 1 optimisation

## Initial solutions

### Part 1

Today saw us finding numbers adjacent to symbols and adding them up. Sounds pretty simple on paper, but this caused me some headaches. 

My first headache came from trying to be clever too soon, and instead of doing "numbers adjacent to symbols", I did "symbols adjacent to numbers". This led to the rather funny issue where if two symbols touched the same number (for example, `.#545%.` ) it would get counted twice. It's not that big of an issue, and it was caught by the sample data.

Then, the real fun began. I was stuck for a while with a solution that was wrong. Clearly, there was some bug not caught by the sample data but present in the larger data. I tried eyeballing it for a while, but it was too large to spot. So I ended up building a frankly ridiculous test.

```rust
fn it_should_extract_part_numbers_adjacent_to_symbol() -> miette::Result<()> {
        let input = include_str!("../input1.txt");

        let part_numbers = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| extract_part_numbers_from_line(line.trim(), i as u32))
            .collect::<Vec<_>>();

        let symbols = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| extract_symbols_from_line(line.trim(), i as u32))
            .collect::<Vec<_>>();

        let parts_next_to_symbols = part_numbers_adaject_to_a_symbol(&part_numbers, &symbols);

        let expect_part_numbers = vec![
            155, 944, 622, 31, 264, 532, 254, 528, //line 1
            111, 495, 558, //line 2
            791, 62, 618, 818, 642, 789, //line 3
            58, 405, 542, 587, 198, 846, 647, // line 4
            964, 474, 302, 786, 43, 505, 436, 51, //line 5
            832, 951, 984, 111, 198, 322, 186, 262, //line 6
            490, 690, 346, 702, 566, 192, 190, 87, //line 7
            816, 588, 152, 535, 425, 53, //line 8
            36, 290, 831, 374, 579, 536, 733, 169, 146, 179, 658, 260, // line 9
            795, 776, 790, 871, 281, // line 10
            78, 716, 400, 319, 167, 399, 599, // line 11
            719, 376, 800, 211, 478, 326, 93, 889, 684, 285, // line 12
            852, 462, 374, 603, 369, // line 13
            960, 966, 321, 925, 926, 947, // line 14
            479, 909, 339, 17, 284, 657, 587, // line 15,
            772, 345, 93, 465, 419, 676, 521, 399, 662, // line 16
            17, 2, 531, 79, 589, 198, 734, 534, 614, // line 17
            301, 321, 895, 344, 694, 717, 511, // line 18
            707, 370, 428, 509, 889, 353, // line 19
            973, 877, 855, 955, 670, 682, 150, 958, 197, 555, // line 20
            504, 352, 468, 688, 10, 306, // line 21
            987, 5, 811, 705, 462, 374, 42, // line 22
            402, 804, 295, 406, 150, 22, 429, 268, 324, // line 23
            270, 982, 644, 87, 505, //  line 24
            98, 370, 19, 867, 396, 272, 760, // line 25
            593, 793, 503, 34, 406, 456, 303, 142, 432, // line 26
            707, 563, 837, 230, 169, 138, 420, // line 27
            689, 503, 449, 39, 77, 404, // line 28
            137, 624, 883, 891, 310, 404, // line 29
            287, 961, 488, 544, 130, 531, 72, 424, 766, // line 30
            476, 722, 780, 613, 533, 96, 553, 91, 835, 690, // line 31
            350, 950, 359, 141, 326, 658, 832, // line 32
            772, 127, 335, 539, 101, 959, 221, 512, // line 33
            798, 138, 207, 999, 574, 484, 364, // line 34
            919, 202, 971, 488, 349, 404, 448, // line 35
            246, 211, 426, 206, 557, 27, 659, 588, 367, 961, 583, 280, // line 36
            724, 324, 788, 685, 788, 532, 85, 139, 75, 196, // line 37
            521, 391, 987, 810, 214, // line 38
            679, 776, 447, 457, 25, 467, 173, 241, // line 39
            43, 898, 412, 742, 540, 825, 259, 997, 514, // line 40
            775, 52, 809, 871, 384, 295, 470, 114, // line 41
            147, 69, 914, 144, 875, 278, 441, 859, 346, 281, 40, // line 42
            89, 578, 519, 676, 473, 361, // line 43
            78, 42, 750, 465, 218, 833, 137, 538, 962, 421, 502, 42, // line 44
            457, 825, 26, 238, 205, 539, 109, 348, 837, 842, // line 45
            175, 925, 399, 560, 636, // line 46
            693, 447, 137, 679, 479, 619, 283, 458, 544, 802, 848, // line 47
            39, 141, // line 48
            471, 502, 663, 986, 633, 530, 598, 220, 542, 568, 219, 532, 15,  // line 49
            840, // line 50
            351, 993, 573, 865, 848, 239, 134, 64, 231, // line 51
            809, 925, 43, 277, 571, // line 52
            698, 355, 55, 847, 409, 78, 363, // line 53
            261, 591, 695, 678, 714, 364, 804, 156, 605, // line 54
            192, 957, 963, 447, 344, // line 55
            524, 568, 691, 169, 218, 10, 10, 399, 46, 488, 491, 16, // line 56
            824, 772, 265, 964, // line 57
            345, 161, 671, 414, 726, 564, // line 58
            155, 483, 546, 968, 591, // line 59
            806, 120, 813, 481, 593, 667, 815, 682, 579, 298, 668, 188, // line 60
            718, 469, 251, 52, 919, 846, 887, 637, // line 61
            81, 51, 236, 167, 338, 963, 258, 980, 816, // line 62
            150, 316, 389, 590, 291, 143, 284, // line 63
            390, 559, 116, 926, 779, // line 64
            500, 821, 594, 220, 830, 89, 915, 363, // line 65
            623, 337, 40, 827, 828, 294, 392, // line 66
            993, 565, 638, 307, 95, 535, 105, 632, 938, 116, 939, // line 67
            444, 378, 283, 971, 689, 937, 736,
            991, // line 68

                 // 608, 362, 642, 262, 617, // line 140
        ];

        assert_eq!(expect_part_numbers, parts_next_to_symbols);
        Ok(())
    }
```

Is this insane? Yes. But I was convinced I'd got the core logic right, and there was something funky in the input I wasn't expecting. So, I went line by line of the input and compared it after each save. 

Eventually, on line 67 of the input I found my bug and was able to build a test case around it. 

```rust
    fn it_should_parse_lines_66_to_68() -> miette::Result<()> {
        let input = "
        ...*...623....337.......................40..........827..............*................828....$294....392....*....*.....%..............*.....
        .993............*....565........................638...............307.............95.......#..............535.105.........632..938.166..$939
        .....$..444@...378...*.......4...283...971@.......*...................689..937...*.......736......@...................991..@....*...........";

        let part_numbers = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| extract_part_numbers_from_line(line.trim(), i as u32))
            .collect::<Vec<_>>();

        let expected_part_numbers: Vec<u32> = vec![
            623, 337, 40, 827, 828, 294, 392, // line 66
            993, 565, 638, 307, 95, 535, 105, 632, 938, 166, 939, // line 67
            444, 378, 4, 283, 971, 689, 937, 736, 991, // line 68
        ];

        assert_eq!(expected_part_numbers, part_numbers.iter().map(|p| p.number).collect::<Vec<_>>());

        let symbols = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| extract_symbols_from_line(line.trim(), i as u32))
            .collect::<Vec<_>>();

        let parts_next_to_symbols = part_numbers_adaject_to_a_symbol(&part_numbers, &symbols);

        // There's some numbers missing from 66 and 68, that's because this is a slice so the row above doesn't trigger them
        let expect_part_numbers = vec![
            337, 294, // line 66
            993, 565, 638, 307, 95, 535, 105, 632, 938, 166, 939, // line 67
            444, 378, 971, 736 // line 68
        ];

        assert_eq!(expect_part_numbers, parts_next_to_symbols);
        Ok(())
    }
```

It was a flaw in my number parsing. Line 67 is the first time a number appears at the end of the line without a non-digit following it. And my parser didn't account for that. It was annoying, but at least I was justified by the insanity that is that test.

Now that it's working, I can talk about my solution. Again, I went for reliability over performance (mostly so I can feel good about my later optimisations 😉). First were some simple `struct`s and constructors:

```rust
struct PartNumber {
    x: u32,
    y: u32,
    width: u32,
    number: u32,
}

impl PartNumber {
    fn new(x: u32, y: u32, width: u32, number: u32) -> Self {
        Self {
            x,
            y,
            width,
            number,
        }
    }
}

struct Symbol {
    x: u32,
    y: u32,
    symbol: char,
}

impl Symbol {
    fn new(x: u32, y: u32, symbol: char) -> Self {
        Self { x, y, symbol }
    }

    fn postition_equals(&self, x: u32, y: u32) -> bool {
        self.x == x && self.y == y
    }
}
```

Then it was a simple case of parsing each line and extracting the part numbers and symbols.

```rust
fn extract_part_numbers_from_line(line: &str, line_index: u32) -> Vec<PartNumber> {
    let mut part_numbers = Vec::new();

    let mut in_digits = false;
    let mut number_start = 0;

    for (i, c) in line.char_indices() {
        if c.is_ascii_digit() {
            if !in_digits {
                in_digits = true;
                number_start = i;
            }
        } else if in_digits {
            in_digits = false;
            let number = line.get(number_start..i).unwrap().parse::<u32>().unwrap();
            part_numbers.push(PartNumber::new(
                number_start as u32,
                line_index,
                i as u32 - number_start as u32,
                number,
            ));
        }
    }

	// This is where my bug was, I didn't have this block so a trailing number would be lost
    if in_digits {
        let number = line.get(number_start..).unwrap().parse::<u32>().unwrap();
        part_numbers.push(PartNumber::new(
            number_start as u32,
            line_index,
            line.len() as u32 - number_start as u32,
            number,
        ));
    }

    part_numbers
}

#[tracing::instrument]
fn extract_symbols_from_line(line: &str, line_index: u32) -> Vec<Symbol> {
    let mut symbols = Vec::new();

    for (i, c) in line.char_indices() {
        if !c.is_ascii_digit() && c != '.' {
            symbols.push(Symbol::new(i as u32, line_index, c));
        }
    }

    symbols
}
```

Ok, cool, this works, now for the complex bit. Finding parts with an adjacent symbol.

```rust
impl PartNumber {
    fn has_adjacent_symbol(&self, symbol: &[Symbol]) -> bool {
        let start_x = if self.x == 0 { 0 } else { self.x - 1 }; // Rust doesn't have ternary operators, you do this instead
        let end_x = self.x + self.width + 1;
        let start_y = if self.y == 0 { 0 } else { self.y - 1 };
        let end_y = self.y + 1;

        for x in start_x..end_x {
            for y in start_y..=end_y {
                if symbol.iter().any(|s| s.postition_equals(x, y)) {
                    return true;
                }
            }
        }

        false
    }
}
```

Slightly cursed, but it works. I loop in a box around the part number and return true as soon as we see a symbol. Using this function, it's easy to find part numbers with an adjacent symbol, and then do the summation.

```rust
fn part_numbers_adaject_to_a_symbol(part_numbers: &[PartNumber], symbols: &[Symbol]) -> Vec<u32> {
    part_numbers
        .iter()
        .filter(|part_number| part_number.has_adjacent_symbol(symbols))
        .map(|part_number| part_number.number)
        .collect::<Vec<_>>()
}

pub fn process(input: &str) -> miette::Result<u32> {
    let part_numbers = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| extract_part_numbers_from_line(line.trim(), i as u32))
        .collect::<Vec<_>>();

    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| extract_symbols_from_line(line.trim(), i as u32))
        .collect::<Vec<_>>();

    let parts_next_to_symbols = part_numbers_adaject_to_a_symbol(&part_numbers, &symbols);

    let sum = parts_next_to_symbols.iter().sum::<u32>();

    Ok(sum)
}
```

### Part 2

Part two, funnily enough, saw us finding pairs of numbers surrounding the `*` symbol, which is very close to what I was originally doing. The changes to make this work were fairly minimal. First was making parts searchable and symbols able to search for them.

```rust
impl PartNumber {
    fn contains_point(&self, x: i32, y: i32) -> bool {
        let start_x = self.x;
        let end_x = self.x + self.width;

        x >= start_x && x < end_x && y == self.y
    }
}

impl Symbol {
    fn adjacent_part_numbers(&self, part_numbers: &[PartNumber]) -> Vec<i32> {
        part_numbers
            .iter()
            .filter(|part_number| {
                part_number.contains_point(self.x - 1, self.y) // left
                    || part_number.contains_point(self.x + 1, self.y) // right
                    || part_number.contains_point(self.x, self.y - 1) // top
                    || part_number.contains_point(self.x, self.y + 1) // bottom
                    || part_number.contains_point(self.x - 1, self.y - 1) // top left
                    || part_number.contains_point(self.x + 1, self.y - 1) // top right
                    || part_number.contains_point(self.x - 1, self.y + 1) // bottom left
                    || part_number.contains_point(self.x + 1, self.y + 1) // bottom right
            })
            .map(|part_number| part_number.number)
            .collect::<Vec<_>>()
    }
}
```

There was a small change to symbol parsing, mostly to skip anything that isn't `*`.

```rust
fn extract_symbols_from_line(line: &str, line_index: i32) -> Vec<Symbol> {
    let mut symbols = Vec::new();

    for (i, c) in line.char_indices() {
        if c == '*' {
            symbols.push(Symbol::new(i as i32, line_index, c));
        }
    }

    symbols
}
```

It was then again a case of just throwing these functions together to get them to work. This is kinda iterator-heavy, but hopefully, it's fine to follow.

```rust
fn symbols_with_2_adjacent_part_numbers(
    symbols: &[Symbol],
    part_numbers: &[PartNumber],
) -> Vec<i32> {
    symbols
        .iter()
        .map(|symbol| symbol.adjacent_part_numbers(part_numbers))
        .filter(|adjacent_part_numbers| adjacent_part_numbers.len() == 2)
        .map(|adjacent_part_numbers| adjacent_part_numbers.iter().product())
        .collect::<Vec<_>>()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32> {
    let part_numbers = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| extract_part_numbers_from_line(line.trim(), i as i32))
        .collect::<Vec<_>>();

    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| extract_symbols_from_line(line.trim(), i as i32))
        .collect::<Vec<_>>();

    let gear_ratios = symbols_with_2_adjacent_part_numbers(&symbols, &part_numbers);

    let sum = gear_ratios.iter().sum::<i32>();

    Ok(sum)
}
```

This felt much easier than part 1, but I think that's because I didn't spend an hour doing manual data entry to find my bug.

## Optimisation

### Part1

After a break, I decided to give optimisation another go.

### Part2

I have chosen not to optimise part 2 as I don't see a nice way to do it. I have some guesses, but they all look quite ugly.

```rust
struct Data {
    symbol_map: Vec<bool>,
    width: usize,
}

impl Data {
    #[tracing::instrument]
    fn new(input: &String) -> Self {
        let is_symbol = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| is_symbol(Some(c)))
            .collect::<Vec<_>>();
        let width = input.lines().next().unwrap().len();

        Self { width, symbol_map }
    }

    #[tracing::instrument]
    fn is_symbol(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }

        match self.symbol_map.get(y as usize * self.width + x as usize) {
            Some(v) => *v,
            None => false,
        }
    }
}
```

This time, rather than accessing the strings as needed, I precompute whether or not a character is a symbol for the entire input. This may seem expensive, and I tried some other methods, but this came out the fastest. Doing this allows for fast lookups when processing the numbers.

A small but non-insignificant change was the way I was calculating if a character is a symbol.

```rust
fn is_symbol(char: Option<char>) -> bool {
    match char {
        Some(c) => {
            matches!(c, '-' | '%' | '+' | '=' | '*' | '/' | '$' | '#' | '&' | '@')
        }
        None => false,
    }
}
```

Rather than checking for any symbol, it only checks the ones present in the input. And thank you to clippy for suggesting the `matches!`, it's very cool (it expands to a match statement returning true for the items listed and false for everything else).

Processing each line has become more complex. While the basic outline and operations remain unchanged, there's some extra fun.

```rust
fn parse_line(line: &str, y: i32, data: &Data) -> Vec<u32> {
    let mut in_number = false;
    let mut number_start = 0;
    let mut adjacent_symbol = false;

    let mut numbers = vec![];

    for (i, c) in line.chars().enumerate() {
        let i_as_i32 = i as i32;
        if c.is_ascii_digit() {
            if !in_number {
                in_number = true;
                number_start = i;

                // Previous
                if data.is_symbol(i_as_i32 - 1, y)
                    || data.is_symbol(i_as_i32 - 1, y - 1)
                    || data.is_symbol(i_as_i32 - 1, y + 1)
                {
                    adjacent_symbol = true;
                }
            }

            // Above below
            if (data.is_symbol(i_as_i32, y - 1)) || (data.is_symbol(i_as_i32, y + 1)) {
                adjacent_symbol = true;
            }
        } else if in_number {
            // Check self, above and below
            if data.is_symbol(i_as_i32, y)
                || data.is_symbol(i_as_i32, y - 1)
                || data.is_symbol(i_as_i32, y + 1)
            {
                adjacent_symbol = true;
            }

            if adjacent_symbol {
                numbers.push(line[number_start..i].parse().unwrap());
            }

            in_number = false;
            adjacent_symbol = false;
        }
    }

    if in_number
        && (adjacent_symbol
            || data.is_symbol(line.len() as i32 - 1, y - 1)
            || data.is_symbol(line.len() as i32 - 1, y + 1))
    {
        numbers.push(line[number_start..].parse().unwrap());
    }

    numbers
}
```

After finding the first digit like we did before, we then immediately look left and diagonally left to see if we can find a symbol.

```rust
if data.is_symbol(i_as_i32 - 1, y)
	|| data.is_symbol(i_as_i32 - 1, y - 1)
	|| data.is_symbol(i_as_i32 - 1, y + 1)
{
	adjacent_symbol = true;
}
```

Then, for every digit following we do similar, and check above and below the current character.

```rust
if (data.is_symbol(i_as_i32, y - 1)) || (data.is_symbol(i_as_i32, y + 1)) {
	adjacent_symbol = true;
}
```

Then, once we've reached the end of the number, we have to check right, and diagonally right. But, we only know if we've reached the end of the number if we've seen a non-number character, so what we really need to do is check the current character, and above and below that, which has the same effect as checking right and diagonally because we're already one character to the right.

```rust
if data.is_symbol(i_as_i32, y)
	|| data.is_symbol(i_as_i32, y - 1)
	|| data.is_symbol(i_as_i32, y + 1)
{
	adjacent_symbol = true;
}
```

The only thing to do then is not to fall prey to my earlier bug and also check if we've reached the end of the line but are still in a number. Again, here, we have to check above and below on the final character for a symbol.

```rust
if in_number
	&& (adjacent_symbol
		|| data.is_symbol(line.len() as i32 - 1, y - 1)
		|| data.is_symbol(line.len() as i32 - 1, y + 1))
{
	numbers.push(line[number_start..].parse().unwrap());
}
```

Combing it all together is then a simple case of:

```rust
pub fn process(input: &str) -> miette::Result<u32> {
    let input = input
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n");

    let data = Data::new(&input);

    let sum = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| parse_line(line, y as i32, &data))
        .sum::<u32>();

    Ok(sum)
}
```

Using these techniques has yielded a rather silly performance increase of 59x, taking us from the ~7ms area to ~120µs.

Overall, I think this is a fairly intuitive optimisation. But I am very pleased with the results. I'm also glad I did it the more over-engineered way first as I would likely have had an even worse time debugging this approach to find my bug.

## Additional Notes

This has been a lesson in not trusting their example input to cover all possible gotchas, I know there was one present in [day-01](https://zoeaubert.me/blog/advent-of-code-2023-day-01/) but I managed to avoid it, and in my hubris assumed it couldn't happen to me 🤣

## Results

```
day_03        fastest       │ slowest       │ median        │ mean
├─ part1      7.107 ms      │ 15.85 ms      │ 7.342 ms      │ 7.564 ms
├─ part1_opt  123.7 µs      │ 150.4 µs      │ 124.1 µs      │ 126.7 µs 
╰─ part2      6.111 ms      │ 6.834 ms      │ 6.355 ms      │ 6.358 ms
```