use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin{
                    primary_window : Some(Window {
                        title: String :: from("Flappy_Bird"),
                        position : WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2 :: new(512.,512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
        )
        .run();
}
