# Add contract start


```sh
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked
secretcli tx compute store task2add.wasm --from sentry --gas 1000000 -y
secretcli tx compute instantiate 34 --label test '{}' --from <your key name> --chain-id enigma-pub-testnet-2
secretcli query compute list-code
secretcli query compute list-contract-by-code 34
secretcli tx compute execute secret13xeyzx73sgsg30ltz8qq0a4fuyp9x2zn2tttam '{"add":{"v1":1,"v2":1}}' --from <your key name> --chain-id enigma-pub-testnet-2
secretcli q compute query secret13xeyzx73sgsg30ltz8qq0a4fuyp9x2zn2tttam '{"get_sum":{}}'
```