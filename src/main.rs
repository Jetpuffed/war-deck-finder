use rand::prelude::*;
use std::cmp::Ordering;
use war::*;

fn main() {
    let mut best_deck = Vec::new();
    let mut best_game = u64::MAX;

    for _ in 0..u32::MAX {
        let rng = thread_rng();
        let mut rounds = 0;

        let deck = Deck::new();
        let (_p1_deck, _p2_deck) = deck.split_deck(rng);
        let (mut p1_deck, mut p2_deck) = (_p1_deck.clone(), _p2_deck.clone());

        loop {
            if p1_deck.is_empty() {
                // println!("Player 1 has ran out of cards.");
                // println!("Player 2 wins!");
                break;
            }

            if p2_deck.is_empty() {
                // println!("Player 2 has ran out of cards.");
                // println!("Player 1 wins!");
                break;
            }

            rounds += 1;

            let (p1_card, p2_card) = (p1_deck.pop().unwrap(), p2_deck.pop().unwrap());

            match (p1_card as u8).cmp(&(p2_card as u8)) {
                Ordering::Greater => {
                    p1_deck.insert(0, p1_card);
                    p1_deck.insert(0, p2_card);
                    continue;
                }
                Ordering::Less => {
                    p2_deck.insert(0, p2_card);
                    p2_deck.insert(0, p1_card);
                    continue;
                }
                Ordering::Equal => {
                    let (mut p1_war_deck, mut p2_war_deck) = (Vec::new(), Vec::new());

                    loop {
                        if p1_deck.is_empty() {
                            break;
                        }

                        if p2_deck.is_empty() {
                            break;
                        }

                        if p1_deck.len() < 4 {
                            for _ in 0..p1_deck.len() {
                                p1_war_deck.push(p1_deck.pop().unwrap())
                            }
                        } else {
                            for _ in 0..=3 {
                                p1_war_deck.push(p1_deck.pop().unwrap());
                            }
                        }

                        if p2_deck.len() < 4 {
                            for _ in 0..p2_deck.len() {
                                p2_war_deck.push(p2_deck.pop().unwrap())
                            }
                        } else {
                            for _ in 0..3 {
                                p2_war_deck.push(p2_deck.pop().unwrap());
                            }
                        }

                        let (p1_war_card, p2_war_card) =
                            (p1_war_deck.pop().unwrap(), p2_war_deck.pop().unwrap());

                        match (p1_war_card as u8).cmp(&(p2_war_card as u8)) {
                            Ordering::Greater => {
                                for card in p1_war_deck {
                                    p1_deck.insert(0, card)
                                }
                                p1_deck.insert(0, p1_war_card);
                                for card in p2_war_deck {
                                    p1_deck.insert(0, card)
                                }
                                p1_deck.insert(0, p2_war_card);
                                break;
                            }
                            Ordering::Less => {
                                for card in p2_war_deck {
                                    p2_deck.insert(0, card)
                                }
                                p2_deck.insert(0, p2_war_card);
                                for card in p1_war_deck {
                                    p2_deck.insert(0, card)
                                }
                                p2_deck.insert(0, p1_war_card);
                                break;
                            }
                            Ordering::Equal => continue,
                        }
                    }
                }
            }
        }

        if rounds < best_game {
            {
                best_deck.clear();
                best_deck.push(_p1_deck);
                best_deck.push(_p2_deck);
            }
            best_game = rounds;
            println!("Found new best: {:#?} rounds\n", best_game);
            println!("{:?}", best_deck);
        }
    }
}
