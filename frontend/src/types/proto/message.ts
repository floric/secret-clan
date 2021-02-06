/* eslint-disable */
import { Player, OwnPlayer } from "./player";
import { Game } from "./game";
import { Writer, Reader } from "protobufjs/minimal";

export const protobufPackage = "";

export interface Client {
  message?:
    | { $case: "authConfirmed"; authConfirmed: Client_AuthConfirmed }
    | { $case: "nameUpdated"; nameUpdated: Client_NameUpdated };
}

export interface Client_AuthConfirmed {
  token: string;
}

export interface Client_NameUpdated {
  name: string;
}

export interface Server {
  message?:
    | { $case: "playerUpdated"; playerUpdated: Server_PlayerUpdated }
    | { $case: "gameUpdated"; gameUpdated: Server_GameUpdated }
    | { $case: "selfUpdated"; selfUpdated: Server_SelfUpdated }
    | { $case: "playerEntered"; playerEntered: Server_PlayerEntered }
    | { $case: "playerLeft"; playerLeft: Server_PlayerLeft }
    | { $case: "gameDeclined"; gameDeclined: Server_GameDeclined };
}

export interface Server_PlayerUpdated {
  player?: Player;
}

export interface Server_SelfUpdated {
  player?: OwnPlayer;
}

export interface Server_GameUpdated {
  game?: Game;
}

export interface Server_PlayerEntered {
  player?: Player;
}

export interface Server_PlayerLeft {
  playerId: string;
}

export interface Server_GameDeclined {}

const baseClient: object = {};

export const Client = {
  encode(message: Client, writer: Writer = Writer.create()): Writer {
    if (message.message?.$case === "authConfirmed") {
      Client_AuthConfirmed.encode(
        message.message.authConfirmed,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.message?.$case === "nameUpdated") {
      Client_NameUpdated.encode(
        message.message.nameUpdated,
        writer.uint32(18).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Client {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseClient } as Client;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.message = {
            $case: "authConfirmed",
            authConfirmed: Client_AuthConfirmed.decode(reader, reader.uint32()),
          };
          break;
        case 2:
          message.message = {
            $case: "nameUpdated",
            nameUpdated: Client_NameUpdated.decode(reader, reader.uint32()),
          };
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Client {
    const message = { ...baseClient } as Client;
    if (object.authConfirmed !== undefined && object.authConfirmed !== null) {
      message.message = {
        $case: "authConfirmed",
        authConfirmed: Client_AuthConfirmed.fromJSON(object.authConfirmed),
      };
    }
    if (object.nameUpdated !== undefined && object.nameUpdated !== null) {
      message.message = {
        $case: "nameUpdated",
        nameUpdated: Client_NameUpdated.fromJSON(object.nameUpdated),
      };
    }
    return message;
  },

  fromPartial(object: DeepPartial<Client>): Client {
    const message = { ...baseClient } as Client;
    if (
      object.message?.$case === "authConfirmed" &&
      object.message?.authConfirmed !== undefined &&
      object.message?.authConfirmed !== null
    ) {
      message.message = {
        $case: "authConfirmed",
        authConfirmed: Client_AuthConfirmed.fromPartial(
          object.message.authConfirmed
        ),
      };
    }
    if (
      object.message?.$case === "nameUpdated" &&
      object.message?.nameUpdated !== undefined &&
      object.message?.nameUpdated !== null
    ) {
      message.message = {
        $case: "nameUpdated",
        nameUpdated: Client_NameUpdated.fromPartial(object.message.nameUpdated),
      };
    }
    return message;
  },

  toJSON(message: Client): unknown {
    const obj: any = {};
    message.message?.$case === "authConfirmed" &&
      (obj.authConfirmed = message.message?.authConfirmed
        ? Client_AuthConfirmed.toJSON(message.message?.authConfirmed)
        : undefined);
    message.message?.$case === "nameUpdated" &&
      (obj.nameUpdated = message.message?.nameUpdated
        ? Client_NameUpdated.toJSON(message.message?.nameUpdated)
        : undefined);
    return obj;
  },
};

const baseClient_AuthConfirmed: object = { token: "" };

export const Client_AuthConfirmed = {
  encode(
    message: Client_AuthConfirmed,
    writer: Writer = Writer.create()
  ): Writer {
    writer.uint32(10).string(message.token);
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Client_AuthConfirmed {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseClient_AuthConfirmed } as Client_AuthConfirmed;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.token = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Client_AuthConfirmed {
    const message = { ...baseClient_AuthConfirmed } as Client_AuthConfirmed;
    if (object.token !== undefined && object.token !== null) {
      message.token = String(object.token);
    }
    return message;
  },

  fromPartial(object: DeepPartial<Client_AuthConfirmed>): Client_AuthConfirmed {
    const message = { ...baseClient_AuthConfirmed } as Client_AuthConfirmed;
    if (object.token !== undefined && object.token !== null) {
      message.token = object.token;
    }
    return message;
  },

  toJSON(message: Client_AuthConfirmed): unknown {
    const obj: any = {};
    message.token !== undefined && (obj.token = message.token);
    return obj;
  },
};

const baseClient_NameUpdated: object = { name: "" };

export const Client_NameUpdated = {
  encode(
    message: Client_NameUpdated,
    writer: Writer = Writer.create()
  ): Writer {
    writer.uint32(10).string(message.name);
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Client_NameUpdated {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseClient_NameUpdated } as Client_NameUpdated;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Client_NameUpdated {
    const message = { ...baseClient_NameUpdated } as Client_NameUpdated;
    if (object.name !== undefined && object.name !== null) {
      message.name = String(object.name);
    }
    return message;
  },

  fromPartial(object: DeepPartial<Client_NameUpdated>): Client_NameUpdated {
    const message = { ...baseClient_NameUpdated } as Client_NameUpdated;
    if (object.name !== undefined && object.name !== null) {
      message.name = object.name;
    }
    return message;
  },

  toJSON(message: Client_NameUpdated): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    return obj;
  },
};

const baseServer: object = {};

export const Server = {
  encode(message: Server, writer: Writer = Writer.create()): Writer {
    if (message.message?.$case === "playerUpdated") {
      Server_PlayerUpdated.encode(
        message.message.playerUpdated,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.message?.$case === "gameUpdated") {
      Server_GameUpdated.encode(
        message.message.gameUpdated,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.message?.$case === "selfUpdated") {
      Server_SelfUpdated.encode(
        message.message.selfUpdated,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.message?.$case === "playerEntered") {
      Server_PlayerEntered.encode(
        message.message.playerEntered,
        writer.uint32(34).fork()
      ).ldelim();
    }
    if (message.message?.$case === "playerLeft") {
      Server_PlayerLeft.encode(
        message.message.playerLeft,
        writer.uint32(42).fork()
      ).ldelim();
    }
    if (message.message?.$case === "gameDeclined") {
      Server_GameDeclined.encode(
        message.message.gameDeclined,
        writer.uint32(50).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Server {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseServer } as Server;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.message = {
            $case: "playerUpdated",
            playerUpdated: Server_PlayerUpdated.decode(reader, reader.uint32()),
          };
          break;
        case 2:
          message.message = {
            $case: "gameUpdated",
            gameUpdated: Server_GameUpdated.decode(reader, reader.uint32()),
          };
          break;
        case 3:
          message.message = {
            $case: "selfUpdated",
            selfUpdated: Server_SelfUpdated.decode(reader, reader.uint32()),
          };
          break;
        case 4:
          message.message = {
            $case: "playerEntered",
            playerEntered: Server_PlayerEntered.decode(reader, reader.uint32()),
          };
          break;
        case 5:
          message.message = {
            $case: "playerLeft",
            playerLeft: Server_PlayerLeft.decode(reader, reader.uint32()),
          };
          break;
        case 6:
          message.message = {
            $case: "gameDeclined",
            gameDeclined: Server_GameDeclined.decode(reader, reader.uint32()),
          };
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Server {
    const message = { ...baseServer } as Server;
    if (object.playerUpdated !== undefined && object.playerUpdated !== null) {
      message.message = {
        $case: "playerUpdated",
        playerUpdated: Server_PlayerUpdated.fromJSON(object.playerUpdated),
      };
    }
    if (object.gameUpdated !== undefined && object.gameUpdated !== null) {
      message.message = {
        $case: "gameUpdated",
        gameUpdated: Server_GameUpdated.fromJSON(object.gameUpdated),
      };
    }
    if (object.selfUpdated !== undefined && object.selfUpdated !== null) {
      message.message = {
        $case: "selfUpdated",
        selfUpdated: Server_SelfUpdated.fromJSON(object.selfUpdated),
      };
    }
    if (object.playerEntered !== undefined && object.playerEntered !== null) {
      message.message = {
        $case: "playerEntered",
        playerEntered: Server_PlayerEntered.fromJSON(object.playerEntered),
      };
    }
    if (object.playerLeft !== undefined && object.playerLeft !== null) {
      message.message = {
        $case: "playerLeft",
        playerLeft: Server_PlayerLeft.fromJSON(object.playerLeft),
      };
    }
    if (object.gameDeclined !== undefined && object.gameDeclined !== null) {
      message.message = {
        $case: "gameDeclined",
        gameDeclined: Server_GameDeclined.fromJSON(object.gameDeclined),
      };
    }
    return message;
  },

  fromPartial(object: DeepPartial<Server>): Server {
    const message = { ...baseServer } as Server;
    if (
      object.message?.$case === "playerUpdated" &&
      object.message?.playerUpdated !== undefined &&
      object.message?.playerUpdated !== null
    ) {
      message.message = {
        $case: "playerUpdated",
        playerUpdated: Server_PlayerUpdated.fromPartial(
          object.message.playerUpdated
        ),
      };
    }
    if (
      object.message?.$case === "gameUpdated" &&
      object.message?.gameUpdated !== undefined &&
      object.message?.gameUpdated !== null
    ) {
      message.message = {
        $case: "gameUpdated",
        gameUpdated: Server_GameUpdated.fromPartial(object.message.gameUpdated),
      };
    }
    if (
      object.message?.$case === "selfUpdated" &&
      object.message?.selfUpdated !== undefined &&
      object.message?.selfUpdated !== null
    ) {
      message.message = {
        $case: "selfUpdated",
        selfUpdated: Server_SelfUpdated.fromPartial(object.message.selfUpdated),
      };
    }
    if (
      object.message?.$case === "playerEntered" &&
      object.message?.playerEntered !== undefined &&
      object.message?.playerEntered !== null
    ) {
      message.message = {
        $case: "playerEntered",
        playerEntered: Server_PlayerEntered.fromPartial(
          object.message.playerEntered
        ),
      };
    }
    if (
      object.message?.$case === "playerLeft" &&
      object.message?.playerLeft !== undefined &&
      object.message?.playerLeft !== null
    ) {
      message.message = {
        $case: "playerLeft",
        playerLeft: Server_PlayerLeft.fromPartial(object.message.playerLeft),
      };
    }
    if (
      object.message?.$case === "gameDeclined" &&
      object.message?.gameDeclined !== undefined &&
      object.message?.gameDeclined !== null
    ) {
      message.message = {
        $case: "gameDeclined",
        gameDeclined: Server_GameDeclined.fromPartial(
          object.message.gameDeclined
        ),
      };
    }
    return message;
  },

  toJSON(message: Server): unknown {
    const obj: any = {};
    message.message?.$case === "playerUpdated" &&
      (obj.playerUpdated = message.message?.playerUpdated
        ? Server_PlayerUpdated.toJSON(message.message?.playerUpdated)
        : undefined);
    message.message?.$case === "gameUpdated" &&
      (obj.gameUpdated = message.message?.gameUpdated
        ? Server_GameUpdated.toJSON(message.message?.gameUpdated)
        : undefined);
    message.message?.$case === "selfUpdated" &&
      (obj.selfUpdated = message.message?.selfUpdated
        ? Server_SelfUpdated.toJSON(message.message?.selfUpdated)
        : undefined);
    message.message?.$case === "playerEntered" &&
      (obj.playerEntered = message.message?.playerEntered
        ? Server_PlayerEntered.toJSON(message.message?.playerEntered)
        : undefined);
    message.message?.$case === "playerLeft" &&
      (obj.playerLeft = message.message?.playerLeft
        ? Server_PlayerLeft.toJSON(message.message?.playerLeft)
        : undefined);
    message.message?.$case === "gameDeclined" &&
      (obj.gameDeclined = message.message?.gameDeclined
        ? Server_GameDeclined.toJSON(message.message?.gameDeclined)
        : undefined);
    return obj;
  },
};

const baseServer_PlayerUpdated: object = {};

export const Server_PlayerUpdated = {
  encode(
    message: Server_PlayerUpdated,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.player !== undefined && message.player !== undefined) {
      Player.encode(message.player, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Server_PlayerUpdated {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseServer_PlayerUpdated } as Server_PlayerUpdated;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.player = Player.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Server_PlayerUpdated {
    const message = { ...baseServer_PlayerUpdated } as Server_PlayerUpdated;
    if (object.player !== undefined && object.player !== null) {
      message.player = Player.fromJSON(object.player);
    }
    return message;
  },

  fromPartial(object: DeepPartial<Server_PlayerUpdated>): Server_PlayerUpdated {
    const message = { ...baseServer_PlayerUpdated } as Server_PlayerUpdated;
    if (object.player !== undefined && object.player !== null) {
      message.player = Player.fromPartial(object.player);
    }
    return message;
  },

  toJSON(message: Server_PlayerUpdated): unknown {
    const obj: any = {};
    message.player !== undefined &&
      (obj.player = message.player ? Player.toJSON(message.player) : undefined);
    return obj;
  },
};

const baseServer_SelfUpdated: object = {};

export const Server_SelfUpdated = {
  encode(
    message: Server_SelfUpdated,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.player !== undefined && message.player !== undefined) {
      OwnPlayer.encode(message.player, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Server_SelfUpdated {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseServer_SelfUpdated } as Server_SelfUpdated;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.player = OwnPlayer.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Server_SelfUpdated {
    const message = { ...baseServer_SelfUpdated } as Server_SelfUpdated;
    if (object.player !== undefined && object.player !== null) {
      message.player = OwnPlayer.fromJSON(object.player);
    }
    return message;
  },

  fromPartial(object: DeepPartial<Server_SelfUpdated>): Server_SelfUpdated {
    const message = { ...baseServer_SelfUpdated } as Server_SelfUpdated;
    if (object.player !== undefined && object.player !== null) {
      message.player = OwnPlayer.fromPartial(object.player);
    }
    return message;
  },

  toJSON(message: Server_SelfUpdated): unknown {
    const obj: any = {};
    message.player !== undefined &&
      (obj.player = message.player
        ? OwnPlayer.toJSON(message.player)
        : undefined);
    return obj;
  },
};

const baseServer_GameUpdated: object = {};

export const Server_GameUpdated = {
  encode(
    message: Server_GameUpdated,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.game !== undefined && message.game !== undefined) {
      Game.encode(message.game, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Server_GameUpdated {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseServer_GameUpdated } as Server_GameUpdated;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.game = Game.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Server_GameUpdated {
    const message = { ...baseServer_GameUpdated } as Server_GameUpdated;
    if (object.game !== undefined && object.game !== null) {
      message.game = Game.fromJSON(object.game);
    }
    return message;
  },

  fromPartial(object: DeepPartial<Server_GameUpdated>): Server_GameUpdated {
    const message = { ...baseServer_GameUpdated } as Server_GameUpdated;
    if (object.game !== undefined && object.game !== null) {
      message.game = Game.fromPartial(object.game);
    }
    return message;
  },

  toJSON(message: Server_GameUpdated): unknown {
    const obj: any = {};
    message.game !== undefined &&
      (obj.game = message.game ? Game.toJSON(message.game) : undefined);
    return obj;
  },
};

const baseServer_PlayerEntered: object = {};

export const Server_PlayerEntered = {
  encode(
    message: Server_PlayerEntered,
    writer: Writer = Writer.create()
  ): Writer {
    if (message.player !== undefined && message.player !== undefined) {
      Player.encode(message.player, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Server_PlayerEntered {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseServer_PlayerEntered } as Server_PlayerEntered;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.player = Player.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Server_PlayerEntered {
    const message = { ...baseServer_PlayerEntered } as Server_PlayerEntered;
    if (object.player !== undefined && object.player !== null) {
      message.player = Player.fromJSON(object.player);
    }
    return message;
  },

  fromPartial(object: DeepPartial<Server_PlayerEntered>): Server_PlayerEntered {
    const message = { ...baseServer_PlayerEntered } as Server_PlayerEntered;
    if (object.player !== undefined && object.player !== null) {
      message.player = Player.fromPartial(object.player);
    }
    return message;
  },

  toJSON(message: Server_PlayerEntered): unknown {
    const obj: any = {};
    message.player !== undefined &&
      (obj.player = message.player ? Player.toJSON(message.player) : undefined);
    return obj;
  },
};

const baseServer_PlayerLeft: object = { playerId: "" };

export const Server_PlayerLeft = {
  encode(message: Server_PlayerLeft, writer: Writer = Writer.create()): Writer {
    writer.uint32(10).string(message.playerId);
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Server_PlayerLeft {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseServer_PlayerLeft } as Server_PlayerLeft;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.playerId = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Server_PlayerLeft {
    const message = { ...baseServer_PlayerLeft } as Server_PlayerLeft;
    if (object.playerId !== undefined && object.playerId !== null) {
      message.playerId = String(object.playerId);
    }
    return message;
  },

  fromPartial(object: DeepPartial<Server_PlayerLeft>): Server_PlayerLeft {
    const message = { ...baseServer_PlayerLeft } as Server_PlayerLeft;
    if (object.playerId !== undefined && object.playerId !== null) {
      message.playerId = object.playerId;
    }
    return message;
  },

  toJSON(message: Server_PlayerLeft): unknown {
    const obj: any = {};
    message.playerId !== undefined && (obj.playerId = message.playerId);
    return obj;
  },
};

const baseServer_GameDeclined: object = {};

export const Server_GameDeclined = {
  encode(_: Server_GameDeclined, writer: Writer = Writer.create()): Writer {
    return writer;
  },

  decode(input: Reader | Uint8Array, length?: number): Server_GameDeclined {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseServer_GameDeclined } as Server_GameDeclined;
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

  fromJSON(_: any): Server_GameDeclined {
    const message = { ...baseServer_GameDeclined } as Server_GameDeclined;
    return message;
  },

  fromPartial(_: DeepPartial<Server_GameDeclined>): Server_GameDeclined {
    const message = { ...baseServer_GameDeclined } as Server_GameDeclined;
    return message;
  },

  toJSON(_: Server_GameDeclined): unknown {
    const obj: any = {};
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
