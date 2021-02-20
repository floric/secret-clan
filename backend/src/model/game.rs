use super::{card::CardColor, Card};
use crate::{
    db::Persist,
    model::proto::{self},
};
use chrono::{DateTime, Utc};
use log::error;
use rand::prelude::*;
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};
use sled::IVec;
use std::{
    collections::{BTreeMap, VecDeque},
    convert::TryFrom,
    iter::FromIterator,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum GameState {
    Initialized,
    Abandoned,
    Started,
}

impl Into<proto::game::Game_State> for GameState {
    fn into(self) -> proto::game::Game_State {
        match self {
            GameState::Initialized => proto::game::Game_State::INITIALIZED,
            GameState::Abandoned => proto::game::Game_State::ABANDONED,
            GameState::Started => proto::game::Game_State::STARTED,
        }
    }
}

/// This struct defines a game session. Each valid game needs to have an admin who is responsible for defining game settings.
/// The admin is also a player but currently not added redundantly to player_ids as well as admin_id.
///
/// Please note that each method here only mutates the struct state but still needs to be persisted to the database separately.
///
/// Example:
/// ```no_run
/// use secret_clan::{model::Game, db::{Database, Client, Command}};
/// let (mut repo, sender): (Database<Game>, tokio::sync::mpsc::Sender<Command<Game>>) = Database::init("test");
/// std::thread::spawn(move || {
///     repo.start_listening();
/// });
/// std::thread::spawn(move || {
///     async move {
///         let client = Client::new(sender.clone());
///         let mut game = Game::new("admin", "GAME");
///         game.start();
///         let _ = client.persist(&game).await;
///     }
/// });
/// ```
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    token: String,
    creation_time: DateTime<Utc>,
    last_action_time: DateTime<Utc>,
    admin_id: Option<String>,
    player_ids: BTreeMap<String, u32>,
    state: GameState,
    pot: u32,
    blind: u32,
    small_blind_id: Option<String>,
    big_blind_id: Option<String>,
    deck: VecDeque<Card>,
}

impl Game {
    pub fn new(admin_id: &str, token: &str) -> Self {
        if token.to_uppercase() != token {
            panic!("Only uppercase characters allowed");
        }
        Game {
            token: String::from(token).to_uppercase(),
            creation_time: Utc::now(),
            last_action_time: Utc::now(),
            admin_id: Some(String::from(admin_id)),
            player_ids: BTreeMap::new(),
            state: GameState::Initialized,
            pot: 0,
            blind: 10,
            small_blind_id: None,
            big_blind_id: None,
            deck: VecDeque::new(),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn admin_id(&self) -> &Option<String> {
        &self.admin_id
    }

    pub fn all_player_ids(&self) -> Vec<String> {
        let mut ids = vec![];
        // TODO think about admin position instead of always first
        if let Some(id) = &self.admin_id {
            ids.push(String::from(id));
        }
        let mut sorted_players = Vec::from_iter(self.player_ids.clone());
        sorted_players.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

        for (id, _) in sorted_players {
            ids.push(String::from(id));
        }
        ids
    }

    pub fn last_action_time(&self) -> &DateTime<Utc> {
        &self.last_action_time
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn add_player(&mut self, player_id: &str, position: u32) {
        match self.admin_id {
            Some(_) => {
                self.player_ids.insert(String::from(player_id), position);
                if self.state == GameState::Abandoned {
                    self.state = GameState::Initialized;
                }
            }
            None => self.admin_id = Some(String::from(player_id)),
        }
    }

    pub fn remove_player(&mut self, player_id: &str) {
        if self.player_ids.contains_key(player_id) {
            self.player_ids.remove(player_id);
        } else if self
            .admin_id
            .to_owned()
            .filter(|id| id == player_id)
            .is_some()
        {
            if let Some(next_player_id) = self.player_ids.keys().next().map(String::from) {
                self.admin_id = Some(String::from(&next_player_id));
                self.player_ids.remove(&next_player_id);
            } else {
                self.admin_id = None;
                self.state = GameState::Abandoned
            }
        } else {
            // game is already abandoned or requesting user is no admin or player
        }
    }

    pub fn start(&mut self) {
        self.state = GameState::Started;
        self.pot = 0;
    }

    pub fn retrieve_card(&mut self) -> Option<Card> {
        self.deck.pop_front()
    }

    pub fn shuffle_deck(&mut self) {
        let mut ordered_deck = vec![];
        for col in vec![
            CardColor::Pikes,
            CardColor::Hearts,
            CardColor::Clovers,
            CardColor::Tiles,
        ] {
            for i in 1..14 {
                ordered_deck.push(Card::new(col.clone(), i));
            }
        }

        let mut rng = thread_rng();
        self.deck = VecDeque::new();
        while ordered_deck.len() > 0 {
            let next_index = rng.gen_range(0..ordered_deck.len());
            let card = ordered_deck.remove(next_index);
            self.deck.push_back(card);
        }
    }

    pub fn set_blinds_roles(&mut self) {
        let ids = self.all_player_ids();
        if ids.len() < 2 {
            return;
        }

        // TODO determine positions from player in game easily
        let current_small_blind_pos = if let Some(id) = self.small_blind_id.clone() {
            ids.iter().position(|r| r == &id)
        } else {
            None
        };
        match current_small_blind_pos {
            Some(current_small_blind_pos) => {
                if current_small_blind_pos >= ids.len() - 1 {
                    self.small_blind_id = Some(ids.get(0).unwrap().clone());
                    self.big_blind_id = Some(ids.get(1).unwrap().clone());
                } else {
                    self.small_blind_id =
                        Some(ids.get(current_small_blind_pos + 1).unwrap().clone());
                    self.big_blind_id = Some(ids.get(current_small_blind_pos + 2).unwrap().clone());
                }
            }
            None => {
                self.small_blind_id = Some(ids.get(0).unwrap().clone());
                self.big_blind_id = Some(ids.get(1).unwrap().clone());
            }
        }
    }

    pub fn deck(&self) -> &VecDeque<Card> {
        &self.deck
    }
}

impl Persist for Game {
    fn id(&self) -> &str {
        self.token()
    }
}

impl Into<IVec> for Game {
    fn into(self) -> IVec {
        IVec::from(bincode::serialize(&self).unwrap())
    }
}

impl TryFrom<IVec> for Game {
    type Error = bincode::Error;
    fn try_from(bytes: IVec) -> Result<Self, Self::Error> {
        let vec: Vec<u8> = bytes.to_vec();

        bincode::deserialize(&vec)
    }
}

impl Into<proto::game::Game> for Game {
    fn into(self) -> proto::game::Game {
        let mut game = proto::game::Game::new();
        game.set_token(String::from(self.token()));
        if let Some(id) = self.admin_id {
            game.set_admin_id(id);
        }
        game.set_state(self.state.into());
        if self.big_blind_id.is_some() {
            game.set_big_blind_id(self.big_blind_id.unwrap());
        }
        if self.small_blind_id.is_some() {
            game.set_small_blind_id(self.small_blind_id.unwrap());
        }
        game.set_blind(self.blind);
        game
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[tokio::test]
    async fn should_shuffle_deck() {
        let mut game = Game::new("admin", "GAME");

        game.shuffle_deck();

        assert_eq!(game.deck().len(), 52);
    }
}
