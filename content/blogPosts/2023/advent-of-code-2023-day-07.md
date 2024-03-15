---
slug: advent-of-code-2023-day-07
date: 2023-12-07T14:40
title: Advent of Code 2023 - Day 07
description: A discussion of my solution to Advent of Code 2023 - Day 07. This post contains spoilers.
tags: ['Programming', 'AdventOfCode', 'Rust']
---
[Advent of Code](https://adventofcode.com/) is a yearly programming challenge. See my [day 01 post](https://zoeaubert.me/blog/advent-of-code-2023-day-01/) to see how the project is set up.

To view my [solutions](https://github.com/GeekyAubergine/advent-of-code/tree/main/2023/day-07) in full, check them out on GitHub. See my [previous posts](https://zoeaubert.me/tags/advent-of-code/) for other solutions.

## Initial solutions

### Part 1

Today saw us implementing "Camel Cards" or TotallyNotPoker™️. Though to be fair, it is a much simpler version of the game, though I see how it'd be easy to play while riding a camel 🤣.

Rather than me explaining all the rules, I suggest you read [the problem](https://adventofcode.com/2023/day/7). This is possibly the easiest to understand so far.

As soon as I saw the problem, I knew how I wanted to build it, and it's taking advantage of some of the really nice things from Rust.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_str(input: char) -> Result<Self> {
        match input {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(Error::CouldNotParseCard(input.to_string())),
        }
    }
}
```

We're into the fun stuff already. `Card` "derives" `Ord`. `Ord` is a way of representing the relative ordering of two values, essentially `>, <, =`. Using this very simple `derive`, `Card`s can now be easily compared without having to hold an internal value to compare against. The order is given ascending in the order it's defined.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &[Card]) -> Result<Self> {
        if cards.len() != 5 {
            return Err(Error::UnexpectedNumberOfCards);
        }

		// Sort asc
        let mut cards = cards.to_vec();
        cards.sort();

		// Cards 0-4 must all be the same
        if cards[0] == cards[4] {
            return Ok(HandType::FiveOfAKind);
        }

		// Cards 0-3 or 1-4 must all be the same
        if cards[0] == cards[3] || cards[1] == cards[4] {
            return Ok(HandType::FourOfAKind);
        }

        // Three of a kind and a pair
        if (cards[0] == cards[2] && cards[3] == cards[4])
            || (cards[0] == cards[1] && cards[2] == cards[4])
        {
            return Ok(HandType::FullHouse);
        }

        // Groups of threes
        if cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] {
            return Ok(HandType::ThreeOfAKind);
        }

        if (cards[0] == cards[1] && cards[2] == cards[3])
            || (cards[0] == cards[1] && cards[3] == cards[4])
            || (cards[1] == cards[2] && cards[3] == cards[4])
        {
            return Ok(HandType::TwoPair);
        }

        if cards[0] == cards[1]
            || cards[1] == cards[2]
            || cards[2] == cards[3]
            || cards[3] == cards[4]
        {
            return Ok(HandType::OnePair);
        }

        Ok(HandType::HighCard)
    }
}
```

Next was determining the `Hand` "type". Similar to last time, we get the comparison for free. The complexity is introduced here in determining what hand we actually have. 

Thanks to `Card` being `Ord` we can call sort on a vector of them and get the cards sorted. Sorting them is important as it allows us to make quick comparisons to determine the hand. For example, if `card[0]` and `card[4]` are the same, then every value between them must also be the same. Therefore, we know that all five cards must be the same, giving us five of a kind.

We can then expand this logic to cover each case. It gets more complex, but hopefully, it's followable. Most of it is taking a "window" of x number of cards and seeing if any of them meet the equality requirement. The only weird one is full-house, which has to check for three of a kind and a pair in two different ways.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

impl Hand {
    fn new(cards: [Card; 5]) -> Result<Self> {
        let hand_type = HandType::from_cards(&cards)?;
        
        Ok(Self { cards, hand_type })
    }

    fn from_str(input: &str) -> Result<Self> {
        let mut cards = [Card::Two; 5];
        for (i, card) in input.chars().enumerate() {
            cards[i] = Card::from_str(card)?;
        }
        Self::new(cards)
    }
}
```

We can then make a `Hand` containing the `Card`s and their corresponding `HandType`. We're going to want to compare `Hand`s, but you might've noticed we've not `derive`d `Ord`. That's because we need to do something a bit more clever here so we're going to implement it ourselves.

```rust
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
	    // Compare the HandType
        match self.hand_type.cmp(&other.hand_type) {
	        // If the two hands have the same HandType
            std::cmp::Ordering::Equal => {
	            // zip takes two arrays and allows you to iterate through them at the same time without having to do index stuff
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
	                // Compare the face value of the Card
                    match self_card.cmp(other_card) {
	                    // If equal, keep going
                        std::cmp::Ordering::Equal => continue,
                        // If they're different, return the result of the face value comparison
                        other => return other,
                    }
                }
                // Default to equal
                std::cmp::Ordering::Equal
            }
            // If HandType is not equal, return the HandType comparison
            other => other,
        }
    }
}

// We need PartialOrd to satisfy other constraints, but it can just use `Ord`
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
```

I've added a bunch of comments here to hopefully explain what this is doing. It might be a bit much if you've not seen Rust before. Essentially, we check if the two `Hand`s have different `HandTypes`. If they're different, return the ordering for that. If they're the same, we then compare each `Card` in the hands until it breaks the tie.

With that, we can now sort out hands.

```rust
fn order_hands(hands: &[Hand]) -> Vec<Hand> {
    let mut hands = hands.to_vec();
    hands.sort();
    hands
}
```

Next, we need to store the `HandAndBet` and be able to compare them, too.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HandAndBet {
    hand: Hand,
    bet: u32,
}

impl HandAndBet {
    fn from_str(input: &str) -> Result<Self> {
        let mut split = input.split_whitespace();

        let hand = split
            .next()
            .ok_or_else(|| Error::CouldNotParseHandAndBet(input.to_string()))?;

        let hand = Hand::from_str(hand)?;

        let bet = split
            .next()
            .ok_or_else(|| Error::CouldNotParseHandAndBet(input.to_string()))?
            .parse::<u32>()
            .map_err(Error::CouldNotParseNumber)?;

        Ok(Self { hand, bet })
    }
}

impl Ord for HandAndBet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
	    // We don't care about the value of bet so just pass through the comparison of hand
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for HandAndBet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


fn sort_hands_and_bets(hands_and_bets: &[HandAndBet]) -> Vec<HandAndBet> {
    let mut hands_and_bets = hands_and_bets.to_vec();
    hands_and_bets.sort();
    hands_and_bets
}
```

And with that, we can now sort `HandAndBet`. All that's left is then to take these sorted `HandAndBet`s, and multiply their bet by their rank and return the sum.

```rust
pub fn process(input: &str) -> miette::Result<u32> {
    let bets_and_hands = input
        .lines()
        .map(|line| HandAndBet::from_str(line.trim()))
        .collect::<Result<Vec<HandAndBet>>>()?;

    let ordered_hands_and_bets = sort_hands_and_bets(&bets_and_hands);

    let total_winnings = ordered_hands_and_bets
        .iter()
        .enumerate()
        .map(|(i, hand_and_bet)| hand_and_bet.bet * (i + 1) as u32)
        .sum::<u32>();

    Ok(total_winnings)
}
```

### Part 2

Part 2 was an obvious twist and made `J` represent a Joker rather than a Jack. Thankfully, because I over-engineered it the first time, it's not that hard to update it to work. 

First, we need to change `J` to be worth less than two.

```rust
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}
```

The only other change is to `HandType`, we need to make this account for Jokers. This is much more complex than before.

```rust
impl HandType {
    fn from_cards(cards: &[Card]) -> Result<Self> {
        if cards.len() != 5 {
            return Err(Error::UnexpectedNumberOfCards);
        }

		// Sorted ascending
        let mut cards = cards.to_vec();
        cards.sort();

        // Joker is always at start of hand

        if cards[0] == cards[4] {
            return Ok(HandType::FiveOfAKind);
        }

        let number_of_jokers = cards.iter().filter(|card| **card == Card::Jack).count();

        match number_of_jokers {
	        // I don't believe this is necessary as the previous check should've got it, but I have it here for completeness
            5 => Ok(HandType::FiveOfAKind),
            
            // If you have 4 Jokers, change all to be the only other card and get five of a kind
            4 => Ok(HandType::FiveOfAKind),
            
            3 => {
	            // If the two other cards match, you can make all the Jokers that card
                if cards[3] == cards[4] {
                    return Ok(HandType::FiveOfAKind);
                }

				// Change all the Jokers to one of the cards and get four of a kind
                Ok(HandType::FourOfAKind)
            }
            
            2 => {
	            // If the last 3 cards are all the same, you can make all the Jokers match
                if cards[2] == cards[4] {
                    return Ok(HandType::FiveOfAKind);
                }
                
				// If you have a pair, you can make four of a kind
                if (cards[2] == cards[3]) || (cards[3] == cards[4]) {
                    return Ok(HandType::FourOfAKind);
                }

				// If you have nothing, you can always match the two jokers to one of the cards for three of a kind
                Ok(HandType::ThreeOfAKind)
            }
            
            1 => {
	            // If all other cards match, you can make the Joker match to make five of a kind
                if cards[1] == cards[4] {
                    return Ok(HandType::FiveOfAKind);
                }

				// If you have three of a kind, you can use the joker to make four of a kind
                if cards[1] == cards[3] || cards[2] == cards[4] {
                    return Ok(HandType::FourOfAKind);
                }

				// If you have two pairs, you can make the joker one of them and make a full house
                if cards[1] == cards[2] && cards[3] == cards[4] {
                    return Ok(HandType::FullHouse);
                }

				// If you hav a pair, you can make three of a kind
                if cards[1] == cards[2] || cards[2] == cards[3] || cards[3] == cards[4] {
                    return Ok(HandType::ThreeOfAKind);
                }

                Ok(HandType::OnePair)
            }

			// Same as in part 1
            0 => {
                if cards[0] == cards[3] || cards[1] == cards[4] {
                    return Ok(HandType::FourOfAKind);
                }

                if (cards[0] == cards[2] && cards[3] == cards[4])
                    || (cards[0] == cards[1] && cards[2] == cards[4])
                {
                    return Ok(HandType::FullHouse);
                }

                if cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] {
                    return Ok(HandType::ThreeOfAKind);
                }

                if (cards[0] == cards[1] && cards[2] == cards[3])
                    || (cards[0] == cards[1] && cards[3] == cards[4])
                    || (cards[1] == cards[2] && cards[3] == cards[4])
                {
                    return Ok(HandType::TwoPair);
                }

                if cards[0] == cards[1]
                    || cards[1] == cards[2]
                    || cards[2] == cards[3]
                    || cards[3] == cards[4]
                {
                    return Ok(HandType::OnePair);
                }

                Ok(HandType::HighCard)
            }
            _ => Err(Error::UnexpectedNumberOfCards),
        }
    }
}
```

This is kinda similar to the previous implementation, but the `Card` lookup has to first consider how many Jokers there are so it can use them. Again, thanks to it being sorted we can make a lot of assumptions that simplify things immensely.

No other changes were necessary to make this work. Over-engineering wins again.

## Optimisation

There are no obvious optimisations that I think would save any significant time. No matter what you do you still need to rank every hand which requires sorting. And I'm not doing anything too crazy with the cards -> hand-type mapping.

I did try multi-threading for part 1, but unsurprisingly it made things slower.

## Thoughts

This was cool and didn't have any major complications. I've always meant to build a poker game thing, so this was a good practice and proof of concept.

## Results

```
day_07        fastest       │ slowest       │ median        │ mean
├─ part1      154.9 µs      │ 251.2 µs      │ 169.4 µs      │ 172.7 µs 
├─ part1_opt  134.7 µs      │ 578.2 µs      │ 211.4 µs      │ 236.7 µs
╰─ part2      173.6 µs      │ 207.4 µs      │ 174.7 µs      │ 178.3 µs 
```