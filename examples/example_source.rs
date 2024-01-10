mod classes;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ui_dsl::*;
use classes::*;

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(ExampleSource {
            hello: "Hello".to_string(),
            world: "world".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Play)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("load.assets.ron")
                .load_collection::<GameSource>(),
        )
        .add_systems(OnEnter(GameState::Play), setup)
        .run();
}

fn setup(mut commands: Commands, world: &World) {
    commands.spawn(Camera2dBundle::default());
    root(c_root, world, &mut commands, |p| {
        text("Hello, world!", c_example, s_font, p);
    });
}

fn s_font(text_style: &mut TextStyle, game_source: &GameSource) {
    text_style.font = game_source.font.clone();
    text_style.font_size = 40.0;
    text_style.color = Color::WHITE;
}

fn c_example(text: &mut TextBundle, game_source: &GameSource, example_source: &ExampleSource) {
    println!("{} {}", example_source.hello, example_source.world);
    text.style.margin = UiRect::all(Val::Px(10.));
    println!("{:?}", game_source.button);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Play,
}

#[derive(Resource)]
pub struct ExampleSource {
    hello: String,
    world: String,
}

#[derive(AssetCollection, Resource, Debug)]
pub struct GameSource {
    #[asset(key = "button")]
    pub button: Handle<Image>,
    #[asset(key = "font")]
    pub font: Handle<Font>,
}
