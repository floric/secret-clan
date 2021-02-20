use super::proto::{self};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Card {
    color: CardColor,
    value: u32,
}

impl Card {
    pub fn new(color: CardColor, value: u32) -> Self {
        if value > 13 {
            panic!("Invalid card created");
        }

        Card { color, value }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum CardColor {
    Hearts,
    Tiles,
    Clovers,
    Pikes,
}

impl From<proto::card::Card> for Card {
    fn from(proto_card: proto::card::Card) -> Self {
        Card {
            color: match proto_card.color {
                proto::card::Card_Color::HEARTS => CardColor::Hearts,
                proto::card::Card_Color::TILES => CardColor::Tiles,
                proto::card::Card_Color::CLOVERS => CardColor::Clovers,
                proto::card::Card_Color::PIKES => CardColor::Pikes,
            },
            value: proto_card.value,
        }
    }
}
impl Into<proto::card::Card> for Card {
    fn into(self) -> proto::card::Card {
        let mut card = proto::card::Card::new();
        card.set_color(match self.color {
            CardColor::Hearts => proto::card::Card_Color::HEARTS,
            CardColor::Tiles => proto::card::Card_Color::TILES,
            CardColor::Clovers => proto::card::Card_Color::CLOVERS,
            CardColor::Pikes => proto::card::Card_Color::PIKES,
        });
        card.set_value(self.value);
        card
    }
}
