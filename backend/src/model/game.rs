use super::{role::Party, Role};
use crate::{
    db::Persist,
    model::proto::{self},
};
use chrono::{DateTime, Utc};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use sled::IVec;
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
};

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
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    token: String,
    creation_time: DateTime<Utc>,
    last_action_time: DateTime<Utc>,
    admin_id: Option<String>,
    player_ids: HashSet<String>,
    state: GameState,
    assigned_roles: HashMap<String, Role>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameResponse {
    token: String,
    admin_id: Option<String>,
    player_ids: HashSet<String>,
    state: GameState,
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
            player_ids: HashSet::with_capacity(10),
            state: GameState::Initialized,
            assigned_roles: HashMap::with_capacity(10),
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

    pub fn all_player_ids(&self) -> Vec<String> {
        let mut ids = vec![];
        if let Some(id) = &self.admin_id {
            ids.push(String::from(id));
        }
        for id in &self.player_ids {
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

    pub fn assigned_roles(&self) -> &HashMap<String, Role> {
        &self.assigned_roles
    }

    pub fn add_player(&mut self, player_id: &str) {
        match self.admin_id {
            Some(_) => {
                self.player_ids.insert(String::from(player_id));
                if self.state == GameState::Abandoned {
                    self.state = GameState::Initialized;
                }
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

    pub fn to_response(&self) -> GameResponse {
        GameResponse {
            admin_id: self.admin_id.to_owned(),
            player_ids: self.player_ids.to_owned(),
            state: self.state.to_owned(),
            token: self.token.to_owned(),
        }
    }

    pub fn start(&mut self) {
        // TODO Determine distribution of roles based on settings or general rules
        let mut roles = self.get_equal_good_and_bad_distribution();
        let mut rng = thread_rng();
        self.assigned_roles = self
            .player_ids
            .iter()
            .chain(std::iter::once(&self.admin_id().to_owned().unwrap()))
            .map(|id| {
                let role_index = rng.gen_range(0..roles.len());
                let role = roles.remove(role_index);
                (String::from(id), role)
            })
            .collect();
        self.state = GameState::Started;
    }

    fn get_equal_good_and_bad_distribution(&self) -> Vec<Role> {
        // TODO Extract roles to separate files and define generic game integration
        let bad_role = Role::new("Bad", Party::Bad, "A bad person. Fight the law.");
        let good_role = Role::new("Good", Party::Good, "A good person. Keep the law.");
        let mut all_roles = vec![];

        // add for uneven games always one good player more
        let total_count = self.player_ids.len() + 1;
        for _ in 0..(total_count - total_count % 2) / 2 {
            all_roles.push(bad_role.clone());
            all_roles.push(good_role.clone());
        }
        if total_count % 2 == 1 {
            all_roles.push(good_role);
        }

        all_roles
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

#[cfg(test)]
mod tests {
    use crate::model::Party;

    use super::Game;

    #[test]
    fn should_distribute_roles_evenly() {
        let mut game = Game::new("admin", "TOKEN");
        game.add_player("player_a");
        game.add_player("player_b");
        game.add_player("player_c");
        game.add_player("player_d");

        game.start();

        let roles = game.assigned_roles();
        assert_eq!(
            roles.values().filter(|r| r.party() == &Party::Good).count(),
            3
        );
        assert_eq!(
            roles.values().filter(|r| r.party() == &Party::Bad).count(),
            2
        );
    }
}

impl Into<proto::game::Game> for Game {
    fn into(self) -> proto::game::Game {
        let mut game = proto::game::Game::new();
        game.set_id(String::from(self.token()));

        game
    }
}
