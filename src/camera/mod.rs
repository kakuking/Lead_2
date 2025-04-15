pub mod camera;
pub mod film;
pub mod projective;

pub use camera::Camera;
pub use projective::ProjectiveCamera;
pub use film::Film;

// You can add your own implementations here
pub mod orthographic;
pub mod perspective;