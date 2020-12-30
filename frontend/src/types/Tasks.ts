export enum TaskType {
  Settings = "settings",
}

export type Tasks = {
  [TaskType.Settings]: SettingsTask;
};

export type SettingsTask = {};
