// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "rpc.proto" (package "rpc", syntax proto3)
// tslint:disable
import type { RpcTransport } from "@protobuf-ts/runtime-rpc";
import type { ServiceInfo } from "@protobuf-ts/runtime-rpc";
import { Helio } from "./rpc";
import type { StartInstanceArgs } from "./rpc";
import type { DeleteInstanceArgs } from "./rpc";
import type { Empty } from "./google/protobuf/empty";
import type { CreateInstanceArgs } from "./rpc";
import { stackIntercept } from "@protobuf-ts/runtime-rpc";
import type { ListInstanceResult } from "./rpc";
import type { ListInstanceArgs } from "./rpc";
import type { UnaryCall } from "@protobuf-ts/runtime-rpc";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
/**
 * @generated from protobuf service rpc.Helio
 */
export interface IHelioClient {
    /**
     * @generated from protobuf rpc: ListInstance(rpc.ListInstanceArgs) returns (rpc.ListInstanceResult);
     */
    listInstance(input: ListInstanceArgs, options?: RpcOptions): UnaryCall<ListInstanceArgs, ListInstanceResult>;
    /**
     * @generated from protobuf rpc: CreateInstance(rpc.CreateInstanceArgs) returns (google.protobuf.Empty);
     */
    createInstance(input: CreateInstanceArgs, options?: RpcOptions): UnaryCall<CreateInstanceArgs, Empty>;
    /**
     * @generated from protobuf rpc: DeleteInstance(rpc.DeleteInstanceArgs) returns (google.protobuf.Empty);
     */
    deleteInstance(input: DeleteInstanceArgs, options?: RpcOptions): UnaryCall<DeleteInstanceArgs, Empty>;
    /**
     * @generated from protobuf rpc: StartInstance(rpc.StartInstanceArgs) returns (google.protobuf.Empty);
     */
    startInstance(input: StartInstanceArgs, options?: RpcOptions): UnaryCall<StartInstanceArgs, Empty>;
}
/**
 * @generated from protobuf service rpc.Helio
 */
export class HelioClient implements IHelioClient, ServiceInfo {
    typeName = Helio.typeName;
    methods = Helio.methods;
    options = Helio.options;
    constructor(private readonly _transport: RpcTransport) {
    }
    /**
     * @generated from protobuf rpc: ListInstance(rpc.ListInstanceArgs) returns (rpc.ListInstanceResult);
     */
    listInstance(input: ListInstanceArgs, options?: RpcOptions): UnaryCall<ListInstanceArgs, ListInstanceResult> {
        const method = this.methods[0], opt = this._transport.mergeOptions(options);
        return stackIntercept<ListInstanceArgs, ListInstanceResult>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: CreateInstance(rpc.CreateInstanceArgs) returns (google.protobuf.Empty);
     */
    createInstance(input: CreateInstanceArgs, options?: RpcOptions): UnaryCall<CreateInstanceArgs, Empty> {
        const method = this.methods[1], opt = this._transport.mergeOptions(options);
        return stackIntercept<CreateInstanceArgs, Empty>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: DeleteInstance(rpc.DeleteInstanceArgs) returns (google.protobuf.Empty);
     */
    deleteInstance(input: DeleteInstanceArgs, options?: RpcOptions): UnaryCall<DeleteInstanceArgs, Empty> {
        const method = this.methods[2], opt = this._transport.mergeOptions(options);
        return stackIntercept<DeleteInstanceArgs, Empty>("unary", this._transport, method, opt, input);
    }
    /**
     * @generated from protobuf rpc: StartInstance(rpc.StartInstanceArgs) returns (google.protobuf.Empty);
     */
    startInstance(input: StartInstanceArgs, options?: RpcOptions): UnaryCall<StartInstanceArgs, Empty> {
        const method = this.methods[3], opt = this._transport.mergeOptions(options);
        return stackIntercept<StartInstanceArgs, Empty>("unary", this._transport, method, opt, input);
    }
}
