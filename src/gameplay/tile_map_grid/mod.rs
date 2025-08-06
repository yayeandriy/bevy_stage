
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_picking::prelude::*;
use std::fmt::Debug;


const QUADRANT_SIDE_LENGTH: u32 = 64;

#[derive(Component)]
struct ClickableRect {
    clicked: bool,
}

impl Default for ClickableRect {
    fn default() -> Self {
        Self { clicked: false }
    }
}

pub struct TileMapGridPlugin;

impl Plugin for TileMapGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::TileMapGrid), 
                startup
            );
    }
}



fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    info!("TileMapGridPlugin startup function called!");
    // Camera is spawned by DrawingMenuPlugin, no need to spawn another one
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    // In total, there will be `(QUADRANT_SIDE_LENGTH * 2) * (QUADRANT_SIDE_LENGTH * 2)` tiles.
    let map_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH * 2,
        y: QUADRANT_SIDE_LENGTH * 2,
    };
    let quadrant_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH,
        y: QUADRANT_SIDE_LENGTH,
    };

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    fill_tilemap_rect_color(
        TileTextureIndex(5),
        TilePos { x: 0, y: 0 },
        quadrant_size,
        Color::srgba(1.0, 0.0, 0.0, 1.0),
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect_color(
        TileTextureIndex(5),
        TilePos {
            x: QUADRANT_SIDE_LENGTH,
            y: 0,
        },
        quadrant_size,
        Color::srgba(0.0, 1.0, 0.0, 1.0),
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect_color(
        TileTextureIndex(5),
        TilePos {
            x: 0,
            y: QUADRANT_SIDE_LENGTH,
        },
        quadrant_size,
        Color::srgba(0., 0., 0., 1.),
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    fill_tilemap_rect_color(
        TileTextureIndex(5),
        TilePos {
            x: QUADRANT_SIDE_LENGTH,
            y: QUADRANT_SIDE_LENGTH,
        },
        quadrant_size,
        Color::srgba(1.0, 1.0, 0.0, 1.0),
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        map_type: TilemapType::Square,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });
    let width = 100.0;
    let height = 100.0;
    let x_offset = 0.0;
    let y_offset = 0.0;
    
    let sprite_size = Vec2::new(width, height);
     commands
        .spawn((
            Sprite::from_color(Color::BLACK, sprite_size),
            Transform::from_xyz(x_offset, y_offset, 1.0),
            Pickable::default(),
        ))
        .observe(recolor_on::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
        .observe(recolor_on::<Pointer<Out>>(Color::srgb(0.682, 0.506, 0.827)))
        .observe(recolor_on::<Pointer<Click>>(Color::srgb(0.051, 0.573, 0.573)));

}
fn recolor_on<E: Debug + Clone + Reflect>(color: Color) -> impl Fn(Trigger<E>, Query<&mut Sprite>) {
    move |ev, mut sprites| {
        log::info!("MOUSE MOVED");
        let Ok(mut sprite) = sprites.get_mut(ev.target()) else {
            return;
        };
        sprite.color = color;
    }
}

// fn swap_texture_or_hide(
//     asset_server: Res<AssetServer>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<(&mut TilemapTexture, &mut Visibility)>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Space) {
//         let texture_a = TilemapTexture::Single(asset_server.load("tiles.png"));
//         let texture_b = TilemapTexture::Single(asset_server.load("tiles2.png"));
//         for (mut tilemap_tex, _) in &mut query {
//             if *tilemap_tex == texture_a {
//                 *tilemap_tex = texture_b.clone();
//             } else {
//                 *tilemap_tex = texture_a.clone();
//             }
//         }
//     }
//     if keyboard_input.just_pressed(KeyCode::KeyH) {
//         for (_, mut visibility) in &mut query {
//             *visibility = match *visibility {
//                 Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
//                 Visibility::Hidden => Visibility::Visible,
//             };
//         }
//     }
// }
