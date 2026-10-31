# Fullbright

See in the dark: toggle full brightness in-game with a key you can rebind.

A native mod for [Horizon](https://github.com/horizon-rust/horizon), a from-scratch Rust rewrite of
Minecraft Java Edition 26.3, built with
[horizon-mixins](https://github.com/horizon-rust/horizon-mixins). Client-side only: it works in
single player, on LAN and on servers, and it never touches your brightness slider, `options.txt` or
Horizon Extras' own full light — turning it off gives your own brightness right back.

## Features

- **Toggle key** — press it in a world to switch full brightness on or off. Defaults to
  **`B`** (`key.keyboard.b`); rebind it in Options → Mods → Fullbright, and the choice is kept.
- **Two modes**, picked on the mod's settings page (Options → Mods → Fullbright):
  - **Full light** — every lightmap entry fully lit, the way Horizon Extras' full light works.
  - **Gamma** — the video slider's gamma past its 100% "Bright" limit, at a strength of your
    choice: 100% up to 1500% (the classic gamma-15 fullbright), cycled on the settings page.
- **Remembered between sessions** — the toggle, the mode, the strength and the key come back as
  you left them.

## Building

Native mods are Rust code compiled into the game. With the Horizon launcher (0.2.23+) and native
mods enabled (Settings → Game → Native mods; the first build downloads Rust and takes a while):

```sh
horizon-launcher native-build /path/to/fullbright/mixins/fullbright --trust-native-code
```

To check the mod against Horizon's source first (from a clone of
[horizon-rust/horizon-mods](https://github.com/horizon-rust/horizon-mods), where the tool lives):

```sh
python3 native/tools/native.py check /path/to/fullbright
```

The mod's own logic tests run with plain Cargo:

```sh
cargo test
```

## How it works

The bundle hooks five of Horizon's official native hook points (hooks 1), so it keeps building on
future versions:

| Hook | What it does |
| --- | --- |
| `native_frame` | Each frame: gives the renderer a full-light lightmap or an over-slider gamma while on, and the player's own brightness otherwise. |
| `native_key` | Toggles on the key you chose (press only, while playing — typing the key in chat never toggles). |
| `native_mods` | Lists the mod and its rebindable key in Options → Mods. |
| `native_settings` / `native_settings_click` | The settings page's three buttons. |

The state lives in `mixins/fullbright/runtime.rs` (also the Cargo library, with the tests); the
hook helpers are the other files. Settings are kept one file per key under
`horizon-mods/.horizon-mod-data/fullbright/`, where Horizon's mod settings live.

## Licence

[MIT](LICENSE)
