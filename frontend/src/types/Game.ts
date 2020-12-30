import type { Player } from "./Player";
import { Tasks } from "./Tasks";

export enum GameState {
  Initialized = "Initialized",
  Started = "Started",
}

export type Game = {
  token: string;
  playerIds: Array<string>;
  adminId: string;
  state: GameState;
};

export type GameDetails = {
  game: Game;
  players: Record<string, Player>;
  openTasks: Array<Tasks>;
};

export type GameStats = {
  total: number;
};
