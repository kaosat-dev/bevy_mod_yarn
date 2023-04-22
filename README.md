# Bevy mod yarn

Parser + helpers for the [YarnSpinner](https://github.com/YarnSpinnerTool/YarnSpinner) dialogue file format for the [Bevy Engine](https://github.com/bevyengine/bevy)
To be able to create narrative games and more in Bevy !


## Usage


```rust no_run
use bevy::prelude::*;
use bevy_mod_yarn::prelude::*;

fn main() {
   App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(YarnPlugin)
        
        .add_system(start_background_audio.on_startup())
        .run();
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("background_audio.ogg")).looped();
}
```


## Development status

This projets is still in the early stages, but it is already useable as it is for some basic Yarn scripts.

Since I am using it myself and will be relying on it heavilly (yeah for dogfooding), 

I am aiming to be able to parse as much of the Yarn Syntax as possible: 

- [x] basic nodes parsing (header + body)
- [x] dialogues: with or without character names
- [ ] dialogues: conditional branching with expressions
- [ ] dialogues: interpolated values
- [ ] dialogues: attributes

- [x] choices: blank line to close a list of choices
- [x] choices: nested/ indentation handling 

- [x] commands: basic parsing

- [ ] expressions parsing
- [ ] conditional expressions

- [x] tags parsing
- [ ] tags available inside statements 

- [ ] add testing
- [ ] add examples
- [ ] add docs

I will put it on crates.io once I feel it is useable enough.

## What this tool does:

- provide a [parser](./src/parser/) (written with Nom). Not specific to Bevy, will likely be extracted into its own Crate down the line.
- provide an [asset loader](./src/yarn_loader.rs) for Bevy
- some additional data structures and functions to deal with the Yarn Format inside bevy, in a minimalistic manner

## What this tool does not:

- provide complex UI or predefined ways to interact with the dialogues inside Bevy, for a few reasons
    * Bevy's UI is still constantly evolving
    * Other UI tools for Bevy , like Egui , Belly or Kayak are not "standard"
    * everyone has their preferences

#

## License

Dual-licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](/LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.


## Compatible Bevy versions

The main branch is compatible with the latest Bevy release (0.10.1), while the branch `bevy_main` tries to track the `main` branch of Bevy (PRs updating the tracked commit are welcome).

Compatibility of `bevy_mod_yarn` versions:
| `bevy_mod_yarn`     | `bevy` |
| :--                 |  :--   |
| `0.1`               | `0.10.1` |
| `main`              | `0.10.1` |
| `bevy_main`         | `main` |


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.



[bevy]: https://bevyengine.org/