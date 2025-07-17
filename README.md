# A sample Flipper application in Rust

This project demonstrates writing Flipper applications in Rust using the excellent [`flipperzero`](https://crates.io/crates/flipperzero) crate.

Additionally a GitHub pipeline is setup which builds the application binary, which can be directly dropped onto the SD Card.

## Building

It can be built using the standard stable toolchain:

```sh
cargo +stable build --release
```

Then the `$CARGO_TARGET_DIR/thumbv7em-none-eabihf/release/flipper-my-project` binary needs to be renamed to append the `.fap` extension. Placing the `flipper-my-project.fap` binary somwhere in `/apps` on the SD Card (e.g. to `/apps/Examples/flipper-my-project.fap`) will show it in the file browser.

Sadly, [there is no way](https://github.com/flipperdevices/flipper-application-catalog/issues/799) to have the app in the [built-in App Catalog](https://lab.flipper.net/apps).

## Updating the icon

To update the icon, modify the PNG and run the [following command](https://github.com/flipperzero-rs/flipperzero-rs/blob/main/docs/icons.md#uncompressed):

```sh
(echo -ne '\x00'; magick src/main.png mono:-) > src/main.icon
```

## Running

The examples directory:

![examples](docs/examples-dir.png)

The app item:

![app item](docs/app-item.png)

The screen after opening the app:

![screen](docs/test.png)
