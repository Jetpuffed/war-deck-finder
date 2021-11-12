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
    hearts: [Card; 13],
    clubs: [Card; 13],
    diamonds: [Card; 13],
    spades: [Card; 13],
}

impl Deck
{
    /// Creates a new card deck arranged in New Deck Order by default.
    fn new() -> Self
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
