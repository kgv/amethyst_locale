use crate::assets::fluent::ResourceData;
use amethyst_assets::{Format, TypeUuid};
use amethyst_core::Result;
use serde::{Deserialize, Serialize};

// https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md
/// Loads content from fluent files.
#[derive(Clone, Debug, Default, Deserialize, Serialize, TypeUuid)]
#[uuid = "fb447f99-90a2-42dc-b376-40edba0f851f"]
pub struct Ftl;

impl Format<ResourceData> for Ftl {
    fn name(&self) -> &'static str {
        "FTL"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<ResourceData> {
        Ok(ResourceData(bytes))
    }
}

amethyst_assets::register_importer!(".ftl", Ftl);
