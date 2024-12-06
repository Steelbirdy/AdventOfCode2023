use prse::parse;

type Input = (Vec<Card>, u64);

#[aoc_generator(day7)]
pub fn generate(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|s| {
            let (hand, bid): (String, u64) = parse!(s, "{} {}");
            let hand: Vec<_> = hand.chars().map(|ch| parse_card(ch)).collect();
            (hand, bid)
        })
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Card {
    Num(u64),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn key1(&self) -> u8 {
        match self {
            Self::Num(x) => *x as _,
            Self::T => 10,
            Self::J => 11,
            Self::Q => 12,
            Self::K => 13,
            Self::A => 14,
        }
    }

    fn key2(&self) -> u8 {
        match self {
            Self::J => 0,
            Self::Num(x) => *x as _,
            Self::T => 10,
            Self::Q => 11,
            Self::K => 12,
            Self::A => 13,
        }
    }

    fn cmp1(&self, other: &Self) -> std::cmp::Ordering {
        self.key1().cmp(&other.key1())
    }

    fn cmp2(&self, other: &Self) -> std::cmp::Ordering {
        self.key2().cmp(&other.key2())
    }
}

fn parse_card(ch: char) -> Card {
    use Card::*;
    match ch {
        'T' => T,
        'J' => J,
        'Q' => Q,
        'K' => K,
        'A' => A,
        _ => Num((ch as u8 - b'0') as u64),
    }
}

fn classify(hand: &[Card]) -> Kind {
    use Kind::*;
    if hand[0] == hand[4] {
        FiveOfAKind
    } else if hand[0] == hand[3] || hand[1] == hand[4] {
        FourOfAKind
    } else if (hand[0] == hand[2] && hand[3] == hand[4])
        || (hand[0] == hand[1] && hand[2] == hand[4])
    {
        FullHouse
    } else if hand[0] == hand[2] || hand[1] == hand[3] || hand[2] == hand[4] {
        ThreeOfAKind
    } else if (hand[0] == hand[1] && (hand[2] == hand[3] || hand[3] == hand[4]))
        || (hand[1] == hand[2] && hand[3] == hand[4])
    {
        TwoPair
    } else if hand[0] == hand[1] || hand[1] == hand[2] || hand[2] == hand[3] || hand[3] == hand[4] {
        OnePair
    } else {
        HighCard
    }
}

fn upgrade_with_jokers(hand: &[Card], kind: Kind) -> Kind {
    use Kind::*;
    if kind == FiveOfAKind {
        return FiveOfAKind;
    }
    let n = hand.iter().filter(|&&c| c == Card::J).count();
    let hand: Vec<_> = hand.iter().filter(|&&c| c != Card::J).collect();
    if n >= 4 {
        FiveOfAKind
    } else if n == 3 {
        if hand[0] == hand[1] {
            FiveOfAKind
        } else {
            FourOfAKind
        }
    } else if n == 2 {
        if hand[0] == hand[2] {
            FiveOfAKind
        } else if hand[0] == hand[1] || hand[1] == hand[2] {
            FourOfAKind
        } else {
            ThreeOfAKind
        }
    } else if n == 1 {
        if kind == FourOfAKind {
            FiveOfAKind
        } else if kind == ThreeOfAKind {
            FourOfAKind
        } else if kind == TwoPair {
            FullHouse
        } else if kind == OnePair {
            ThreeOfAKind
        } else {
            OnePair
        }
    } else {
        kind
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &[Input]) -> u64 {
    let mut classified: Vec<_> = input
        .iter()
        .map(|(hand, bid)| {
            let mut sorted = hand.clone();
            sorted.sort_by(Card::cmp1);
            (classify(&sorted), *bid, hand)
        })
        .collect();
    classified.sort_by(|x, y| {
        x.0.cmp(&y.0).then_with(|| {
            let x_keys = x.2.iter().map(Card::key1);
            let y_keys = y.2.iter().map(Card::key1);
            x_keys.cmp(y_keys)
        })
    });
    classified
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid, _))| (i as u64 + 1) * bid)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[Input]) -> u64 {
    let mut classified: Vec<_> = input
        .iter()
        .map(|(hand, bid)| {
            let mut sorted = hand.clone();
            sorted.sort_by(Card::cmp2);
            (upgrade_with_jokers(&sorted, classify(&sorted)), *bid, hand)
        })
        .collect();
    classified.sort_by(|x, y| {
        x.0.cmp(&y.0).then_with(|| {
            let x_keys = x.2.iter().map(Card::key2);
            let y_keys = y.2.iter().map(Card::key2);
            x_keys.cmp(y_keys)
        })
    });
    classified
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid, _))| (i as u64 + 1) * bid)
        .sum()
}
