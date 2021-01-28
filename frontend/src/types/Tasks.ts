import type { Role } from "./Role";

export enum TaskType {
  Settings = "settings",
  DiscloseRole = "discloseRole",
  Discuss = "discuss",
}

export type Task = SettingsTask | DiscloseRoleTask | DiscussTask;

export type SettingsTask = { type: TaskType.Settings };
export type DiscloseRoleTask = { type: TaskType.DiscloseRole; role: Role };
export type DiscussTask = { type: TaskType.Discuss; timeLimit: string };
