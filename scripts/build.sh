pwd
wasm-pack build --target web
# cp -a ./pkg/. ./distributable_wasm
cp -a ./pkg/. ./web_app_using_namada_sdk/src/utils/namadaSdk
cp -a ./javascript_dependencies/networkingUtils.ts ./web_app_using_namada_sdk/src/utils/namadaSdk