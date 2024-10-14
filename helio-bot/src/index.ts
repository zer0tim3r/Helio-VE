import { HelioClient } from "@/proto/rpc.client";
import { ChannelCredentials } from "@grpc/grpc-js";
import { GrpcTransport } from "@protobuf-ts/grpc-transport";
import {v4} from "uuid";

let gRPC = new HelioClient(new GrpcTransport({
    host: "127.0.0.1:8080",
    channelCredentials: ChannelCredentials.createInsecure(),
}));

(async () => {
    const uuid = "4eb120b9-4483-4a8d-a9b2-f57ab9272699";
    console.log(uuid)

    // var response = await gRPC.createInstance({ 
    //     uuid,
    //     itype: 0,
    //     image: 0,
    //     createdBy: "823554839911989280" 
    // }).then(r => r.response, e => { throw e; });

    // console.log(response)

    // var response = await gRPC.listInstance({
    //     createdBy: "823554839911989280"
    // }).then(r => r.response, e => { throw e; });

    // console.log(response)

    var response = await gRPC.startInstance({
        uuid,
        createdBy: "823554839911989280"
    }).then(r => r.response, e => ({ error: e }));

    console.log(response)

    // var response = await gRPC.deleteInstance({
    //     uuid,
    //     createdBy: "823554839911989280"
    // }).then(r => r.response, e => { throw e; });

    // console.log(response)
})()