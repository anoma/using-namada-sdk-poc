<div align="center">
  <h1>Namada SDK usage example</h1>
</div>

**how to run it**

```bash
# at root
./script/build.sh

# now navigate to the web app folder
cd web_app_using_namada_sdk

# install web app dependencies
npm install

# start the web app
npm run start

# visit localhost:3000
```
> :warning: the code has some hard coded values (url of node and the address to be queried) for now 

- [About](#about)
- [Project](#project)
  - [Rust part](#rust-part)
  - [TypeScript utils](#typescript-utils)
  - [React web app](#react-web-app)


## About
Namada SDK makes it easy to use Namada as part of other applications. This project demonstrates the usage from a web application (React, TypeScript).

For now Namada SDK only exist in Rust but in the future it will also be delivered on other platforms such as TypeScript for web as well as Swift and Kotlin for mobile devices.

This project uses the early stage Rust SDK and utilizes it on a web application through WebAssembly.

## Project
There are 3 parts in this project

1. **Rust part** - this defines Namada SDK as a dependency and wraps it to code that mostly just performs, type conversions and error handling
2. **TypeScript utils** - this is a light function that is being made available for Namada SDK for performing network calls
3. **React web app** - This is the example application using the SDK

### Rust part
lives in `/usage-of-namada-sdk/src` 

**TLDR; We create a client that utilizes foreign function created in TypeScript which is using browser's `fetch`. We then pass this clint to any calls we perform with Namada SDK**

The idea is that we use `namada` dependency and from there we can use various calls to contact the chain and fetch data. This is how we query epoch:

```rust

 use namada::sdk::query_epoch;
 use crate::webClient;
 
 let epoch = query_epoch(webClient);
```

Seems easy, but what is the `webClient`. It is a client that we had to develop for our platform (internet browser) specific use case. We can implement it by implementing a structure that implements `namada::ledger::queries::types::Client` trait. note that this trait has a default implementation for `simple_request` so we only have to implement `request`. It looks like this:

```rust
#[async_trait::async_trait(?Send)]
pub trait NamadaClient {
  type Error: From<std::io::Error>;

  async fn simple_request(&self, path: String) -> Result<Vec<u8>, Self::Error> {
      // has a default implementation
  }

  async fn request(
      &self,
      path: String,
      data: Option<Vec<u8>>,
      height: Option<BlockHeight>,
      prove: bool,
  ) -> Result<EncodedResponseQuery, Self::Error>;
}
```
there is an implementation in this project at `src/web_namada/mod.rs`. You will see that it is using foreign functions like this:

```rust
#[wasm_bindgen(raw_module = "./networkingUtils")]
extern "C" {
    #[wasm_bindgen(js_class = "NetworkingUtils")]
    type NetworkingUtils;

    #[wasm_bindgen(constructor)]
    fn new() -> NetworkingUtils;

    #[wasm_bindgen(catch, method, js_class = "NetworkingUtils", js_name = "rpcCall")]
    async fn rpc_call(
        this: &NetworkingUtils,
        path: JsValue,
        prove: bool,
        data: JsValue,
        height: JsValue,
    ) -> Result<JsValue, JsValue>;
}
```
So we need this `rpc_call` function in javascript with the above signature defined above.

In the end what happens as we provide the `client` to Namada SDK it will use our client to perform any calls to the chain with the right data. All of these calls will return serialized and encrypted data. Namada SDK knows what to do with them it is not the task of the consumer of this SDK.

Next part explains how to define and use those foreign functions in JavaScript.

### TypeScript utils
lives in `/javascript_dependencies`

**TLDR; we cerate a TypeScript function that takes a few parameters, performs a network call and returns what ever came from the network.**

As stated in the previous part, there are foreign functions defined in JavaScript. There are several ways of structuring the project when using this and the current is not the best but if is enough to make it work now.

When our Rust code is compiled with wasm-pack tooling, there is some glue code being generated to allow usage Js -> Rust -> Js usage. Our callback function that we defined in the `extern "C" {}` will in some sense become part of that glue code. 

We see that there is this `#[wasm_bindgen(raw_module = "./networkingUtils")]` this indicates where the glue code can import our JavaScript or TypeScript code. This will simply be turned into an import statement in the generated glue code file. Also notice that in our case we are copying the generated glue code as part of the compilation, so we have to ensure that any TypeScript files are also placed so that the import path will be correct.

In our case the files live in `javascript_dependencies` but will be copied over to `web_app_using_namada_sdk/src/utils/namadaSdk` when compiling the Rust code. Note that this file is importing something that is being auto generated during the Rust compilation, so likely the proper way is to just let this live next to the auto generated code and let the auto generated code be checked in to version control. The imports are only valid when the compilation has been done.

Otherwise [wasm-bindgen docs](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-js-imports/index.html) provides more details about how to communicate various details that map the Js/Ts concepts to Rust.

### React web app
lives in `/web_app_using_namada_sdk`

As mentioned before the autogenerated code is being copied over during the compilation process. It is being copied to `web_app_using_namada_sdk/src/utils/namadaSdk`. So we can just import it from there, initialize the wasm and start performing the calls to functions defined in Rust.

```tsx
// we import the files from the location where we copied them
import { init, performRequestFromUi } from "./utils/namadaSdk/index";

useEffect(() => {
  // we have to put this to async as we call async functions
  const asyncBlock = async () => {
    // we have to initiate the wasm binary
    await init();

    // we can perform calls to auto generated functions
    await performRequestFromUi();
  };

  asyncBlock();
}, []);
```