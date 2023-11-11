mod evaluator;
pub mod parser;
mod yarn_asset;
mod yarn_loader;

use bevy::prelude::{App, AssetApp, Plugin};
pub use evaluator::DialogueRunner;
pub use yarn_asset::YarnAsset;
pub use yarn_loader::YarnAssetLoader;

/// A Bevy plugin for yarn dialogue files
///
/// Add this plugin to your Bevy app to get access to
/// the DialogueRunner component
/// ```
/// # use bevy::prelude::*;
/// # use bevy_mod_yarn::prelude::*;
///
/// # use bevy::asset::AssetPlugin;
/// # use bevy::app::AppExit;
///
/// fn main() {
///    App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugin(YarnPlugin)
///         .init_resource::<State>()
///
///         .add_startup_system(setup)
///         .add_system(dialogue_init)
///         .run();
/// }
/// // only needed for manual loading, not when using tools like [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader)
/// #[derive(Resource, Default)]
/// struct State {
///   handle: Handle<YarnAsset>,
///   done: bool
/// }
///
/// fn setup(
/// mut state: ResMut<State>,
/// asset_server: Res<AssetServer>,
///  mut commands: bevy::prelude::Commands
/// ){
///
/// // load the yarn dialogue file
/// state.handle = asset_server.load("dialogues/single_node_simple.yarn");
///
/// // any other bevy setup
/// }
/// // spawn a dialogueRunner
/// fn dialogue_init(mut state: ResMut<State>, dialogues: Res<Assets<YarnAsset>>, mut commands: bevy::prelude::Commands) {
///    if let Some(dialogues)= dialogues.get(&state.handle) {
///      if !state.done {
///       commands.spawn( DialogueRunner::new(dialogues.clone(), "Start"));
///       state.done = true;
///      }
///    }
/// }
///
/// ```

#[derive(Default)]
pub struct YarnPlugin;
impl Plugin for YarnPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<YarnAsset>()
            .init_asset_loader::<YarnAssetLoader>();
    }
}

/// Most commonly used types
pub mod prelude {

    pub use crate::YarnPlugin;
    #[doc(hidden)]
    pub use crate::{
        evaluator::dialogue_runner::*,
        parser::{
            parse_yarn_nodes, statement_choice, statement_command, statement_dialogue, structs::*,
        },
        yarn_asset::YarnAsset,
        yarn_loader::YarnAssetLoader,
    };
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;
