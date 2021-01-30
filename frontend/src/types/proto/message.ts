/* eslint-disable */
import { Task } from './task';
import { Player } from './player';
import { Game } from './game';
import { Writer, Reader } from 'protobufjs/minimal';


export interface Client {
  message?: { $case: 'auth', auth: Client_Auth };
}

export interface Client_Auth {
  token: string;
}

export interface Server {
  message?: { $case: 'welcome', welcome: Server_Welcome } | { $case: 'newTask', newTask: Server_NewTask } | { $case: 'playerUpdated', playerUpdated: Server_PlayerUpdated } | { $case: 'gameUpdated', gameUpdated: Server_GameUpdated };
}

export interface Server_Welcome {
}

export interface Server_NewTask {
  task?: Task;
}

export interface Server_PlayerUpdated {
  player?: Player;
}

export interface Server_GameUpdated {
  game?: Game;
}

const baseClient: object = {
};

const baseClient_Auth: object = {
  token: "",
};

const baseServer: object = {
};

const baseServer_Welcome: object = {
};

const baseServer_NewTask: object = {
};

const baseServer_PlayerUpdated: object = {
};

const baseServer_GameUpdated: object = {
};

export const protobufPackage = ''

export const Client = {
  encode(message: Client, writer: Writer = Writer.create()): Writer {
    if (message.message?.$case === 'auth') {
      Client_Auth.encode(message.message.auth, writer.uint32(10).fork()).ldelim();
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
          message.message = {$case: 'auth', auth: Client_Auth.decode(reader, reader.uint32())};
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
    if (object.auth !== undefined && object.auth !== null) {
      message.message = {$case: 'auth', auth: Client_Auth.fromJSON(object.auth)};
    }
    return message;
  },
  fromPartial(object: DeepPartial<Client>): Client {
    const message = { ...baseClient } as Client;
    if (object.message?.$case === 'auth' && object.message?.auth !== undefined && object.message?.auth !== null) {
      message.message = {$case: 'auth', auth: Client_Auth.fromPartial(object.message.auth)};
    }
    return message;
  },
  toJSON(message: Client): unknown {
    const obj: any = {};
    message.message?.$case === 'auth' && (obj.auth = message.message?.auth ? Client_Auth.toJSON(message.message?.auth) : undefined);
    return obj;
  },
};

export const Client_Auth = {
  encode(message: Client_Auth, writer: Writer = Writer.create()): Writer {
    writer.uint32(10).string(message.token);
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Client_Auth {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseClient_Auth } as Client_Auth;
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
  fromJSON(object: any): Client_Auth {
    const message = { ...baseClient_Auth } as Client_Auth;
    if (object.token !== undefined && object.token !== null) {
      message.token = String(object.token);
    }
    return message;
  },
  fromPartial(object: DeepPartial<Client_Auth>): Client_Auth {
    const message = { ...baseClient_Auth } as Client_Auth;
    if (object.token !== undefined && object.token !== null) {
      message.token = object.token;
    }
    return message;
  },
  toJSON(message: Client_Auth): unknown {
    const obj: any = {};
    message.token !== undefined && (obj.token = message.token);
    return obj;
  },
};

export const Server = {
  encode(message: Server, writer: Writer = Writer.create()): Writer {
    if (message.message?.$case === 'welcome') {
      Server_Welcome.encode(message.message.welcome, writer.uint32(10).fork()).ldelim();
    }
    if (message.message?.$case === 'newTask') {
      Server_NewTask.encode(message.message.newTask, writer.uint32(18).fork()).ldelim();
    }
    if (message.message?.$case === 'playerUpdated') {
      Server_PlayerUpdated.encode(message.message.playerUpdated, writer.uint32(26).fork()).ldelim();
    }
    if (message.message?.$case === 'gameUpdated') {
      Server_GameUpdated.encode(message.message.gameUpdated, writer.uint32(34).fork()).ldelim();
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
          message.message = {$case: 'welcome', welcome: Server_Welcome.decode(reader, reader.uint32())};
          break;
        case 2:
          message.message = {$case: 'newTask', newTask: Server_NewTask.decode(reader, reader.uint32())};
          break;
        case 3:
          message.message = {$case: 'playerUpdated', playerUpdated: Server_PlayerUpdated.decode(reader, reader.uint32())};
          break;
        case 4:
          message.message = {$case: 'gameUpdated', gameUpdated: Server_GameUpdated.decode(reader, reader.uint32())};
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
    if (object.welcome !== undefined && object.welcome !== null) {
      message.message = {$case: 'welcome', welcome: Server_Welcome.fromJSON(object.welcome)};
    }
    if (object.newTask !== undefined && object.newTask !== null) {
      message.message = {$case: 'newTask', newTask: Server_NewTask.fromJSON(object.newTask)};
    }
    if (object.playerUpdated !== undefined && object.playerUpdated !== null) {
      message.message = {$case: 'playerUpdated', playerUpdated: Server_PlayerUpdated.fromJSON(object.playerUpdated)};
    }
    if (object.gameUpdated !== undefined && object.gameUpdated !== null) {
      message.message = {$case: 'gameUpdated', gameUpdated: Server_GameUpdated.fromJSON(object.gameUpdated)};
    }
    return message;
  },
  fromPartial(object: DeepPartial<Server>): Server {
    const message = { ...baseServer } as Server;
    if (object.message?.$case === 'welcome' && object.message?.welcome !== undefined && object.message?.welcome !== null) {
      message.message = {$case: 'welcome', welcome: Server_Welcome.fromPartial(object.message.welcome)};
    }
    if (object.message?.$case === 'newTask' && object.message?.newTask !== undefined && object.message?.newTask !== null) {
      message.message = {$case: 'newTask', newTask: Server_NewTask.fromPartial(object.message.newTask)};
    }
    if (object.message?.$case === 'playerUpdated' && object.message?.playerUpdated !== undefined && object.message?.playerUpdated !== null) {
      message.message = {$case: 'playerUpdated', playerUpdated: Server_PlayerUpdated.fromPartial(object.message.playerUpdated)};
    }
    if (object.message?.$case === 'gameUpdated' && object.message?.gameUpdated !== undefined && object.message?.gameUpdated !== null) {
      message.message = {$case: 'gameUpdated', gameUpdated: Server_GameUpdated.fromPartial(object.message.gameUpdated)};
    }
    return message;
  },
  toJSON(message: Server): unknown {
    const obj: any = {};
    message.message?.$case === 'welcome' && (obj.welcome = message.message?.welcome ? Server_Welcome.toJSON(message.message?.welcome) : undefined);
    message.message?.$case === 'newTask' && (obj.newTask = message.message?.newTask ? Server_NewTask.toJSON(message.message?.newTask) : undefined);
    message.message?.$case === 'playerUpdated' && (obj.playerUpdated = message.message?.playerUpdated ? Server_PlayerUpdated.toJSON(message.message?.playerUpdated) : undefined);
    message.message?.$case === 'gameUpdated' && (obj.gameUpdated = message.message?.gameUpdated ? Server_GameUpdated.toJSON(message.message?.gameUpdated) : undefined);
    return obj;
  },
};

export const Server_Welcome = {
  encode(_: Server_Welcome, writer: Writer = Writer.create()): Writer {
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Server_Welcome {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseServer_Welcome } as Server_Welcome;
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
  fromJSON(_: any): Server_Welcome {
    const message = { ...baseServer_Welcome } as Server_Welcome;
    return message;
  },
  fromPartial(_: DeepPartial<Server_Welcome>): Server_Welcome {
    const message = { ...baseServer_Welcome } as Server_Welcome;
    return message;
  },
  toJSON(_: Server_Welcome): unknown {
    const obj: any = {};
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