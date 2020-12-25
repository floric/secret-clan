use crate::db::Persist;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sled::IVec;
use std::{collections::HashSet, convert::TryFrom};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum GameState {
    Initialized,
    Abandoned,
    Started,
}

/// This struct defines a game session. Each valid game needs to have an admin who is responsible for defining game settings.
/// The admin is also a player but currently not added redundantly to player_ids as well as admin_id.
///
/// Please note that each method here only mutates the struct state but still needs to be persisted to the database separately.
///
/// Example:
/// ```no_run
/// use secret_clan::{model::Game, db::{Database, Client}};
/// let mut repo: Database<Game> = Database::init("test");
/// let sender = repo.sender();
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
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Game {
    token: String,
    creation_time: DateTime<Utc>,
    last_action_time: DateTime<Utc>,
    admin_id: Option<String>,
    player_ids: HashSet<String>,
    state: GameState,
}

impl Game {
    pub fn new(admin_id: &str, token: &str) -> Self {
        Game {
            token: String::from(token).to_uppercase(),
            creation_time: Utc::now(),
            last_action_time: Utc::now(),
            admin_id: Some(String::from(admin_id)),
            player_ids: HashSet::new(),
            state: GameState::Initialized,
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn player_ids(&self) -> &HashSet<String> {
        &self.player_ids
    }

    pub fn admin_id(&self) -> &Option<String> {
        &self.admin_id
    }

    pub fn last_action_time(&self) -> &DateTime<Utc> {
        &self.last_action_time
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn add_player(&mut self, player_id: &str) {
        match self.admin_id {
            Some(_) => {
                self.player_ids.insert(String::from(player_id));
            }
            None => self.admin_id = Some(String::from(player_id)),
        }
    }

    pub fn remove_player(&mut self, player_id: &str) {
        if self.player_ids.contains(player_id) {
            self.player_ids.remove(player_id);
        } else if self
            .admin_id
            .to_owned()
            .filter(|id| id == player_id)
            .is_some()
        {
            if let Some(next_player_id) = self.player_ids.iter().next().map(String::from) {
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
