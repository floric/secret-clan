import type { PublicPlayer } from "./Player";

export enum GameState {
  Initialized = "Initialized",
  Started = "Started",
}

export type Game = {
  token: string;
  creation_time: string;
  last_action_time: string;
  player_ids: Array<string>;
  admin_id: string;
  state: GameState;
};

export type GameDetails = {
  game: Game;
  participants: {
    admin: PublicPlayer;
    players: Array<PublicPlayer>;
  };
};

export type GameStats = {
  total: number;
};
