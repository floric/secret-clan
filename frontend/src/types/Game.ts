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

export type GameStats = {
  total: number;
};
