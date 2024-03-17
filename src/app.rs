mod game_component;
mod game_resource;
mod particle_bundle;
mod setup;
mod update;

use game_component::GameComponent;
pub use game_resource::GameResource;
use particle_bundle::ParticleBundle;
pub use setup::make_setup;
pub use update::make_update_system;
