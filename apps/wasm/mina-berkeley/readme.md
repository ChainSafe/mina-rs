### API list

For Chain:
- [x] `initLogger`
- [x] `runProcessor`
- [x] `setApiList`
- [x] `setTrackingAccounts`
- [x] `pollLatestBlocks`
- [x] `getBestChainState`
- [x] `getBestChainStateHash`
- [x] `getSparseMerkleLedger`

For Sending Payment:
- [x] `unlockAccount`
- [x] `sendPayment`
- [x] `lockAccount`

### Prerequisites

- [node (16)](https://nodejs.org/en/download/)
- [yarn (latest)](https://yarnpkg.com/)
- [rust (stable)](https://rustup.rs/)
- [wasm-pack (latest)](https://rustwasm.github.io/wasm-pack/)

### Build

to generate server side npm package
```bash
wasm-pack build -t nodejs -d pkg-node
```

to generate browser side npm package,
```bash
wasm-pack build -t web -d pkg-web
```

### Usage

```
import wasmUrl from "raw:./../../pkg/wasm_bg.wasm";
import init from "~/pkg/wasm";
```

### Web examples
```bash
cd web/
yarn
yarn build
yarn dev
```