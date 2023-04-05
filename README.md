# Bing Lite
A lightweight new Bing (AI chat) desktop application which based on [`Tauri`](https://tauri.app). 

*No more Microsoft Edge, no more Chromium/Electron!* 

## Download
The latest pre-built application in [release page](https://github.com/I-Info/BingLite/releases).

## How to use
1. Prepare:
   - A Microsoft account **with access to new Bing**.
   - Network proxy (necessary in some regions, such as China)
2. Sign in to your Microsoft account.
3. **Reload** the page after logging in.

## Build from source
1. Prepare Tauri dev-env. ([See guide](https://tauri.app/v1/guides/getting-started/prerequisites))
2. Install font-end dependencies
   
    It is recommended to use `pnpm`:
    ```sh
    pnpm install
    ```

3. Build release

    ```sh
    pnpm build
    ```

## Notes
### More application features
IPC access for remote URLs in Tauri is currently not allowed,
waiting for [PR#5918](https://github.com/tauri-apps/tauri/pull/5918) merge. 
If it is necessary, I would try to create a fork.
### Solution for redirecting to `cn.bing.com`
1. Setup network proxies properly.
2. Restart the app.

## Credit
- [dice2o/BingGPT](https://github.com/dice2o/BingGPT)