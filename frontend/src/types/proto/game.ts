/* eslint-disable */
import { Card } from "./card";
import { Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "";

export interface Game {
  token: string;
  adminId: string;
  state: Game_State;
  pot: number;
  cards: Card[];
  smallBlindId: string;
  bigBlindId: string;
}

export enum Game_State {
  INITIALIZED = 0,
  STARTED = 1,
  ABANDONED = 2,
  UNRECOGNIZED = -1,
}

export function game_StateFromJSON(object: any): Game_State {
  switch (object) {
    case 0:
    case "INITIALIZED":
      return Game_State.INITIALIZED;
    case 1:
    case "STARTED":
      return Game_State.STARTED;
    case 2:
    case "ABANDONED":
      return Game_State.ABANDONED;
    case -1:
    case "UNRECOGNIZED":
    default:
      return Game_State.UNRECOGNIZED;
  }
}

export function game_StateToJSON(object: Game_State): string {
  switch (object) {
    case Game_State.INITIALIZED:
      return "INITIALIZED";
    case Game_State.STARTED:
      return "STARTED";
    case Game_State.ABANDONED:
      return "ABANDONED";
    default:
      return "UNKNOWN";
  }
}

const baseGame: object = {
  token: "",
  adminId: "",
  state: 0,
  pot: 0,
  smallBlindId: "",
  bigBlindId: "",
};

export const Game = {
  encode(message: Game, writer: Writer = Writer.create()): Writer {
    writer.uint32(10).string(message.token);
    writer.uint32(18).string(message.adminId);
    writer.uint32(24).int32(message.state);
    writer.uint32(32).uint32(message.pot);
    for (const v of message.cards) {
      Card.encode(v!, writer.uint32(42).fork()).ldelim();
    }
    writer.uint32(50).string(message.smallBlindId);
    writer.uint32(58).string(message.bigBlindId);
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Game {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseGame } as Game;
    message.cards = [];
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.token = reader.string();
          break;
        case 2:
          message.adminId = reader.string();
          break;
        case 3:
          message.state = reader.int32() as any;
          break;
        case 4:
          message.pot = reader.uint32();
          break;
        case 5:
          message.cards.push(Card.decode(reader, reader.uint32()));
          break;
        case 6:
          message.smallBlindId = reader.string();
          break;
        case 7:
          message.bigBlindId = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Game {
    const message = { ...baseGame } as Game;
    message.cards = [];
    if (object.token !== undefined && object.token !== null) {
      message.token = String(object.token);
    }
    if (object.adminId !== undefined && object.adminId !== null) {
      message.adminId = String(object.adminId);
    }
    if (object.state !== undefined && object.state !== null) {
      message.state = game_StateFromJSON(object.state);
    }
    if (object.pot !== undefined && object.pot !== null) {
      message.pot = Number(object.pot);
    }
    if (object.cards !== undefined && object.cards !== null) {
      for (const e of object.cards) {
        message.cards.push(Card.fromJSON(e));
      }
    }
    if (object.smallBlindId !== undefined && object.smallBlindId !== null) {
      message.smallBlindId = String(object.smallBlindId);
    }
    if (object.bigBlindId !== undefined && object.bigBlindId !== null) {
      message.bigBlindId = String(object.bigBlindId);
    }
    return message;
  },

  fromPartial(object: DeepPartial<Game>): Game {
    const message = { ...baseGame } as Game;
    message.cards = [];
    if (object.token !== undefined && object.token !== null) {
      message.token = object.token;
    }
    if (object.adminId !== undefined && object.adminId !== null) {
      message.adminId = object.adminId;
    }
    if (object.state !== undefined && object.state !== null) {
      message.state = object.state;
    }
    if (object.pot !== undefined && object.pot !== null) {
      message.pot = object.pot;
    }
    if (object.cards !== undefined && object.cards !== null) {
      for (const e of object.cards) {
        message.cards.push(Card.fromPartial(e));
      }
    }
    if (object.smallBlindId !== undefined && object.smallBlindId !== null) {
      message.smallBlindId = object.smallBlindId;
    }
    if (object.bigBlindId !== undefined && object.bigBlindId !== null) {
      message.bigBlindId = object.bigBlindId;
    }
    return message;
  },

  toJSON(message: Game): unknown {
    const obj: any = {};
    message.token !== undefined && (obj.token = message.token);
    message.adminId !== undefined && (obj.adminId = message.adminId);
    message.state !== undefined &&
      (obj.state = game_StateToJSON(message.state));
    message.pot !== undefined && (obj.pot = message.pot);
    if (message.cards) {
      obj.cards = message.cards.map((e) => (e ? Card.toJSON(e) : undefined));
    } else {
      obj.cards = [];
    }
    message.smallBlindId !== undefined &&
      (obj.smallBlindId = message.smallBlindId);
    message.bigBlindId !== undefined && (obj.bigBlindId = message.bigBlindId);
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
