use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum Card
{
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

pub struct Deck
{
    pub hearts: [Card; 13],
    pub clubs: [Card; 13],
    pub diamonds: [Card; 13],
    pub spades: [Card; 13],
}

impl Default for Deck
{
    /// Constructs all suits with their default arrangement.
    fn default() -> Self
    {
        Self
        {
            hearts: [Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five, Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten, Card::Jack, Card::Queen, Card::King],
            clubs: [Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five, Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten, Card::Jack, Card::Queen, Card::King],
            diamonds: [Card::King, Card::Queen, Card::Jack, Card::Ten, Card::Nine, Card::Eight, Card::Seven, Card::Six, Card::Five, Card::Four, Card::Three, Card::Two, Card::Ace],
            spades: [Card::King, Card::Queen, Card::Jack, Card::Ten, Card::Nine, Card::Eight, Card::Seven, Card::Six, Card::Five, Card::Four, Card::Three, Card::Two, Card::Ace],
        }
    }
}

impl Deck
{
    pub fn new() -> Self
    {
        Deck::default()
    }

    pub fn as_vec(&self) -> Vec<Card>
    {
        let mut tmp = Vec::with_capacity(52);

        tmp.append(&mut self.hearts.to_vec());
        tmp.append(&mut self.clubs.to_vec());
        tmp.append(&mut self.diamonds.to_vec());
        tmp.append(&mut self.spades.to_vec());
        tmp.reverse();

        tmp
    }

    pub fn as_shuffled(&self, mut rng: ThreadRng) -> Vec<Card>
    {
        let mut deck = self.as_vec();
        deck.shuffle(&mut rng);

        deck
    }

    pub fn split_deck(&self, rng: ThreadRng) -> (Vec<Card>, Vec<Card>)
    {
        let deck = self.as_shuffled(rng);
        let (a, b) = deck.split_at(26);

        (a.to_vec(), b.to_vec())
    }
}
