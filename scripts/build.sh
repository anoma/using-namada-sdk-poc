# build Rust code and generate the glue code
(cd usage-of-namada-sdk; wasm-pack build --target web)

# copy the resulting artifacts to the web app project
cp -a ./usage-of-namada-sdk/pkg/. ./web_app_using_namada_sdk/src/utils/namadaSdk

# copy also the JavaScript dependencies
cp -a ./javascript_dependencies/networkingUtils.ts ./web_app_using_namada_sdk/src/utils/namadaSdk