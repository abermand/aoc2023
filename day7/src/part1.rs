use std::str::FromStr;
use core::cmp::Ordering;
use itertools::Itertools;


#[derive(Debug, Hash, PartialOrd, Ord, Clone, Copy, Eq, PartialEq)]
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


impl FromStr for Card {
    type Err = String;

    fn from_str(input: &str) -> Result<Card, Self::Err> {
        match input {
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "J" => Ok(Card::Jack),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            x      => Err(format!("Could not parse card {}", x)),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    points: u32,
}

#[derive(Debug, PartialOrd, Ord, Clone, Copy, Eq, PartialEq)]
enum  HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Hand {
    fn get_type(self: &Hand) -> HandType {
        // println!("get_type for {:?}", self.cards);
        let cards : Vec<usize> = self.cards.iter()
            .counts()
            .iter()
            // .inspect(|x| println!("counts {:?}", x))
            .map(|(_card, count)| *count)
            .sorted()
            // .inspect(|x| println!("sorted {:?}", x))
            .collect();        
        let len = cards.len(); 
        match len {
            1 => HandType::FiveKind,
            2 => if cards[0] == 1 {
                    HandType::FourKind
                } else {
                    HandType::FullHouse
                },
            3 => if cards[1] == 1 {
                HandType::ThreeKind
            } else {
                HandType::TwoPair
            },
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let c1 = self.cards.iter();
        let c2 = other.cards.iter();
        c1.zip(c2).all(|(x,y)| x == y)

    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {        
        let t1 = self.get_type();
        let t2 = other.get_type();
        println!("cmp: s={:?} c={:?} t1={:?} t2={:?}", &self.cards, &other.cards, t1, t2);
        if t1 != t2 {
            return t1.cmp(&t2);
        }
        let h1 = self.cards.iter();
        let h2 = other.cards.iter();

        for (c1, c2) in h1.zip(h2) {
            println!("\t...comparing {:?} to {:?}: {:?}", c1, c2, c1.cmp(&c2));
            if c1 != c2 {
                return c1.cmp(&c2);
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("./sample2.txt");
    let output = do_the_job(input);
    dbg!(output);
}


fn parse_input(input: &str) -> Vec<Hand> {
    input.lines().map(|line| {
        let (c, points) = line.split_once(" ").unwrap();
        println!("Hand '{}' points: {}", &c, &points);
        let cards : Vec<Card> = c.chars()
            // .inspect(|c| println!("Char '{}'", c))
            .map(|c| Card::from_str(&c.to_string()).unwrap())
            .collect();
        let points = points.parse::<u32>().unwrap();
        Hand{cards, points}
    }).collect()
}

fn do_the_job(input: &str) -> u64 {
    let hands = parse_input(&input);
    let hands = hands.iter()
        .inspect(|h| println!("Init {:>8} {:?} -> {:?}", h.points, h.cards, h.get_type()))
        .sorted()
        .inspect(|h| println!("Sorted {:>8} {:?} -> {:?}", h.points, h.cards, h.get_type()));
    let score : u64 = hands
        .enumerate()
        .map(|(i, h)| (i as u64 +1) * h.points as u64)
        .sum();
    score
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./sample1.txt");
        let result = do_the_job(input);
        assert_eq!(result, 6440);
    }

    //    #[rstest]
    //   #[case("Game 95: 1 blue, 11 red; 15 red, 1 blue, 3 green; 13 red, 2 blue, 3 green; 1 green, 1 blue", false)]
    //#[case("Game 98: 8 red, 12 green, 2 blue; 7 green, 8 red, 1 blue; 2 blue, 6 red, 3 green; 9 red, 1 blue, 4 green", true)]
    //#[case("Game 5: 17 red, 5 blue, 3 green; 8 green, 9 red, 10 blue; 2 green, 9 blue, 4 red", false)]
    //fn is_game_possible(#[case] line: &str, #[case] expected: bool) {
    //let game = Game::parse_line(line);
    //assert_eq!(game.is_possible(), expected);
    //}
}
