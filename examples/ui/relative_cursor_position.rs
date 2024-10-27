//! Showcases the [`RelativeCursorPosition`] component, used to check the position of the cursor relative to a UI node.

use bevy::{prelude::*, ui::RelativeCursorPosition, window::WindowResized, winit::WinitSettings};
use bevy_render::camera::Viewport;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (set_camera_viewports, relative_cursor_position_system),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Camera {
            // Cursor position will take the viewport offset into account
            viewport: Some(Viewport {
                physical_size: [600, 600].into(),
                ..default()
            }),
            ..default()
        },
    ));

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(250.),
                        height: Val::Px(250.),
                        margin: UiRect::bottom(Val::Px(15.)),
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(235., 35., 12.)),
                ))
                .insert(RelativeCursorPosition::default());

            parent.spawn((
                Text::new("(0.0, 0.0)"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

/// This systems polls the relative cursor position and displays its value in a text component.
fn relative_cursor_position_system(
    relative_cursor_position: Single<&RelativeCursorPosition>,
    output_query: Single<(&mut Text, &mut TextColor)>,
) {
    let (mut output, mut text_color) = output_query.into_inner();

    **output = if let Some(relative_cursor_position) = relative_cursor_position.normalized {
        format!(
            "({:.1}, {:.1})",
            relative_cursor_position.x, relative_cursor_position.y
        )
    } else {
        "unknown".to_string()
    };

    text_color.0 = if relative_cursor_position.mouse_over() {
        Color::srgb(0.1, 0.9, 0.1)
    } else {
        Color::srgb(0.9, 0.1, 0.1)
    };
}

fn set_camera_viewports(
    windows: Query<&Window>,
    mut resize_events: EventReader<WindowResized>,
    mut query: Query<&mut Camera>,
) {
    // We need to dynamically resize the camera's viewports whenever the window size changes
    // so then each camera always takes up half the screen.
    // A resize_event is sent when the window is first created, allowing us to reuse this system for initial setup.
    for resize_event in resize_events.read() {
        let window = windows.get(resize_event.window).unwrap();
        let size = window.physical_size() / 2;

        for mut camera in &mut query {
            camera.viewport = Some(Viewport {
                physical_position: size,
                physical_size: size,
                ..default()
            });
        }
    }
}
