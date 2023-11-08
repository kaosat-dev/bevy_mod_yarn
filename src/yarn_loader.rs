use std::str::Utf8Error;

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    utils::{thiserror, BoxedFuture},
};

use crate::prelude::{parse_yarn_nodes, YarnAsset};

#[derive(Default)]
pub struct YarnAssetLoader;

/// Possible errors that can be produced by [`YarnAssetLoader`]
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum YarnAssetLoaderError {
    ///An [IO](std::io) Error
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not load file: {0}")]
    Utf8(#[from] Utf8Error),
}

impl AssetLoader for YarnAssetLoader {
    type Asset = YarnAsset;
    type Settings = ();
    type Error = YarnAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let data_str = std::str::from_utf8(&bytes)?;
            let asset = YarnAsset {
                nodes: parse_yarn_nodes(data_str),
                raw: data_str.into(),
            };
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["yarn"]
    }
}
