// @ts-ignore: Duplicate identifier

// this file will be copied by script
// this import is available there from the bindgen glue code
import { response_query_serde_to_js_value } from "./usage_of_namada_sdk";

const RPS_PAYLOAD = {
  id: "",
  jsonrpc: "2.0",
  method: "abci_query",
  params: [
    "/shell/value/#atest1v4ehgw36x3prswzxggunzv6pxqmnvdj9xvcyzvpsggeyvs3cg9qnywf589qnwvfsg5erg3fkl09rg5/balance/#atest1d9khqw36xyuyxdf5g4qnjwfkxvc52vf48qu5yseexcm52wpcg56ygd3sxdp52s2z89znsvz93amfph",
    "",
    "0",
    false,
  ],
};

export class NetworkingUtils {
  constructor() {
    console.log("NetworkingUtils.constructor");
  }

  rpcCall = async (
    path: string,
    prove: boolean,
    data: string = "",
    height: string = "0"
  ): Promise<any> => {
    try {
      const rpcRequest = {
        url: "http://127.0.0.1:27657",
        path: path,
        data: data,
      };
      const rpcResponse = await fetch(rpcRequest.url, {
        method: "POST",
        body: JSON.stringify({
          ...RPS_PAYLOAD,
          params: [path, data, height, prove],
        }),
      });
      const rpcResponseData = (await rpcResponse.json()) as {
        result: { response: { info: string; value: string; proof?: string } };
      };
      const response = rpcResponseData.result.response;
      const { info, value, proof } = response;

      // have to turn the borsh encoded data to byte array
      const utf8Encode = new TextEncoder();
      const valueAsByteArray = utf8Encode.encode(value);
      const responseQuery = response_query_serde_to_js_value(
        valueAsByteArray,
        info,
        proof
      );
      return responseQuery;
    } catch {
      return Promise.reject("error while performing the network request");
    }
  };

  rpcCallWithStringifiedJson = async (
    abciQueryPayloadJson: string
  ): Promise<any> => {
    try {
      const rpcResponse = await fetch("http://127.0.0.1:27657", {
        method: "POST",
        body: abciQueryPayloadJson,
      });
      const rpcResponseData = (await rpcResponse.json()) as {
        result: { response: { info: string; value: string; proof?: string } };
      };
      const response = rpcResponseData.result.response;
      const { info, value, proof } = response;

      // have to turn the borsh encoded data to byte array
      const utf8Encode = new TextEncoder();
      const valueAsByteArray = utf8Encode.encode(value);
      const responseQuery = response_query_serde_to_js_value(
        valueAsByteArray,
        info,
        proof
      );
      return responseQuery;
    } catch {
      return Promise.reject("error while performing the network request");
    }
  };

  //   method1 = async (someParameter: number): Promise<string> => {
  //     return `got: ${someParameter} at NetworkingUtils.call`;
  //   };

  //   method2 = async (someParameter: string): Promise<string> => {
  //     return `got: ${someParameter} at NetworkingUtils.call`;
  //   };
}
