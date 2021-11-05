## Commands

### Run mina node
```
cd $MINA_REPO
make libp2p_helper
export MINA_LIBP2P_HELPER_PATH=$PWD/src/app/libp2p_helper/result/bin/libp2p_helper
export DUNE_PROFILE=mainnet
dune build src/app/cli/src/mina.exe --profile=$DUNE_PROFILE
./_build/default/src/app/cli/src/mina.exe daemon \
  --config-file genesis_ledgers/mainnet.json \
  --peer-list-url https://storage.googleapis.com/mina-seed-lists/mainnet_seeds.txt
```

### Run demo
Update const MINA_NODE_ADDRESS in main.go with actual mina node address (local or remote)
```
go mod vendor
go run .
```
