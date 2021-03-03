pub use self::{
    asset::FtlFormat,
    assets::{Bundle as BundleAsset, Resource as ResourceAsset},
    resources::Bundle as BundleResource,
};

pub mod asset;
pub mod assets;
pub mod resources;

#[cfg(test)]
mod tests {
    use amethyst_assets::Handle;
    use fluent::FluentBundle;
    use super::*;

    #[test]
    fn it_works() {
        let bundle_resource = BundleResource::builder(&Default::default()).build();
        // assert_eq!(bundle_resource, Some(FluentBundle::new(Vec::new()))); 
    }
}
