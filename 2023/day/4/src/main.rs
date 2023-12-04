use std::fs;

use lalrpop_util::lalrpop_mod;

mod ast;

lalrpop_mod!(pub cards);

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let cards: Vec<ast::Card> = input.lines().map(|s| cards::CardParser::new().parse(s).unwrap()).collect();

    let mut answer1 = 0;
    for card in &cards {
        let matches: u32 = card.chosen.iter().map(|c| if card.winning.contains(c) { 1 } else { 0 }).sum();
        if matches > 0 {
            answer1 += 1 << (matches - 1);
        }
    }

    println!("{}", answer1);

    let mut answer2 = 0;

    let mut winnings: Vec<u32> = Vec::new();
    winnings.resize(cards.len(), 0);

    for i in (0..cards.len()).rev() {
        let card = &cards[i];

        let matches: usize = card.chosen.iter().map(|c| if card.winning.contains(c) { 1 } else { 0 }).sum();
        winnings[i] = 1 + winnings[i+1 .. i+1+matches].iter().sum::<u32>();
        answer2 += winnings[i];
    }

    println!("{}", answer2);
}
