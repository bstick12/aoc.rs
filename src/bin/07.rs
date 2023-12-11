use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::HashMap;


fn main() -> Result<(), Box<dyn std::error::Error>>  {

    //let lines: Vec<String> = aoc2023::load_sample_data("/Users/bnolan/workspace/bstick12/aoc.rs/aoc2023/src/resources/day07_sample.txt")?;
    let lines = aoc2023::load_input_data(2023,7)?;
    println!("Answer part 1 - {}", part1(&lines, false));
    //println!("Answer part 2 - {}", part1(&lines, true));
    Ok(())

}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Hand<'a> {
    hand: &'a str,
    bid: usize,
    jokers_wild: bool,
    score: usize
}

impl Hand<'_> {

    pub fn new(hand: &str, bid: usize, jokers_wild: bool) -> Hand {
        Hand {
            hand: hand,
            bid: bid,
            jokers_wild: jokers_wild,
            score: Self::get_score(hand, jokers_wild)
        }
    }

    fn get_score(hand: &str, _jokers_wild: bool) -> usize {
        let mut hand_count:HashMap<char, usize> = HashMap::new();
        for c in hand.chars() {
            hand_count.entry(c).and_modify(|v| *v+=1).or_insert(1);
        }
        let score:Vec<usize> = hand_count.into_values().into_iter().map(|v| v.pow(2) as usize).collect();
        score.iter().sum()
    }

    fn get_order(&self) -> HashMap<char, usize> {

        let mut order: HashMap<char, usize> = vec![
            ('1', 1),
            ('2', 2),
            ('3', 3),
            ('4', 4),
            ('5', 5),
            ('6', 6),
            ('7', 7),
            ('8', 8),
            ('9', 9),
            ('T', 10),
            ('Q', 12),
            ('K', 13),
            ('A', 14)
        ]
        .into_iter()
        .collect();

        if self.jokers_wild {
            order.insert('J', 0);
        } else {
            order.insert('J', 11);
        }

        order

    }

}

impl Ord for Hand<'_> {

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.score == other.score {
            for x in self.hand.chars().zip(other.hand.chars()) {
                let (a , b) = x;
                let order = self.get_order();
                if order.get(&a).unwrap().ne(order.get(&b).unwrap()) {
                    return a.cmp(&b);
                }
            }
            Ordering::Equal
        } else {
            self.score.cmp(&other.score)
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(lines: &Vec<String>, jokers_wild: bool) -> usize {

    let mut hands: BTreeSet<Hand> = BTreeSet::new();

    for line in lines {
        let split:Vec<&str> = line.split(" ").collect();
        let hand = Hand::new(
            split.get(0).unwrap(),
            split.get(1).unwrap().parse::<usize>().unwrap(),
            jokers_wild);
            hands.insert(hand);

    }

    let mut answer: usize = 0;

    for (i, hand) in hands.iter().enumerate() {
        println!("{} {:?}", i, hand);
        answer += (i + 1) * hand.bid;
    }

    answer

}
