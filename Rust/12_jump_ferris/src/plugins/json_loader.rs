use std::marker::PhantomData;

use anyhow::Result;
use bevy::utils::BoxedFuture;
use bevy::{
    asset::{Asset, AssetLoader, LoadedAsset},
    prelude::*,
};
use serde::Deserialize;
use serde_json::from_slice;

pub struct JsonPlugin<T>(PhantomData<T>);

struct JsonLoader<T>(PhantomData<T>);

impl<T> Plugin for JsonPlugin<T>
where
    for<'de> T: Deserialize<'de> + Asset,
{
    fn build(&self, app: &mut App) {
        app.add_asset::<T>()
            .add_asset_loader(JsonLoader::<T>(PhantomData));
    }
}

impl<T> Default for JsonPlugin<T>
where
    for<'de> T: Deserialize<'de> + Asset,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> AssetLoader for JsonLoader<T>
where
    for<'de> T: Deserialize<'de> + Asset,
{
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            let asset = from_slice::<T>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
