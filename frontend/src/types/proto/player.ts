/* eslint-disable */
import { Task } from "./task";
import { Card } from "./card";
import { Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "";

export interface Player {
  id: string;
  name: string;
  credits: number;
  position: number;
}

export interface OwnPlayer {
  id: string;
  name: string;
  openTasks: Task[];
  credits: number;
  cards: Card[];
  position: number;
}

const basePlayer: object = { id: "", name: "", credits: 0, position: 0 };

export const Player = {
  encode(message: Player, writer: Writer = Writer.create()): Writer {
    writer.uint32(10).string(message.id);
    writer.uint32(18).string(message.name);
    writer.uint32(24).uint32(message.credits);
    writer.uint32(32).uint32(message.position);
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Player {
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
        case 3:
          message.credits = reader.uint32();
          break;
        case 4:
          message.position = reader.uint32();
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
    if (object.credits !== undefined && object.credits !== null) {
      message.credits = Number(object.credits);
    }
    if (object.position !== undefined && object.position !== null) {
      message.position = Number(object.position);
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
    if (object.credits !== undefined && object.credits !== null) {
      message.credits = object.credits;
    }
    if (object.position !== undefined && object.position !== null) {
      message.position = object.position;
    }
    return message;
  },

  toJSON(message: Player): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    message.name !== undefined && (obj.name = message.name);
    message.credits !== undefined && (obj.credits = message.credits);
    message.position !== undefined && (obj.position = message.position);
    return obj;
  },
};

const baseOwnPlayer: object = { id: "", name: "", credits: 0, position: 0 };

export const OwnPlayer = {
  encode(message: OwnPlayer, writer: Writer = Writer.create()): Writer {
    writer.uint32(10).string(message.id);
    writer.uint32(18).string(message.name);
    for (const v of message.openTasks) {
      Task.encode(v!, writer.uint32(26).fork()).ldelim();
    }
    writer.uint32(32).uint32(message.credits);
    for (const v of message.cards) {
      Card.encode(v!, writer.uint32(42).fork()).ldelim();
    }
    writer.uint32(48).uint32(message.position);
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): OwnPlayer {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseOwnPlayer } as OwnPlayer;
    message.openTasks = [];
    message.cards = [];
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
        case 4:
          message.credits = reader.uint32();
          break;
        case 5:
          message.cards.push(Card.decode(reader, reader.uint32()));
          break;
        case 6:
          message.position = reader.uint32();
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
    message.cards = [];
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
    if (object.credits !== undefined && object.credits !== null) {
      message.credits = Number(object.credits);
    }
    if (object.cards !== undefined && object.cards !== null) {
      for (const e of object.cards) {
        message.cards.push(Card.fromJSON(e));
      }
    }
    if (object.position !== undefined && object.position !== null) {
      message.position = Number(object.position);
    }
    return message;
  },

  fromPartial(object: DeepPartial<OwnPlayer>): OwnPlayer {
    const message = { ...baseOwnPlayer } as OwnPlayer;
    message.openTasks = [];
    message.cards = [];
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
    if (object.credits !== undefined && object.credits !== null) {
      message.credits = object.credits;
    }
    if (object.cards !== undefined && object.cards !== null) {
      for (const e of object.cards) {
        message.cards.push(Card.fromPartial(e));
      }
    }
    if (object.position !== undefined && object.position !== null) {
      message.position = object.position;
    }
    return message;
  },

  toJSON(message: OwnPlayer): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    message.name !== undefined && (obj.name = message.name);
    if (message.openTasks) {
      obj.openTasks = message.openTasks.map((e) =>
        e ? Task.toJSON(e) : undefined
      );
    } else {
      obj.openTasks = [];
    }
    message.credits !== undefined && (obj.credits = message.credits);
    if (message.cards) {
      obj.cards = message.cards.map((e) => (e ? Card.toJSON(e) : undefined));
    } else {
      obj.cards = [];
    }
    message.position !== undefined && (obj.position = message.position);
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
  ? { [K in keyof Omit<T, "$case">]?: DeepPartial<T[K]> } & {
      $case: T["$case"];
    }
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;
