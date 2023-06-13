# Bing Lite
A lightweight new Bing (AI chat) desktop application based on [`Tauri`](https://tauri.app). 

*No more Microsoft Edge, no more Chromium/Electron!* 

![screenshot](https://s2.loli.net/2023/06/13/dcyS5kru6FjfmKs.jpg)

## Download
See the latest build in [release page](https://github.com/I-Info/BingLite/releases).

## How to use
1. Prepare:
   - A Microsoft account.
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
### Warning: "BingLite" is damaged and can’t be opened.
This warning is shown because the build is not signed.
Run the following command to suppress this warning, and reopen the app.
```sh
xattr -r -d com.apple.quarantine /Applications/BingLite.app
```
### Mobile support
Work in progress...
### Solution for redirecting to `cn.bing.com`
1. Setup network proxies properly.
2. Restart the app.

## Credits
- [dice2o/BingGPT](https://github.com/dice2o/BingGPT)