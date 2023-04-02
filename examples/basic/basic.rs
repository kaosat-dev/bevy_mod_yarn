use bevy::prelude::*;
use bevy_mod_yarn::prelude::*;




fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<State>()
        .add_asset::<YarnAsset>()
        .init_asset_loader::<YarnAssetLoader>()
        .add_startup_system(setup)
        .add_system(print_on_load )
        .run();
}

#[derive(Resource, Default)]
struct State {
    handle: Handle<YarnAsset>,
}

fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("micro.yarn");
    // let yarn:Handle<YarnAsset> = asset_server.load("assets/micro.yarn");
}

fn print_on_load(mut state: ResMut<State>, dialogues: Res<Assets<YarnAsset>>,) {
    //let custom_asset = custom_assets.get(&state.handle);
    if let Some(dialogues)= dialogues.get(&state.handle) {
        println!("dialogue loaded {:?}", dialogues)
    }
    // info!("Custom asset loaded: {:?}", custom_asset.unwrap());
    // state.printed = true;
}