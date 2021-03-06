# How it works

In order to connect the web node to the mina network, websocket protocol needs to be enabled. Before that happens, we use a temporary proxy node that connects to mina network via TCP while exposing websocket endpoint to allow inbound connection from the web node.

![arch-uml](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/ChainSafe/mina-rs/main/docs/mvp-arch.iuml)

# Prerequisites

[rust stable](https://rustup.rs/)

[nodejs 16](https://nodejs.org/en/download/)

[go 1.18.x](https://go.dev/dl/)

[yarn](https://classic.yarnpkg.com/en/docs/install)

```
cargo install wasm-pack
```

# Commands

- Start [proxy node](https://github.com/ChainSafe/mina-rs/tree/mvp-proxy/apps/mvp-proxy) (Note: It's on `mvp-proxy` branch)

  ```
  cd apps/mvp-proxy
  go mod vendor
  go run .
  ```

- Build

  ```
  cd apps/wasm/mvp
  yarn
  yarn build
  # Run lint
  yarn lint
  # Regenerate protobuf code
  yarn build:proto
  ```

  Alternatively, this package can be built from root directory of the repo with [workspace manager](https://lerna.js.org/)

  ```
  yarn global add lerna
  lerna bootstrap
  lerna run build
  ```

- Run as static web app

  ```
  cd apps/wasm/mvp
  yarn start
  ```

  Copy & paste peer address that `proxy` prints on console into the web gui at http://localhost:1234/

- Run as chrome extension v3

  open chrome canary, load extension from `dist/ext` folder, then click on the extension icon, it will open the same web gui in a new tab. `dist/ext` can be downloaded from `mvp-chrome-ext` in CI build artifacts as well.

  NOTE: To load web assembly with chrome extension manifect version 3, `unsafe-wasm-eval` is required CSP, which is not yet supported on chrome stable, when you load the extension, it fails with error 'content_security_policy.extension_pages': Insecure CSP value "'wasm-unsafe-eval'" in directive 'script-src'.'. Chrome canary is required for now.

- Run as chrome extension v2 (deprecated)

  open chrome (or edge) stable, load extension from `dist/ext-v2`

- Run as firefox extension

  ```
  yarn global add web-ext
  cd dist/ext-v2
  web-ext run
  ```

  or

  ```
  yarn global add web-ext
  cd dist/ext-v2
  web-ext build
  ```

  And manually load the zipped add-on from firefox.

  `web-ext build` step can be replace by manually zipping the dist/ext-v2 folder, with manifest.json at the root the zip file.

  For more details, please refer to [firefox add-on doc](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Your_first_WebExtension#installing)

  ```
  proxyHost: /ip4/127.0.0.1/tcp/23333/ws/p2p/12D3KooWBJS5MW5tY93UgKvQ9KKzd4s4SRHEZoUsv7frvcAZKaQt
  ```

  Then click on `Connect` button on the web page

- Testing
  ```
  yarn test
  ```

# Notes

- `mvp-proxy` is used as a proxy that exposes websocket endpoint before websocket is enabled for mina nodes. It's a temparory solution, will be replaced with mina graphql API when it's ready.

- `patch.js` is used to post process wasm bundle generated by `wasm-pack` to make current bundler happy, further investigation needed to replace it with some more elegant solution

- RSA identity is [not supported](https://github.com/libp2p/rust-libp2p/blob/a168410dbed0d0941f2e5a14543206044ccb2260/core/src/identity.rs#L70) in wasm build, use Ed25519 or Secp256k1 instead

  ```rust
  #[derive(Clone)]
  pub enum Keypair {
      /// An Ed25519 keypair.
      Ed25519(ed25519::Keypair),
      #[cfg(not(target_arch = "wasm32"))]
      /// An RSA keypair.
      Rsa(rsa::Keypair),
      /// A Secp256k1 keypair.
      #[cfg(feature = "secp256k1")]
      Secp256k1(secp256k1::Keypair),
  }
  ```

# Others

- Chrome extension [examples repository](https://github.com/GoogleChrome/chrome-extensions-samples)
- Chrome extension [api references](https://developer.chrome.com/docs/extensions/reference/)
