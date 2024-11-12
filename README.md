# Trial task - Simple Web Browser with Tauri 

## Build and run

1. Install dependencies: [`Rust`](https://rustup.rs/),
[`Tauri`](https://v2.tauri.app/v1/guides/getting-started/prerequisites),
and [`yarn`](https://classic.yarnpkg.com/lang/en/docs/install)
2. Install Tauri CLI: `cargo install tauri-cli --version "^2.0.0"`
3. Run `yarn install`
4. Run `cargo tauri dev`

## Updater

Generate public and private keys: 

`cargo tauri signer generate -w ./.tauri/myapp.key`

Set Tauri env var to use the proper private key for signing:

`export TAURI_SIGNING_PRIVATE_KEY=./.tauri/myapp.key`

(Optional) Set the private key password to env for easier builds:

`export TAURI_SIGNING_PRIVATE_KEY_PASSWORD="1234"`

Build and bundle the app:

```bash
cargo tauri build
# `cargo tauri build` already bundles your app, 
# but if you have the app built in release, you can just bundle it instead

# cargo tauri bundle
```

Change the update server and your public key value in
the [`tauri.conf.json`](./src-tauri/tauri.conf.json) configuration file:

```json
"plugins": {
  "updater": {
    "pubkey": "YOUR PUBLIC KEY",
    "endpoints": [
      "https:// YOUR UPDATE SERVER DOMAIN /latest.json"
    ]
  }
}
```

Copy your bundles to the update server:

```bash
# Linux (AppImage)
scp $CARGO_TARGET_DIR/release/bundle/appimage/demo-tauri-browser_VERSION_amd64.* update-server:serve-dir/appimage
```

Fill and create the `latest.json` file on the update server:

```json
{
  "version": "VERSION",
  "platforms": {
    "linux-x86_64": {
      "signature": "https:// YOUR UPDATE SERVER DOMAIN /appimage/demo-tauri-browser_VERSION_amd64.AppImage.sig",
      "url": "https:// YOUR UPDATE SERVER DOMAIN /appimage/demo-tauri-browser_VERSION_amd64.AppImage"
    },
    "windows-x86_64": {
      "signature": "",
      "url": ""
    },
    "darwin-x86_64": {
      "signature": "",
      "url": ""
    }
  }
}
```

```
scp latest.json update-server:serve-dir/latest.json
```