# Add contract start


```sh
secretcli tx compute instantiate 18 --label test '{}' --from <your key name> --chain-id enigma-pub-testnet-1
secretcli query compute list-code
secretcli query compute list-contract-by-code 18
secretcli tx compute execute secret1urnd3cv9n90ucrauy5rd50ynp7tfehgdu3c949 '{"add":{"v1":1,"v2":1}}' --from validator --chain-id enigma-pub-testnet-1
secretcli q compute query secret1urnd3cv9n90ucrauy5rd50ynp7tfehgdu3c949 '{"get_sum":{}}'
```
