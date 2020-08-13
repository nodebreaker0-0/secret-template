# Add contract start


```sh
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked
secretcli tx compute store task2add.wasm --from sentry --gas 1000000 -y
secretcli tx compute instantiate 35 --label test '{}' --from <your key name> --chain-id enigma-pub-testnet-2
secretcli query compute list-code
secretcli query compute list-contract-by-code 34
secretcli tx compute execute secret1pnc0sl87t2vp3w8jgy0lp5l32a7m3w20x3ruk9 '{"add":{"v1":1,"v2":1}}' --from <your key name> --chain-id enigma-pub-testnet-2
secretcli q compute query secret1pnc0sl87t2vp3w8jgy0lp5l32a7m3w20x3ruk9 '{"get_sum":{}}'
```