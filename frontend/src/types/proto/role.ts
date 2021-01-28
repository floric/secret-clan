/* eslint-disable */
import { Writer, Reader } from 'protobufjs/minimal';


export interface Role {
}

const baseRole: object = {
};

export const protobufPackage = ''

export const Role = {
  encode(_: Role, writer: Writer = Writer.create()): Writer {
    return writer;
  },
  decode(input: Uint8Array | Reader, length?: number): Role {
    const reader = input instanceof Uint8Array ? new Reader(input) : input;
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = { ...baseRole } as Role;
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
  fromJSON(_: any): Role {
    const message = { ...baseRole } as Role;
    return message;
  },
  fromPartial(_: DeepPartial<Role>): Role {
    const message = { ...baseRole } as Role;
    return message;
  },
  toJSON(_: Role): unknown {
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
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;