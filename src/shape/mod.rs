pub mod shape;
pub mod primitive;
pub mod area_light;
pub mod geometric_primitive;
pub mod aggregate;

pub use shape::Shape;
pub use area_light::AreaLight;
pub use primitive::Primitive;
pub use geometric_primitive::GeometricPrimitive;

pub mod sphere;
pub use sphere::Sphere;