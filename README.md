This is modified version of cargo-quill from [Feather repository](https://github.com/feather-rs/feather).

# Installation
Before running cargo-quill, you will [need to change your rustc compiler version to 1.51.0](https://github.com/feather-rs/feather/blob/main/quill/README.md).
```
rustup default 1.51.0
```
Also, you will need a way to compile to WASM.
```
rustup target add wasm32-wasi
```

First, you have to uninstall previous cargo-quill if you have one.
```
cargo uninstall cargo-quill
```
and you need to install cargo-quill from this repo.
```
cargo install --git https://github.com/elginas/cargo-quill
```

# Usage
# Example from Feather repository
Go to plugin directory, for example https://github.com/feather-rs/feather/tree/main/quill/example-plugins/observe-creativemode-flight-event and run
```
cargo-quill build
```
and if building was successful, you should see message similar to this:
```
Wrote plugin file to C:\Users\ginta\Documents\Git\feather\target\wasm32-wasi\debug\observe-creativemode-flight-event.plugin
```
this is location where cargo-quill builded your plugin.

To test the plugin, you will need Feather server. To get one, you will need to clone [Feather repository](https://github.com/feather-rs/feather) and run `cargo build --release`.
Once you have `feather-server.exe`, put it in an empty directory and create `plugins` folder.
Copy `observe-creativemode-flight-event.plugin` to `plugins` folder.

Now you can run `feather-server.exe`.
In console window you should see
```
2021-07-04 14:57:47,728 ←[36mINFO ←[0m [feather_plugin_host::plugin] Loading WebAssembly plugin ObserveCreativemodeFlightEvent version 0.1.0
2021-07-04 14:57:47,904 ←[36mINFO ←[0m [feather_plugin_host::plugin] Enabled plugin ObserveCreativemodeFlightEvent
```

Now launch Minecraft version 1.16 and connect to `localhost:25565`. Once you fly or crouch, you will see text message appear in Minecraft chat.

*Note: I am using Windows. The only difference between operating systems is that your Feather server executable name will vary.*

# Your own plugin (WIP)

First, you need to make your plugin directory, to do this, you need to run:
```
cargo-quill new <name_of_plugin>
```
for example: `cargo-quill new my-amazing-plugin`.

It should generate `src` folder `src/lib.rs` file and `Cargo.toml` file.
Inside of `src/main.rs`, add this code:
```rust
/*
This plugin observers the CreativeFlightEvent printing a msg when someone starts
flying.
*/

use quill::{
    events::{CreativeFlyingEvent, SneakEvent},
    Game, Plugin, Setup,
};

quill::plugin!(FlightPlugin);

struct FlightPlugin {}

impl Plugin for FlightPlugin {
    fn enable(_game: &mut Game, setup: &mut Setup<Self>) -> Self {
        setup.add_system(flight_observer_system);
        setup.add_system(sneak_observer_system);
        FlightPlugin {}
    }

    fn disable(self, _game: &mut Game) {}
}

fn flight_observer_system(_plugin: &mut FlightPlugin, game: &mut Game) {
    for (entity, change) in game.query::<&CreativeFlyingEvent>() {
        if change.is_flying {
            entity.send_message("Enjoy your flight!");
        } else {
            entity.send_message("Hope you enjoyed your flight.");
        }
    }
}

fn sneak_observer_system(_plugin: &mut FlightPlugin, game: &mut Game) {
    for (player, change) in game.query::<&SneakEvent>() {
        if change.is_sneaking {
            player.send_message("Enjoy sneaking!");
        } else {
            player.send_message("How was it to be sneaking?");
        }
    }
}
```
*Note: This tutorial won't explain this code, if you want to learn how to write Quill plugins for Feather server, check [Quill book](https://github.com/Defman/feather/blob/Docs/docs/src/SUMMARY.md). As of writing, this book is work in progress. Alternatively, you can learn from example plugins from [Feather repository](https://github.com/feather-rs/feather/tree/main/quill/example-plugins).*

Run this command to compile plugin and put it into `plugins` directory for Feather server.
```
cargo-quill build --server-path <path_to_your_feather_server_executable>
```
for example: `cargo-quill build --server-path C:\Users\ginta\Documents\feather_env\feather-server.exe`.
Alternatively, you can run this command and copy compiled .plugin file from `target/wasm32-wasi/debug` directory into `plugins` directory near Feather server executable.
```
cargo-quill build
```

Now run your Feather server.
In console window you should see
```
2021-07-04 14:57:47,728 ←[36mINFO ←[0m [feather_plugin_host::plugin] Loading WebAssembly plugin ObserveCreativemodeFlightEvent version 0.1.0
2021-07-04 14:57:47,904 ←[36mINFO ←[0m [feather_plugin_host::plugin] Enabled plugin ObserveCreativemodeFlightEvent
```

Now launch Minecraft version 1.16 and connect to `localhost:25565`. Once you fly or crouch, you will see text message appear in Minecraft chat.

If you've made changes to your plugin, build your plugin and relaunch Feather server.