/* eslint-disable */
import { Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "";

export interface Card {
  color: Card_Color;
  value: number;
}

export enum Card_Color {
  HEARTS = 0,
  TILES = 1,
  CLOVERS = 2,
  PIKES = 3,
  UNRECOGNIZED = -1,
}

export function card_ColorFromJSON(object: any): Card_Color {
  switch (object) {
    case 0:
    case "HEARTS":
      return Card_Color.HEARTS;
    case 1:
    case "TILES":
      return Card_Color.TILES;
    case 2:
    case "CLOVERS":
      return Card_Color.CLOVERS;
    case 3:
    case "PIKES":
      return Card_Color.PIKES;
    case -1:
    case "UNRECOGNIZED":
    default:
      return Card_Color.UNRECOGNIZED;
  }
}

export function card_ColorToJSON(object: Card_Color): string {
  switch (object) {
    case Card_Color.HEARTS:
      return "HEARTS";
    case Card_Color.TILES:
      return "TILES";
    case Card_Color.CLOVERS:
      return "CLOVERS";
    case Card_Color.PIKES:
      return "PIKES";
    default:
      return "UNKNOWN";
  }
}

const baseCard: object = { color: 0, value: 0 };

export const Card = {
  encode(message: Card, writer: Writer = Writer.create()): Writer {
    writer.uint32(8).int32(message.color);
    writer.uint32(16).uint32(message.value);
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Card {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseCard } as Card;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.color = reader.int32() as any;
          break;
        case 2:
          message.value = reader.uint32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Card {
    const message = { ...baseCard } as Card;
    if (object.color !== undefined && object.color !== null) {
      message.color = card_ColorFromJSON(object.color);
    }
    if (object.value !== undefined && object.value !== null) {
      message.value = Number(object.value);
    }
    return message;
  },

  fromPartial(object: DeepPartial<Card>): Card {
    const message = { ...baseCard } as Card;
    if (object.color !== undefined && object.color !== null) {
      message.color = object.color;
    }
    if (object.value !== undefined && object.value !== null) {
      message.value = object.value;
    }
    return message;
  },

  toJSON(message: Card): unknown {
    const obj: any = {};
    message.color !== undefined &&
      (obj.color = card_ColorToJSON(message.color));
    message.value !== undefined && (obj.value = message.value);
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
