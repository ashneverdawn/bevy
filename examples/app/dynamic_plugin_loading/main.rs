use bevy::prelude::*;
use bevy;
fn main() {
    App::build()
        .add_default_plugins()


        /*
        .add_plugin(bevy::type_registry::TypeRegistryPlugin::default())
        //.load_plugin("./crates/bevy_type_registry/target/debug/.dylib")

        .add_plugin(bevy::core::CorePlugin::default())
        .add_plugin(bevy::transform::TransformPlugin::default())
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
        .add_plugin(bevy::input::InputPlugin::default())
        .add_plugin(bevy::window::WindowPlugin::default())
        .add_plugin(bevy::asset::AssetPlugin::default())
        .add_plugin(bevy::scene::ScenePlugin::default())
        //.add_plugin(bevy::render::RenderPlugin::default())
        //.add_plugin(bevy::sprite::SpritePlugin::default())
        //.add_plugin(bevy::pbr::PbrPlugin::default())
        //.add_plugin(bevy::ui::UiPlugin::default())
        .add_plugin(bevy::text::TextPlugin::default())
        */

        //make sure to build t
        .load_plugin("./examples/app/dynamic_plugin_loading/example_plugin/target/debug/libexample_plugin.dylib")
        .run();
}
