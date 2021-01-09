use super::{connections::Connections, logger::init_logger};
use crate::{
    config::AppConfig,
    db::{Client, Database},
    model::{Game, OutgoingMessage, Player, Voting, WsCommand, WsRequest},
};
use envconfig::Envconfig;
use futures::stream::SplitSink;
use log::error;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

pub struct DbClients {
    games: Client<Game>,
    players: Client<Player>,
    votings: Client<Voting>,
}

impl DbClients {
    pub fn init() -> DbClients {
        let (mut games_repo, games_sender) = Database::init("games");
        let (mut players_repo, players_sender) = Database::init("players");
        let (mut votings_repo, votings_sender) = Database::init("votings");

        tokio::task::spawn(async move {
            tokio::join!(
                players_repo.start_listening(),
                games_repo.start_listening(),
                votings_repo.start_listening()
            );
        });

        DbClients {
            games: Client::new(games_sender),
            players: Client::new(players_sender),
            votings: Client::new(votings_sender),
        }
    }

    pub fn games(&self) -> &Client<Game> {
        &self.games
    }

    pub fn players(&self) -> &Client<Player> {
        &self.players
    }

    pub fn votings(&self) -> &Client<Voting> {
        &self.votings
    }
}

pub struct AppContext {
    db: DbClients,
    config: AppConfig,
    message_sender: mpsc::Sender<WsCommand>,
}

impl AppContext {
    pub fn init() -> AppContext {
        let config = AppConfig::init_from_env().expect("Loading server config failed");

        init_logger(&config);

        let db = DbClients::init();

        let (mut connections, message_sender) = Connections::new();

        tokio::task::spawn(async move {
            connections.start_listening().await;
        });

        AppContext {
            db,
            config,
            message_sender,
        }
    }

    pub fn db(&self) -> &DbClients {
        &self.db
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn is_dev(&self) -> bool {
        cfg!(debug_assertions)
    }

    pub async fn add_connection(&self, peer_id: &str, sender: SplitSink<WebSocket, Message>) {
        if self
            .message_sender
            .clone()
            .send((String::from(peer_id), WsRequest::AddConnection { sender }))
            .await
            .is_err()
        {
            error!("Sending connection from {} has failed", peer_id);
        };
    }

    pub async fn send_message(
        &self,
        message: OutgoingMessage,
        peer_id: &str,
    ) -> Result<(), String> {
        self.message_sender
            .clone()
            .send((
                String::from(peer_id),
                WsRequest::SendMessage { msg: message },
            ))
            .await
            .map_err(|err| err.to_string())
    }
}
