import type { Game } from "./Game";
import type { Player } from "./Player";
import type { Tasks } from "./Tasks";

export enum IncomingMessageType {
  NewTask = "newTask",
  PlayerUpdated = "playerUpdated",
  GameUpdated = "gameUpdated",
}

export type IncomingMessages = {
  [IncomingMessageType.NewTask]: NewTask;
  [IncomingMessageType.PlayerUpdated]: PlayerUpdated;
  [IncomingMessageType.GameUpdated]: GameUpdated;
};

export type NewTask = { task: Tasks };
export type PlayerUpdated = { player: Player };
export type GameUpdated = { game: Game };
