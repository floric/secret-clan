/* eslint-disable */
import { Task } from './task';
import { Player } from './player';
import { Game } from './game';
import { Writer, Reader } from 'protobufjs/minimal';


export interface Client {
  auth: Client_Auth | undefined;
}

export interface Client_Auth {
  token: string;
}

export interface Server {
  welcome: Server_Welcome | undefined;
  newTask: Server_NewTask | undefined;
  playerUpdated: Server_PlayerUpdated | undefined;
  gameUpdated: Server_GameUpdated | undefined;
}

export interface Server_Welcome {
}

export interface Server_NewTask {
  task: Task | undefined;
}

export interface Server_PlayerUpdated {
  player: Player | undefined;
}

export interface Server_GameUpdated {
  game: Game | undefined;
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
    if (message.auth !== undefined) {
      Client_Auth.encode(message.auth, writer.uint32(10).fork()).ldelim();
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
          message.auth = Client_Auth.decode(reader, reader.uint32());
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
      message.auth = Client_Auth.fromJSON(object.auth);
    } else {
      message.auth = undefined;
    }
    return message;
  },
  fromPartial(object: DeepPartial<Client>): Client {
    const message = { ...baseClient } as Client;
    if (object.auth !== undefined && object.auth !== null) {
      message.auth = Client_Auth.fromPartial(object.auth);
    } else {
      message.auth = undefined;
    }
    return message;
  },
  toJSON(message: Client): unknown {
    const obj: any = {};
    message.auth !== undefined && (obj.auth = message.auth ? Client_Auth.toJSON(message.auth) : undefined);
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
    } else {
      message.token = "";
    }
    return message;
  },
  fromPartial(object: DeepPartial<Client_Auth>): Client_Auth {
    const message = { ...baseClient_Auth } as Client_Auth;
    if (object.token !== undefined && object.token !== null) {
      message.token = object.token;
    } else {
      message.token = "";
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
    if (message.welcome !== undefined) {
      Server_Welcome.encode(message.welcome, writer.uint32(10).fork()).ldelim();
    }
    if (message.newTask !== undefined) {
      Server_NewTask.encode(message.newTask, writer.uint32(18).fork()).ldelim();
    }
    if (message.playerUpdated !== undefined) {
      Server_PlayerUpdated.encode(message.playerUpdated, writer.uint32(26).fork()).ldelim();
    }
    if (message.gameUpdated !== undefined) {
      Server_GameUpdated.encode(message.gameUpdated, writer.uint32(34).fork()).ldelim();
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
          message.welcome = Server_Welcome.decode(reader, reader.uint32());
          break;
        case 2:
          message.newTask = Server_NewTask.decode(reader, reader.uint32());
          break;
        case 3:
          message.playerUpdated = Server_PlayerUpdated.decode(reader, reader.uint32());
          break;
        case 4:
          message.gameUpdated = Server_GameUpdated.decode(reader, reader.uint32());
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
      message.welcome = Server_Welcome.fromJSON(object.welcome);
    } else {
      message.welcome = undefined;
    }
    if (object.newTask !== undefined && object.newTask !== null) {
      message.newTask = Server_NewTask.fromJSON(object.newTask);
    } else {
      message.newTask = undefined;
    }
    if (object.playerUpdated !== undefined && object.playerUpdated !== null) {
      message.playerUpdated = Server_PlayerUpdated.fromJSON(object.playerUpdated);
    } else {
      message.playerUpdated = undefined;
    }
    if (object.gameUpdated !== undefined && object.gameUpdated !== null) {
      message.gameUpdated = Server_GameUpdated.fromJSON(object.gameUpdated);
    } else {
      message.gameUpdated = undefined;
    }
    return message;
  },
  fromPartial(object: DeepPartial<Server>): Server {
    const message = { ...baseServer } as Server;
    if (object.welcome !== undefined && object.welcome !== null) {
      message.welcome = Server_Welcome.fromPartial(object.welcome);
    } else {
      message.welcome = undefined;
    }
    if (object.newTask !== undefined && object.newTask !== null) {
      message.newTask = Server_NewTask.fromPartial(object.newTask);
    } else {
      message.newTask = undefined;
    }
    if (object.playerUpdated !== undefined && object.playerUpdated !== null) {
      message.playerUpdated = Server_PlayerUpdated.fromPartial(object.playerUpdated);
    } else {
      message.playerUpdated = undefined;
    }
    if (object.gameUpdated !== undefined && object.gameUpdated !== null) {
      message.gameUpdated = Server_GameUpdated.fromPartial(object.gameUpdated);
    } else {
      message.gameUpdated = undefined;
    }
    return message;
  },
  toJSON(message: Server): unknown {
    const obj: any = {};
    message.welcome !== undefined && (obj.welcome = message.welcome ? Server_Welcome.toJSON(message.welcome) : undefined);
    message.newTask !== undefined && (obj.newTask = message.newTask ? Server_NewTask.toJSON(message.newTask) : undefined);
    message.playerUpdated !== undefined && (obj.playerUpdated = message.playerUpdated ? Server_PlayerUpdated.toJSON(message.playerUpdated) : undefined);
    message.gameUpdated !== undefined && (obj.gameUpdated = message.gameUpdated ? Server_GameUpdated.toJSON(message.gameUpdated) : undefined);
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
    } else {
      message.task = undefined;
    }
    return message;
  },
  fromPartial(object: DeepPartial<Server_NewTask>): Server_NewTask {
    const message = { ...baseServer_NewTask } as Server_NewTask;
    if (object.task !== undefined && object.task !== null) {
      message.task = Task.fromPartial(object.task);
    } else {
      message.task = undefined;
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
    } else {
      message.player = undefined;
    }
    return message;
  },
  fromPartial(object: DeepPartial<Server_PlayerUpdated>): Server_PlayerUpdated {
    const message = { ...baseServer_PlayerUpdated } as Server_PlayerUpdated;
    if (object.player !== undefined && object.player !== null) {
      message.player = Player.fromPartial(object.player);
    } else {
      message.player = undefined;
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
    } else {
      message.game = undefined;
    }
    return message;
  },
  fromPartial(object: DeepPartial<Server_GameUpdated>): Server_GameUpdated {
    const message = { ...baseServer_GameUpdated } as Server_GameUpdated;
    if (object.game !== undefined && object.game !== null) {
      message.game = Game.fromPartial(object.game);
    } else {
      message.game = undefined;
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
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;