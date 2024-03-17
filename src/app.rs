mod game_component;
mod particle_bundle;
mod setup;
mod update;

use game_component::GameComponent;
use particle_bundle::ParticleBundle;
pub use setup::make_setup;
pub use update::make_update_system;
