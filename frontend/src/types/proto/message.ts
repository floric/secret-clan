/* eslint-disable */
import { Task } from './task';
import { Player, OwnPlayer } from './player';
import { Game } from './game';
import { Writer, Reader } from 'protobufjs/minimal';


export interface Client {
  message?: { $case: 'authConfirmed', authConfirmed: Client_AuthConfirmed } | { $case: 'nameUpdated', nameUpdated: Client_NameUpdated };
}

export interface Client_AuthConfirmed {
  token: string;
}

export interface Client_NameUpdated {
  name: string;
}

export interface Server {
  message?: { $case: 'newTask', newTask: Server_NewTask } | { $case: 'playerUpdated', playerUpdated: Server_PlayerUpdated } | { $case: 'gameUpdated', gameUpdated: Server_GameUpdated } | { $case: 'selfUpdated', selfUpdated: Server_SelfUpdated };
}

export interface Server_NewTask {
  task?: Task;
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

const baseClient: object = {
};

const baseClient_AuthConfirmed: object = {
  token: "",
};

const baseClient_NameUpdated: object = {
  name: "",
};

const baseServer: object = {
};

const baseServer_NewTask: object = {
};

const baseServer_PlayerUpdated: object = {
};

const baseServer_SelfUpdated: object = {
};

const baseServer_GameUpdated: object = {
};

export const protobufPackage = ''

export const Client = {
  encode(message: Client, writer: Writer = Writer.create()): Writer {
    if (message.message?.$case === 'authConfirmed') {
      Client_AuthConfirmed.encode(message.message.authConfirmed, writer.uint32(10).fork()).ldelim();
    }
    if (message.message?.$case === 'nameUpdated') {
      Client_NameUpdated.encode(message.message.nameUpdated, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Client {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseClient } as Client;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.message = {$case: 'authConfirmed', authConfirmed: Client_AuthConfirmed.decode(reader, reader.uint32())};
          break;
        case 2:
          message.message = {$case: 'nameUpdated', nameUpdated: Client_NameUpdated.decode(reader, reader.uint32())};
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
      message.message = {$case: 'authConfirmed', authConfirmed: Client_AuthConfirmed.fromJSON(object.authConfirmed)};
    }
    if (object.nameUpdated !== undefined && object.nameUpdated !== null) {
      message.message = {$case: 'nameUpdated', nameUpdated: Client_NameUpdated.fromJSON(object.nameUpdated)};
    }
    return message;
  },
  fromPartial(object: DeepPartial<Client>): Client {
    const message = { ...baseClient } as Client;
    if (object.message?.$case === 'authConfirmed' && object.message?.authConfirmed !== undefined && object.message?.authConfirmed !== null) {
      message.message = {$case: 'authConfirmed', authConfirmed: Client_AuthConfirmed.fromPartial(object.message.authConfirmed)};
    }
    if (object.message?.$case === 'nameUpdated' && object.message?.nameUpdated !== undefined && object.message?.nameUpdated !== null) {
      message.message = {$case: 'nameUpdated', nameUpdated: Client_NameUpdated.fromPartial(object.message.nameUpdated)};
    }
    return message;
  },
  toJSON(message: Client): unknown {
    const obj: any = {};
    message.message?.$case === 'authConfirmed' && (obj.authConfirmed = message.message?.authConfirmed ? Client_AuthConfirmed.toJSON(message.message?.authConfirmed) : undefined);
    message.message?.$case === 'nameUpdated' && (obj.nameUpdated = message.message?.nameUpdated ? Client_NameUpdated.toJSON(message.message?.nameUpdated) : undefined);
    return obj;
  },
};

export const Client_AuthConfirmed = {
  encode(message: Client_AuthConfirmed, writer: Writer = Writer.create()): Writer {
    writer.uint32(10).string(message.token);
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Client_AuthConfirmed {
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

export const Client_NameUpdated = {
  encode(message: Client_NameUpdated, writer: Writer = Writer.create()): Writer {
    writer.uint32(10).string(message.name);
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Client_NameUpdated {
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

export const Server = {
  encode(message: Server, writer: Writer = Writer.create()): Writer {
    if (message.message?.$case === 'newTask') {
      Server_NewTask.encode(message.message.newTask, writer.uint32(10).fork()).ldelim();
    }
    if (message.message?.$case === 'playerUpdated') {
      Server_PlayerUpdated.encode(message.message.playerUpdated, writer.uint32(18).fork()).ldelim();
    }
    if (message.message?.$case === 'gameUpdated') {
      Server_GameUpdated.encode(message.message.gameUpdated, writer.uint32(26).fork()).ldelim();
    }
    if (message.message?.$case === 'selfUpdated') {
      Server_SelfUpdated.encode(message.message.selfUpdated, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Server {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseServer } as Server;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.message = {$case: 'newTask', newTask: Server_NewTask.decode(reader, reader.uint32())};
          break;
        case 2:
          message.message = {$case: 'playerUpdated', playerUpdated: Server_PlayerUpdated.decode(reader, reader.uint32())};
          break;
        case 3:
          message.message = {$case: 'gameUpdated', gameUpdated: Server_GameUpdated.decode(reader, reader.uint32())};
          break;
        case 4:
          message.message = {$case: 'selfUpdated', selfUpdated: Server_SelfUpdated.decode(reader, reader.uint32())};
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
    if (object.newTask !== undefined && object.newTask !== null) {
      message.message = {$case: 'newTask', newTask: Server_NewTask.fromJSON(object.newTask)};
    }
    if (object.playerUpdated !== undefined && object.playerUpdated !== null) {
      message.message = {$case: 'playerUpdated', playerUpdated: Server_PlayerUpdated.fromJSON(object.playerUpdated)};
    }
    if (object.gameUpdated !== undefined && object.gameUpdated !== null) {
      message.message = {$case: 'gameUpdated', gameUpdated: Server_GameUpdated.fromJSON(object.gameUpdated)};
    }
    if (object.selfUpdated !== undefined && object.selfUpdated !== null) {
      message.message = {$case: 'selfUpdated', selfUpdated: Server_SelfUpdated.fromJSON(object.selfUpdated)};
    }
    return message;
  },
  fromPartial(object: DeepPartial<Server>): Server {
    const message = { ...baseServer } as Server;
    if (object.message?.$case === 'newTask' && object.message?.newTask !== undefined && object.message?.newTask !== null) {
      message.message = {$case: 'newTask', newTask: Server_NewTask.fromPartial(object.message.newTask)};
    }
    if (object.message?.$case === 'playerUpdated' && object.message?.playerUpdated !== undefined && object.message?.playerUpdated !== null) {
      message.message = {$case: 'playerUpdated', playerUpdated: Server_PlayerUpdated.fromPartial(object.message.playerUpdated)};
    }
    if (object.message?.$case === 'gameUpdated' && object.message?.gameUpdated !== undefined && object.message?.gameUpdated !== null) {
      message.message = {$case: 'gameUpdated', gameUpdated: Server_GameUpdated.fromPartial(object.message.gameUpdated)};
    }
    if (object.message?.$case === 'selfUpdated' && object.message?.selfUpdated !== undefined && object.message?.selfUpdated !== null) {
      message.message = {$case: 'selfUpdated', selfUpdated: Server_SelfUpdated.fromPartial(object.message.selfUpdated)};
    }
    return message;
  },
  toJSON(message: Server): unknown {
    const obj: any = {};
    message.message?.$case === 'newTask' && (obj.newTask = message.message?.newTask ? Server_NewTask.toJSON(message.message?.newTask) : undefined);
    message.message?.$case === 'playerUpdated' && (obj.playerUpdated = message.message?.playerUpdated ? Server_PlayerUpdated.toJSON(message.message?.playerUpdated) : undefined);
    message.message?.$case === 'gameUpdated' && (obj.gameUpdated = message.message?.gameUpdated ? Server_GameUpdated.toJSON(message.message?.gameUpdated) : undefined);
    message.message?.$case === 'selfUpdated' && (obj.selfUpdated = message.message?.selfUpdated ? Server_SelfUpdated.toJSON(message.message?.selfUpdated) : undefined);
    return obj;
  },
};

export const Server_NewTask = {
  encode(message: Server_NewTask, writer: Writer = Writer.create()): Writer {
    if (message.task !== undefined && message.task !== undefined) {
      Task.encode(message.task, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Server_NewTask {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseServer_NewTask } as Server_NewTask;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.task = Task.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },
  fromJSON(object: any): Server_NewTask {
    const message = { ...baseServer_NewTask } as Server_NewTask;
    if (object.task !== undefined && object.task !== null) {
      message.task = Task.fromJSON(object.task);
    }
    return message;
  },
  fromPartial(object: DeepPartial<Server_NewTask>): Server_NewTask {
    const message = { ...baseServer_NewTask } as Server_NewTask;
    if (object.task !== undefined && object.task !== null) {
      message.task = Task.fromPartial(object.task);
    }
    return message;
  },
  toJSON(message: Server_NewTask): unknown {
    const obj: any = {};
    message.task !== undefined && (obj.task = message.task ? Task.toJSON(message.task) : undefined);
    return obj;
  },
};

export const Server_PlayerUpdated = {
  encode(message: Server_PlayerUpdated, writer: Writer = Writer.create()): Writer {
    if (message.player !== undefined && message.player !== undefined) {
      Player.encode(message.player, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Server_PlayerUpdated {
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
    message.player !== undefined && (obj.player = message.player ? Player.toJSON(message.player) : undefined);
    return obj;
  },
};

export const Server_SelfUpdated = {
  encode(message: Server_SelfUpdated, writer: Writer = Writer.create()): Writer {
    if (message.player !== undefined && message.player !== undefined) {
      OwnPlayer.encode(message.player, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Server_SelfUpdated {
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
    message.player !== undefined && (obj.player = message.player ? OwnPlayer.toJSON(message.player) : undefined);
    return obj;
  },
};

export const Server_GameUpdated = {
  encode(message: Server_GameUpdated, writer: Writer = Writer.create()): Writer {
    if (message.game !== undefined && message.game !== undefined) {
      Game.encode(message.game, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Server_GameUpdated {
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
    message.game !== undefined && (obj.game = message.game ? Game.toJSON(message.game) : undefined);
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