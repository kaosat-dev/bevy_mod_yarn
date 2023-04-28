use std::collections::HashMap;
use bevy::{
    reflect::TypeUuid,
};
use crate::prelude::YarnNode;

#[derive(Debug, Default, TypeUuid, Clone)]
#[uuid = "2ede09ba-8be6-4fe4-8f7a-8a1b3ea96b3b"]
pub struct YarnAsset {
    pub raw: String,
    pub nodes: HashMap<String, YarnNode>
}
