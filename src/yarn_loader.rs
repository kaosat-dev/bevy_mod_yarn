use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::{BoxedFuture, HashMap},
};
use serde::Deserialize;

use crate::gameplay::{YarnNode, parse_yarn_nodes_naive};

#[derive(Debug, TypeUuid)]
#[uuid = "8f061f89-6b4a-407b-96b7-c61cc2e2202b"]
pub struct YarnAsset {
    pub raw: String,
    pub nodes: HashMap<String, YarnNode>
}

#[derive(Default)]
pub struct YarnAssetLoader;

impl AssetLoader for YarnAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let data_str = std::str::from_utf8(bytes)?;
            // parse_yarn_nodes_nom(data_str);
            let asset = YarnAsset {
                nodes: parse_yarn_nodes_naive(data_str),
                raw: data_str.into()
            };
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["yarn"]
    }
}
