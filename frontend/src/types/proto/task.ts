/* eslint-disable */
import { Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "";

export interface Task {
  definition?: { $case: "settings"; settings: Task_Settings };
}

export interface Task_Settings {}

const baseTask: object = {};

export const Task = {
  encode(message: Task, writer: Writer = Writer.create()): Writer {
    if (message.definition?.$case === "settings") {
      Task_Settings.encode(
        message.definition.settings,
        writer.uint32(10).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Task {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = globalThis.Object.create(baseTask) as Task;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.definition = {
            $case: "settings",
            settings: Task_Settings.decode(reader, reader.uint32()),
          };
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Task {
    const message = globalThis.Object.create(baseTask) as Task;
    if (object.settings !== undefined && object.settings !== null) {
      message.definition = {
        $case: "settings",
        settings: Task_Settings.fromJSON(object.settings),
      };
    }
    return message;
  },

  fromPartial(object: DeepPartial<Task>): Task {
    const message = { ...baseTask } as Task;
    if (
      object.definition?.$case === "settings" &&
      object.definition?.settings !== undefined &&
      object.definition?.settings !== null
    ) {
      message.definition = {
        $case: "settings",
        settings: Task_Settings.fromPartial(object.definition.settings),
      };
    }
    return message;
  },

  toJSON(message: Task): unknown {
    const obj: any = {};
    message.definition?.$case === "settings" &&
      (obj.settings = message.definition?.settings
        ? Task_Settings.toJSON(message.definition?.settings)
        : undefined);
    return obj;
  },
};

const baseTask_Settings: object = {};

export const Task_Settings = {
  encode(_: Task_Settings, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Task_Settings {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = globalThis.Object.create(
      baseTask_Settings
    ) as Task_Settings;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): Task_Settings {
    const message = globalThis.Object.create(
      baseTask_Settings
    ) as Task_Settings;
    return message;
  },

  fromPartial(_: DeepPartial<Task_Settings>): Task_Settings {
    const message = { ...baseTask_Settings } as Task_Settings;
    return message;
  },

  toJSON(_: Task_Settings): unknown {
    const obj: any = {};
    return obj;
  },
};

declare var self: any | undefined;
declare var window: any | undefined;
var globalThis: any = (() => {
  if (typeof globalThis !== "undefined") return globalThis;
  if (typeof self !== "undefined") return self;
  if (typeof window !== "undefined") return window;
  if (typeof global !== "undefined") return global;
  throw "Unable to locate global object";
})();

type Builtin = Date | Function | Uint8Array | string | number | undefined;
export type DeepPartial<T> = T extends Builtin
  ? T
  : T extends Array<infer U>
  ? Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U>
  ? ReadonlyArray<DeepPartial<U>>
  : T extends { $case: string }
  ? { [K in keyof Omit<T, "$case">]?: DeepPartial<T[K]> } & {
      $case: T["$case"];
    }
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;
