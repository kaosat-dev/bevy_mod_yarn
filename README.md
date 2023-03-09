# Bevy mod yarn

Parser + helpers for the [YarnSpinner](https://github.com/YarnSpinnerTool/YarnSpinner) dialogue file format for the [Bevy Engine](https://github.com/bevyengine/bevy)
To be able to create narrative games and more in Bevy !

## What this tool does:

- provide a parser(written with Nom). Not specific to Bevy, will likely be extracted into its own crate down the line.
- provide an asset loader for Bevy
- some additional data structures and functions to deal with the Yarn Format inside bevy, in a minimalistic manner

## What this tool does not:

- provide complex UI or predefined ways to interact with the dialogues inside Bevy, for a few reasons
    * Bevy's UI is still constantly evolving
    * Other UI tools for Bevy , like Egui , Belly or Kayak are not "standard"
    * everyone has their preferences


## License

Dual-licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](/LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.


## Compatible Bevy versions

The main branch is compatible with the latest Bevy release, while the branch `bevy_main` tries to track the `main` branch of Bevy (PRs updating the tracked commit are welcome).

Compatibility of `bevy_mod_yarn` versions:
| `bevy_mod_yarn`     | `bevy` |
| :--                 |  :--   |
| `0.1`               | `0.10` |
| `main`              | `0.10` |
| `bevy_main`         | `main` |


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.



[bevy]: https://bevyengine.org/