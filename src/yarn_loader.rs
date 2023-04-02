use std::collections::HashMap;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::{BoxedFuture},
};

use crate::{prelude::{YarnNode, parse_yarn_nodes_nom, YarnAsset}};
//use serde::Deserialize;

// use crate::gameplay::{YarnNode, parse_yarn_nodes_naive};


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
            let asset = YarnAsset {
                nodes: parse_yarn_nodes_nom(data_str),
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
