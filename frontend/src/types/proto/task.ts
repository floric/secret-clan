/* eslint-disable */
import { Role } from './role';
import { Writer, Reader } from 'protobufjs/minimal';


export interface Task {
  definition?: { $case: 'settings', settings: Task_Settings } | { $case: 'discloseRole', discloseRole: Task_DiscloseRole } | { $case: 'discuss', discuss: Task_Discuss } | { $case: 'vote', vote: Task_Vote };
}

export interface Task_Settings {
}

export interface Task_DiscloseRole {
  role?: Role;
}

export interface Task_Discuss {
  timeLimit: number;
}

export interface Task_Vote {
}

const baseTask: object = {
};

const baseTask_Settings: object = {
};

const baseTask_DiscloseRole: object = {
};

const baseTask_Discuss: object = {
  timeLimit: 0,
};

const baseTask_Vote: object = {
};

export const protobufPackage = ''

export const Task = {
  encode(message: Task, writer: Writer = Writer.create()): Writer {
    if (message.definition?.$case === 'settings') {
      Task_Settings.encode(message.definition.settings, writer.uint32(10).fork()).ldelim();
    }
    if (message.definition?.$case === 'discloseRole') {
      Task_DiscloseRole.encode(message.definition.discloseRole, writer.uint32(18).fork()).ldelim();
    }
    if (message.definition?.$case === 'discuss') {
      Task_Discuss.encode(message.definition.discuss, writer.uint32(26).fork()).ldelim();
    }
    if (message.definition?.$case === 'vote') {
      Task_Vote.encode(message.definition.vote, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Task {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseTask } as Task;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.definition = {$case: 'settings', settings: Task_Settings.decode(reader, reader.uint32())};
          break;
        case 2:
          message.definition = {$case: 'discloseRole', discloseRole: Task_DiscloseRole.decode(reader, reader.uint32())};
          break;
        case 3:
          message.definition = {$case: 'discuss', discuss: Task_Discuss.decode(reader, reader.uint32())};
          break;
        case 4:
          message.definition = {$case: 'vote', vote: Task_Vote.decode(reader, reader.uint32())};
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },
  fromJSON(object: any): Task {
    const message = { ...baseTask } as Task;
    if (object.settings !== undefined && object.settings !== null) {
      message.definition = {$case: 'settings', settings: Task_Settings.fromJSON(object.settings)};
    }
    if (object.discloseRole !== undefined && object.discloseRole !== null) {
      message.definition = {$case: 'discloseRole', discloseRole: Task_DiscloseRole.fromJSON(object.discloseRole)};
    }
    if (object.discuss !== undefined && object.discuss !== null) {
      message.definition = {$case: 'discuss', discuss: Task_Discuss.fromJSON(object.discuss)};
    }
    if (object.vote !== undefined && object.vote !== null) {
      message.definition = {$case: 'vote', vote: Task_Vote.fromJSON(object.vote)};
    }
    return message;
  },
  fromPartial(object: DeepPartial<Task>): Task {
    const message = { ...baseTask } as Task;
    if (object.definition?.$case === 'settings' && object.definition?.settings !== undefined && object.definition?.settings !== null) {
      message.definition = {$case: 'settings', settings: Task_Settings.fromPartial(object.definition.settings)};
    }
    if (object.definition?.$case === 'discloseRole' && object.definition?.discloseRole !== undefined && object.definition?.discloseRole !== null) {
      message.definition = {$case: 'discloseRole', discloseRole: Task_DiscloseRole.fromPartial(object.definition.discloseRole)};
    }
    if (object.definition?.$case === 'discuss' && object.definition?.discuss !== undefined && object.definition?.discuss !== null) {
      message.definition = {$case: 'discuss', discuss: Task_Discuss.fromPartial(object.definition.discuss)};
    }
    if (object.definition?.$case === 'vote' && object.definition?.vote !== undefined && object.definition?.vote !== null) {
      message.definition = {$case: 'vote', vote: Task_Vote.fromPartial(object.definition.vote)};
    }
    return message;
  },
  toJSON(message: Task): unknown {
    const obj: any = {};
    message.definition?.$case === 'settings' && (obj.settings = message.definition?.settings ? Task_Settings.toJSON(message.definition?.settings) : undefined);
    message.definition?.$case === 'discloseRole' && (obj.discloseRole = message.definition?.discloseRole ? Task_DiscloseRole.toJSON(message.definition?.discloseRole) : undefined);
    message.definition?.$case === 'discuss' && (obj.discuss = message.definition?.discuss ? Task_Discuss.toJSON(message.definition?.discuss) : undefined);
    message.definition?.$case === 'vote' && (obj.vote = message.definition?.vote ? Task_Vote.toJSON(message.definition?.vote) : undefined);
    return obj;
  },
};

export const Task_Settings = {
  encode(_: Task_Settings, writer: Writer = Writer.create()): Writer {
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Task_Settings {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseTask_Settings } as Task_Settings;
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
    const message = { ...baseTask_Settings } as Task_Settings;
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

export const Task_DiscloseRole = {
  encode(message: Task_DiscloseRole, writer: Writer = Writer.create()): Writer {
    if (message.role !== undefined && message.role !== undefined) {
      Role.encode(message.role, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Task_DiscloseRole {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseTask_DiscloseRole } as Task_DiscloseRole;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.role = Role.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },
  fromJSON(object: any): Task_DiscloseRole {
    const message = { ...baseTask_DiscloseRole } as Task_DiscloseRole;
    if (object.role !== undefined && object.role !== null) {
      message.role = Role.fromJSON(object.role);
    }
    return message;
  },
  fromPartial(object: DeepPartial<Task_DiscloseRole>): Task_DiscloseRole {
    const message = { ...baseTask_DiscloseRole } as Task_DiscloseRole;
    if (object.role !== undefined && object.role !== null) {
      message.role = Role.fromPartial(object.role);
    }
    return message;
  },
  toJSON(message: Task_DiscloseRole): unknown {
    const obj: any = {};
    message.role !== undefined && (obj.role = message.role ? Role.toJSON(message.role) : undefined);
    return obj;
  },
};

export const Task_Discuss = {
  encode(message: Task_Discuss, writer: Writer = Writer.create()): Writer {
    writer.uint32(8).uint32(message.timeLimit);
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Task_Discuss {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseTask_Discuss } as Task_Discuss;
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.timeLimit = reader.uint32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },
  fromJSON(object: any): Task_Discuss {
    const message = { ...baseTask_Discuss } as Task_Discuss;
    if (object.timeLimit !== undefined && object.timeLimit !== null) {
      message.timeLimit = Number(object.timeLimit);
    }
    return message;
  },
  fromPartial(object: DeepPartial<Task_Discuss>): Task_Discuss {
    const message = { ...baseTask_Discuss } as Task_Discuss;
    if (object.timeLimit !== undefined && object.timeLimit !== null) {
      message.timeLimit = object.timeLimit;
    }
    return message;
  },
  toJSON(message: Task_Discuss): unknown {
    const obj: any = {};
    message.timeLimit !== undefined && (obj.timeLimit = message.timeLimit);
    return obj;
  },
};

export const Task_Vote = {
  encode(_: Task_Vote, writer: Writer = Writer.create()): Writer {
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Task_Vote {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseTask_Vote } as Task_Vote;
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
  fromJSON(_: any): Task_Vote {
    const message = { ...baseTask_Vote } as Task_Vote;
    return message;
  },
  fromPartial(_: DeepPartial<Task_Vote>): Task_Vote {
    const message = { ...baseTask_Vote } as Task_Vote;
    return message;
  },
  toJSON(_: Task_Vote): unknown {
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
  ? { [K in keyof Omit<T, '$case'>]?: DeepPartial<T[K]> } & { $case: T['$case'] }
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;