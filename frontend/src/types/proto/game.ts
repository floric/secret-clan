/* eslint-disable */
import { Writer, Reader } from 'protobufjs/minimal';


export interface Game {
  token: string;
  adminId: string;
}

const baseGame: object = {
  token: "",
  adminId: "",
};

export const protobufPackage = ''

export const Game = {
  encode(message: Game, writer: Writer = Writer.create()): Writer {
    writer.uint32(10).string(message.token);
    writer.uint32(18).string(message.adminId);
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Game {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseGame } as Game;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.token = reader.string();
          break;
        case 2:
          message.adminId = reader.string();
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
    if (object.token !== undefined && object.token !== null) {
      message.token = String(object.token);
    }
    if (object.adminId !== undefined && object.adminId !== null) {
      message.adminId = String(object.adminId);
    }
    return message;
  },
  fromPartial(object: DeepPartial<Game>): Game {
    const message = { ...baseGame } as Game;
    if (object.token !== undefined && object.token !== null) {
      message.token = object.token;
    }
    if (object.adminId !== undefined && object.adminId !== null) {
      message.adminId = object.adminId;
    }
    return message;
  },
  toJSON(message: Game): unknown {
    const obj: any = {};
    message.token !== undefined && (obj.token = message.token);
    message.adminId !== undefined && (obj.adminId = message.adminId);
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