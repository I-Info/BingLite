# Bing Lite
A lightweight new Bing (AI chat) desktop application which based on [`Tauri`](https://tauri.app).

## How to use
1. Prepare:
   - A Microsoft account with access to new Bing.
   - Proxy (If necessary)
2. For the first time, sign in to your Microsoft account.
3. Reload the page after logging in.

## Build from source
1. Prepare `Tauri` dev-env.
[Guide](https://tauri.app/v1/guides/getting-started/prerequisites)
2. Install `tauri-cli`
   
    It is recommended to use `pnpm`:
    ```sh
    pnpm install
    ```

3. Build release

    ```sh
    pnpm tauri build
    ```