import type { Tasks } from "./Tasks";

export enum IncomingMessageType {
  NewTask = "newTask",
}

export type IncomingMessages = {
  [IncomingMessageType.NewTask]: NewTask;
};

export type NewTask = { task: Tasks };
