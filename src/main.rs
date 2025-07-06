use bevy::prelude::*;
use bevy_sandbox::SandboxPlugin;

fn main() -> AppExit {
    App::new().add_plugins(SandboxPlugin).run()
}
