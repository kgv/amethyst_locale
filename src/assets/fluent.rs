use amethyst_assets::{
    distill_importer::{self, typetag, SerdeImportable},
    register_asset_type, Asset, AssetProcessorSystem, AssetStorage, Handle, LoadHandle,
    ProcessableAsset, ProcessingState, TypeUuid,
};
use amethyst_core::Result;
use fluent::FluentResource;
use serde::{Deserialize, Serialize};
use shrinkwraprs::Shrinkwrap;
use std::sync::Arc;
use unic_langid::LanguageIdentifier;

/// `FluentBundle` representation.
///
/// A collection of resource handles for a single locale.  
/// If locales is empty then it is interlocale bundle.
#[derive(Clone, Debug, Default, Deserialize, SerdeImportable, Serialize, TypeUuid)]
#[uuid = "114026af-0d81-4d92-96da-90855c915345"]
pub struct Bundle {
    pub locales: Vec<LanguageIdentifier>,
    pub resources: Vec<Handle<Resource>>,
}

impl Asset for Bundle {
    type Data = Bundle;

    fn name() -> &'static str {
        "locale::Bundle"
    }
}

register_asset_type!(Bundle => Bundle; AssetProcessorSystem<Bundle>);

/// `FluentResource` wrapper.
#[derive(Debug, Shrinkwrap, TypeUuid)]
#[uuid = "84ec8270-ce02-47e5-a5d2-c5424b76a3c3"]
pub struct Resource(pub Arc<FluentResource>);

impl Asset for Resource {
    type Data = ResourceData;

    fn name() -> &'static str {
        "locale::Resource"
    }
}

impl ProcessableAsset for Resource {
    fn process(
        data: ResourceData,
        _storage: &mut AssetStorage<Resource>,
        _handle: &LoadHandle,
    ) -> Result<ProcessingState<ResourceData, Resource>> {
        let fluent_resource = FluentResource::try_new(String::from_utf8(data.0)?)
            .expect("Failed to create fluent resource.");
        Ok(ProcessingState::Loaded(Resource(Arc::new(fluent_resource))))
    }
}

/// `Resource` internal representation.
#[derive(Clone, Debug, Default, Deserialize, SerdeImportable, Serialize, TypeUuid)]
#[uuid = "f5be909a-2173-47ba-8bc5-09defd301bcc"]
pub struct ResourceData(pub Vec<u8>);

register_asset_type!(ResourceData => Resource; AssetProcessorSystem<Resource>);
