use bevy::prelude::*;
use bevy;
fn main() {
    App::build()
        .add_default_plugins()
        .load_plugin("./examples/app/dynamic_plugin_loading/example_plugin/target/debug/libexample_plugin.dylib")
        .run();
}
