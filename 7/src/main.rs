use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Card {
    J = 1,
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Card::*;
        let card = match s {
            "A" => Some(A),
            "K" => Some(K),
            "Q" => Some(Q),
            "J" => Some(J),
            "T" => Some(T),
            "9" => Some(Nine),
            "8" => Some(Eight),
            "7" => Some(Seven),
            "6" => Some(Six),
            "5" => Some(Five),
            "4" => Some(Four),
            "3" => Some(Three),
            "2" => Some(Two),
            _ => None,
        }
        .and_then(|s| Some(s))
        .ok_or(ParseCardError);
        card
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Hand(Vec<Card>);

impl Hand {
    /// Get the count of each card type in this hand, adding the joker count to
    /// the most frequent card.
    fn counts(&self) -> HashMap<Card, usize> {
        let mut counts: HashMap<Card, usize> = HashMap::new();
        use Card::*;
        let cards = [Two, Three, Four, Five, Six, Seven, Eight, Nine, T, Q, K, A];
        let jokers = self.0.iter().filter(|x| **x == J).count();
        for c in cards {
            counts.insert(c.clone(), self.0.iter().filter(|x| **x == c).count());
        }
        // Update the most frequent card to add the joker count.
        let max = counts.iter().max_by_key(|x| x.1).unwrap();
        counts.insert(max.0.clone(), max.1 + jokers);
        counts
    }

    fn has_quintuplet(&self, counts: &HashMap<Card, usize>) -> bool {
        counts
            .iter()
            .fold(false, |acc, (_card, count)| (acc || *count == 5))
    }

    fn has_quadruplet(&self, counts: &HashMap<Card, usize>) -> bool {
        counts
            .iter()
            .fold(false, |acc, (_card, count)| (acc || *count == 4))
    }

    fn has_triplet(&self, counts: &HashMap<Card, usize>) -> bool {
        let three_identical = counts
            .iter()
            .fold(false, |acc, (_card, count)| (acc || *count == 3));
        three_identical
    }

    fn has_two_pairs(&self, counts: &HashMap<Card, usize>) -> bool {
        let pairs: Vec<Card> = counts.iter().fold(vec![], |mut acc, (card, count)| {
            if *count == 2 {
                acc.push(card.clone());
            }
            acc
        });
        pairs.len() == 2
    }

    // Returns true if this hand has at least one pair (2 equal cards)
    fn has_pair(&self, counts: &HashMap<Card, usize>) -> bool {
        let two_identical = counts
            .iter()
            .fold(false, |acc, (_card, count)| (acc || *count == 2));
        two_identical
    }

    fn get_strength(&self) -> Strength {
        let counts = self.counts();
        if self.has_quintuplet(&counts) {
            Strength::FiveOfAKind
        } else if self.has_quadruplet(&counts) {
            Strength::FourOfAKind
        } else if self.has_triplet(&counts) {
            if self.has_pair(&counts) {
                Strength::FullHouse
            } else {
                Strength::ThreeOfAKind
            }
        } else if self.has_two_pairs(&counts) {
            Strength::TwoPair
        } else if self.has_pair(&counts) {
            Strength::OnePair
        } else {
            Strength::HighCard
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Strength {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    cards: Hand,
    bid: u32,
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.cards.get_strength().cmp(&other.cards.get_strength());
        match ord {
            Ordering::Equal => {
                // println!("Cards have same strength:");
                // dbg!(&self.cards, &other.cards);
                // Compare cards in order from each hand
                let card_ord = self.cards.0.iter().zip(other.cards.0.iter()).fold(
                    Ordering::Equal,
                    |mut card_ord, (a, b)| {
                        if card_ord == Ordering::Equal {
                            card_ord = a.cmp(b);
                        }
                        card_ord
                    },
                );
                // dbg!(&card_ord);
                card_ord
            }
            _ => ord,
        }
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut games: Vec<Game> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|x| Game {
            cards: Hand(
                x[0].chars()
                    .map(|c| c.to_string().parse::<Card>().unwrap())
                    .collect::<Vec<Card>>(),
            ),
            bid: x[1].parse().unwrap(),
        })
        .collect();
    // dbg!(games);

    games.sort();

    let total_winnings = games.iter().enumerate().fold(0, |winnings, (idx, game)| {
        winnings + ((idx + 1) as u32) * game.bid
    });
    println!("Total winnings: {}", total_winnings);
}

mod test {
    use crate::{Card, Hand, Strength};

    #[test]
    fn test_five_of_a_kind() {
        let h = Hand(vec![Card::A, Card::A, Card::A, Card::A, Card::A]);
        let counts = h.counts();
        assert!(h.has_quintuplet(&counts));
        assert_eq!(h.get_strength(), Strength::FiveOfAKind);
        let h = Hand(vec![Card::Q, Card::A, Card::A, Card::A, Card::A]);
        let counts = h.counts();
        assert!(!h.has_quintuplet(&counts));
        assert_ne!(h.get_strength(), Strength::FiveOfAKind);
    }

    #[test]
    fn test_four_of_a_kind() {
        let h = Hand(vec![Card::A, Card::A, Card::A, Card::A, Card::K]);
        let counts = h.counts();
        assert!(h.has_quadruplet(&counts));
        assert_eq!(h.get_strength(), Strength::FourOfAKind);
        let h = Hand(vec![Card::Q, Card::A, Card::A, Card::A, Card::K]);
        let counts = h.counts();
        assert!(!h.has_quadruplet(&counts));
        assert_ne!(h.get_strength(), Strength::FourOfAKind);
    }

    #[test]
    fn test_three_of_a_kind_full_house() {
        let h = Hand(vec![Card::Nine, Card::A, Card::A, Card::A, Card::K]);
        let counts = h.counts();
        assert!(h.has_triplet(&counts));
        assert!(!h.has_pair(&counts));
        assert_ne!(h.get_strength(), Strength::FullHouse);
        assert_eq!(h.get_strength(), Strength::ThreeOfAKind);
        let h = Hand(vec![Card::Q, Card::A, Card::A, Card::A, Card::Q]);
        let counts = h.counts();
        assert!(h.has_triplet(&counts));
        assert!(h.has_pair(&counts));
        assert_ne!(h.get_strength(), Strength::ThreeOfAKind);
        assert_eq!(h.get_strength(), Strength::FullHouse);
    }

    #[test]
    fn test_two_pairs() {
        let h = Hand(vec![Card::Nine, Card::A, Card::A, Card::K, Card::K]);
        let counts = h.counts();
        assert!(h.has_two_pairs(&counts));
        assert!(h.has_pair(&counts));
        assert_eq!(h.get_strength(), Strength::TwoPair);
        assert_ne!(h.get_strength(), Strength::OnePair);

        let h = Hand(vec![Card::Nine, Card::A, Card::A, Card::Q, Card::K]);
        let counts = h.counts();
        assert!(!h.has_two_pairs(&counts));
        assert!(h.has_pair(&counts));
        assert_ne!(h.get_strength(), Strength::TwoPair);
        assert_eq!(h.get_strength(), Strength::OnePair);
    }

    #[test]
    fn test_highcard() {
        let h = Hand(vec![Card::Nine, Card::Q, Card::A, Card::K, Card::T]);
        let counts = h.counts();
        assert!(!h.has_two_pairs(&counts));
        assert!(!h.has_pair(&counts));
        assert!(!h.has_quintuplet(&counts));
        assert!(!h.has_quadruplet(&counts));
        assert!(!h.has_triplet(&counts));
        assert_eq!(h.get_strength(), Strength::HighCard);
    }

    #[test]
    fn test_jokers() {
        // One joker
        let h = Hand(vec![Card::J, Card::Q, Card::Q, Card::Q, Card::K]);
        assert_eq!(h.get_strength(), Strength::FourOfAKind);

        let h = Hand(vec![Card::J, Card::Q, Card::T, Card::Q, Card::K]);
        assert_eq!(h.get_strength(), Strength::ThreeOfAKind);

        let h = Hand(vec![Card::J, Card::Q, Card::T, Card::Q, Card::T]);
        assert_eq!(h.get_strength(), Strength::FullHouse);

        // Two jokers
        let h = Hand(vec![Card::J, Card::Q, Card::Q, Card::Q, Card::J]);
        assert_eq!(h.get_strength(), Strength::FiveOfAKind);

        let h = Hand(vec![Card::J, Card::Q, Card::K, Card::Q, Card::J]);
        assert_eq!(h.get_strength(), Strength::FourOfAKind);

        let h = Hand(vec![Card::J, Card::Q, Card::K, Card::T, Card::J]);
        assert_eq!(h.get_strength(), Strength::ThreeOfAKind);
    }
}
