import type { Role } from "./Role";

export enum TaskType {
  Settings = "settings",
  DiscloseRole = "discloseRole",
  Discuss = "discuss",
}

export type Tasks = {
  [TaskType.Settings]: SettingsTask;
  [TaskType.DiscloseRole]: DiscloseRoleTask;
  [TaskType.Discuss]: DiscussTask;
};

export type SettingsTask = {};
export type DiscloseRoleTask = { role: Role };
export type DiscussTask = { timeLimit: string };
