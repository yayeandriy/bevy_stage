use bevy::prelude::*;

#[derive(Resource)]
pub struct UiAssets {
    pub back: Handle<Image>,
}

impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        
        UiAssets {
            back: asset_server.load("ui/back.png"),
        }
    }
}

pub struct UiAssetsPlugin;

impl Plugin for UiAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiAssets>();
    }
}
