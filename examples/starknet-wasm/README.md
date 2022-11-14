# Example usage of starknet-rs from WASM

This is an example of using `starknet-rs` as an WebAssembly (WASM) module. With WASM you can use `starknet-rs` in browser, as a Node.js module, or really just anywhere WebAssembly is supported.

To run this example, make sure you have [`wasm-pack`](https://github.com/rustwasm/wasm-pack) installed. See installation instructions [here](https://rustwasm.github.io/wasm-pack/installer/).

Check if `wasm-pack` is installed:

```console
$ wasm-pack --version
wasm-pack 0.10.3
```

Then, install the Node.js dependencies:

```console
$ yarn install
```

Build the web app, which uses `starknet-rs` as a WASM module:

```console
$ yarn build
```

Built artifacts should now live in the `./dist/` folder.

Serve the web app using any HTTP server. We use [`serve`](https://www.npmjs.com/package/serve) here (install with `yarn global add serve`):

```console
$ serve -p 3000 ./dist/
```

Access `http://localhost:3000/` and open developer tools. You should see the following printed to the console:

```log
Private Key: 0x03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc
Public Key: 0x077a3b314db07c45076d11f62b6f9e748a39790441823307743cf00d6597ea43
```
