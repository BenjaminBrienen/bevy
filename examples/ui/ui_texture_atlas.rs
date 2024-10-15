//! This example illustrates how to use `TextureAtlases` within ui

use bevy::{color::palettes::css::*, prelude::*, winit::WinitSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, increment_atlas_index)
        .run();
}

fn setup(
    mut commands: Commands<'_, '_>,
    asset_server: Res<'_, AssetServer>,
    mut texture_atlases: ResMut<'_, Assets<TextureAtlasLayout>>,
) {
    // Camera
    commands.spawn(Camera2d);

    let text_font = TextFont::default();

    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(text_font.font_size * 2.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(256.),
                        height: Val::Px(256.),
                        ..default()
                    },
                    image: UiImage::new(texture_handle),
                    background_color: BackgroundColor(ANTIQUE_WHITE.into()),
                    ..default()
                },
                TextureAtlas::from(texture_atlas_handle),
                Outline::new(Val::Px(8.0), Val::ZERO, CRIMSON.into()),
            ));
            parent
                .spawn((Text::new("press "), text_font.clone()))
                .with_child((
                    TextSpan::new("space"),
                    TextColor(YELLOW.into()),
                    text_font.clone(),
                ))
                .with_child((TextSpan::new(" to advance frames"), text_font));
        });
}

fn increment_atlas_index(
    mut atlas_images: Query<'_, '_, &mut TextureAtlas>,
    keyboard: Res<'_, ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut atlas_image in &mut atlas_images {
            atlas_image.index = (atlas_image.index + 1) % 6;
        }
    }
}
