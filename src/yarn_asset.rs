use std::collections::HashMap;
use bevy::{
    reflect::{TypeUuid, TypePath}, prelude::Asset,
};
use crate::prelude::YarnNode;

#[derive(Asset, Debug, Default, TypeUuid, TypePath, Clone)]
#[uuid = "2ede09ba-8be6-4fe4-8f7a-8a1b3ea96b3b"]
pub struct YarnAsset {
    pub raw: String,
    pub nodes: HashMap<String, YarnNode>
}
