use crate::{assets::Resource, BundleAsset};
use amethyst_assets::{AssetStorage, Handle};
use amethyst_core::ecs::{systems::Fetch, Resources};
use fluent::{FluentBundle, FluentResource};
use log::warn;
use shrinkwraprs::Shrinkwrap;
use std::sync::Arc;

/// `FluentBundle` wrapper.
#[derive(Shrinkwrap)]
pub struct Bundle(FluentBundle<Arc<FluentResource>>);

impl Bundle {
    pub fn builder(resources: &Resources) -> Builder {
        Builder::new(resources)
    }
}

/// Bundle resource builder.
pub struct Builder<'a> {
    bundle_storage: Fetch<'a, AssetStorage<BundleAsset>>,
    resource_storage: Fetch<'a, AssetStorage<Resource>>,
    fluent_bundle: Option<FluentBundle<Arc<FluentResource>>>,
}

impl<'a> Builder<'a> {
    pub fn new(resources: &'a Resources) -> Self {
        let bundle_storage = resources
            .get::<AssetStorage<BundleAsset>>()
            .expect("Get `Bundle` storage resource.");
        let resource_storage = resources
            .get::<AssetStorage<Resource>>()
            .expect("Get `Resource` storage resource.");
        let fluent_bundle = Some(FluentBundle::new(Vec::new()));
        Self {
            bundle_storage,
            resource_storage,
            fluent_bundle,
        }
    }

    pub fn with(self, handle: &Handle<BundleAsset>) -> Self {
        let Self {
            bundle_storage,
            resource_storage,
            mut fluent_bundle,
        } = self;
        fluent_bundle.as_mut().map(|fluent_bundle| {
            let bundle = bundle_storage.get(handle)?;
            fluent_bundle.locales.extend_from_slice(&bundle.locales);
            for handle in &bundle.resources {
                let resource = resource_storage.get(handle)?;
                if let Err(error) = fluent_bundle.add_resource(resource.0.clone()) {
                    warn!("Fluent resource overriding: {:?}.", error);
                }
            }
            Some(fluent_bundle)
        });
        Self {
            bundle_storage,
            resource_storage,
            fluent_bundle,
        }
    }

    pub fn build(self) -> Option<Bundle> {
        self.fluent_bundle.map(Bundle)
    }
}
