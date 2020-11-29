import type { PublicPlayer } from "./Player";

export type Game = {
  token: string;
  creation_time: string;
  last_action_time: string;
  player_ids: Array<string>;
  admin_id: string;
};

export type GameDetails = {
  game: Game;
  participants: {
    admin: PublicPlayer;
    players: Array<PublicPlayer>;
  };
};
