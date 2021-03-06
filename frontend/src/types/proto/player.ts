/* eslint-disable */
import { Task } from './task';
import { Writer, Reader } from 'protobufjs/minimal';


export interface Player {
  id: string;
  name: string;
}

export interface OwnPlayer {
  id: string;
  name: string;
  openTasks: Task[];
}

const basePlayer: object = {
  id: "",
  name: "",
};

const baseOwnPlayer: object = {
  id: "",
  name: "",
};

export const protobufPackage = ''

export const Player = {
  encode(message: Player, writer: Writer = Writer.create()): Writer {
    writer.uint32(10).string(message.id);
    writer.uint32(18).string(message.name);
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Player {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...basePlayer } as Player;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.string();
          break;
        case 2:
          message.name = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },
  fromJSON(object: any): Player {
    const message = { ...basePlayer } as Player;
    if (object.id !== undefined && object.id !== null) {
      message.id = String(object.id);
    }
    if (object.name !== undefined && object.name !== null) {
      message.name = String(object.name);
    }
    return message;
  },
  fromPartial(object: DeepPartial<Player>): Player {
    const message = { ...basePlayer } as Player;
    if (object.id !== undefined && object.id !== null) {
      message.id = object.id;
    }
    if (object.name !== undefined && object.name !== null) {
      message.name = object.name;
    }
    return message;
  },
  toJSON(message: Player): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    message.name !== undefined && (obj.name = message.name);
    return obj;
  },
};

export const OwnPlayer = {
  encode(message: OwnPlayer, writer: Writer = Writer.create()): Writer {
    writer.uint32(10).string(message.id);
    writer.uint32(18).string(message.name);
    for (const v of message.openTasks) {
      Task.encode(v!, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): OwnPlayer {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseOwnPlayer } as OwnPlayer;
    message.openTasks = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.string();
          break;
        case 2:
          message.name = reader.string();
          break;
        case 3:
          message.openTasks.push(Task.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },
  fromJSON(object: any): OwnPlayer {
    const message = { ...baseOwnPlayer } as OwnPlayer;
    message.openTasks = [];
    if (object.id !== undefined && object.id !== null) {
      message.id = String(object.id);
    }
    if (object.name !== undefined && object.name !== null) {
      message.name = String(object.name);
    }
    if (object.openTasks !== undefined && object.openTasks !== null) {
      for (const e of object.openTasks) {
        message.openTasks.push(Task.fromJSON(e));
      }
    }
    return message;
  },
  fromPartial(object: DeepPartial<OwnPlayer>): OwnPlayer {
    const message = { ...baseOwnPlayer } as OwnPlayer;
    message.openTasks = [];
    if (object.id !== undefined && object.id !== null) {
      message.id = object.id;
    }
    if (object.name !== undefined && object.name !== null) {
      message.name = object.name;
    }
    if (object.openTasks !== undefined && object.openTasks !== null) {
      for (const e of object.openTasks) {
        message.openTasks.push(Task.fromPartial(e));
      }
    }
    return message;
  },
  toJSON(message: OwnPlayer): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    message.name !== undefined && (obj.name = message.name);
    if (message.openTasks) {
      obj.openTasks = message.openTasks.map(e => e ? Task.toJSON(e) : undefined);
    } else {
      obj.openTasks = [];
    }
    return obj;
  },
};

type Builtin = Date | Function | Uint8Array | string | number | undefined;
export type DeepPartial<T> = T extends Builtin
  ? T
  : T extends Array<infer U>
  ? Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U>
  ? ReadonlyArray<DeepPartial<U>>
  : T extends { $case: string }
  ? { [K in keyof Omit<T, '$case'>]?: DeepPartial<T[K]> } & { $case: T['$case'] }
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;