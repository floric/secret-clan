import type { Role } from "./Role";

export enum TaskType {
  Settings = "settings",
  DiscloseRole = "discloseRole",
}

export type Tasks = {
  [TaskType.Settings]: SettingsTask;
  [TaskType.DiscloseRole]: DiscloseRoleTask;
};

export type SettingsTask = {};
export type DiscloseRoleTask = { role: Role };
