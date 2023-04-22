
pub mod parser;
mod evaluator;
mod yarn_asset;
mod yarn_loader;



use bevy::prelude::{
    App,Plugin, AddAsset
};
pub use yarn_asset::YarnAsset;
pub use yarn_loader::YarnAssetLoader;
pub use evaluator::DialogueRunner;

/// A Bevy plugin for yarn dialogue files
///
/// Add this plugin to your Bevy app to get access to
/// the DialogueRunner component
/// ```
/// # use bevy::prelude::*;
/// # use bevy_mod_yarn::prelude::*;
/// # use bevy::asset::AssetPlugin;
/// # use bevy::app::AppExit;
/// fn main() {
///    let mut app = App::new();
///    app
///         .add_plugins(MinimalPlugins)
///         .add_plugin(AssetPlugin::default())
///         .add_plugin(YarnPlugin)
///         .add_system(start_dialogue.on_startup());
///    app.run();
/// }
///
/// fn start_dialogue(asset_server: Res<AssetServer>, dialogue: Res<DialogueRunner>) {
///     dialogue.next(asset_server.load("hello_world.yarn"));
/// }
///
/// ```
#[derive(Default)]
pub struct YarnPlugin;
impl Plugin for YarnPlugin {
  fn build(&self, app: &mut App) {
      app
        // .register_type::<DialogueRunner>()
        .add_asset::<YarnAsset>()
        .init_asset_loader::<YarnAssetLoader>()


        /* .add_system_set(ConditionSet::new()
          .run_in_state(AppState::GameRunning)
          .with_system(focusing)
          .into()
        )  */

      ;
  }
}

/// Most commonly used types
pub mod prelude {

    #[doc(hidden)]
    pub use crate::{
        yarn_asset::YarnAsset,
        yarn_loader::YarnAssetLoader,
        parser::{
            structs::*,
            parse_yarn_nodes_nom,
            statement_dialogue,
            statement_choice,
            statement_command
        },
        evaluator::{
            dialogue_runner::*
        },
    };
    pub use crate::{YarnPlugin};

}