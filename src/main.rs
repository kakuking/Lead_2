use std::time::Instant;
use rand::Rng;

pub mod geometry;
pub mod medium;
pub mod interaction;
pub mod shape;
pub mod shading;
pub mod math;

pub mod common;

use common::*;

fn runtime_test(primitive: &dyn Primitive, name: &str, r: Option<Vector3>) {
    if let Some(ray_d) = r {
        let ray = &mut Ray::init(&Point3::new(0.0, 0.0, 0.0), &ray_d, Some(INFINITY), Some(0.0), None);

        let start = Instant::now();
        let hit = primitive.intersect_p(ray);
        let dur = start.elapsed();

        println!("Time taken by {name}: {:?}", dur);
        if hit {
            println!("ray hits the {name}!");
        } else {
            println!("ray does not hit the {name}!");
        }

        return;
    }

    let mut r0 = Ray::init(&Point3::new(5.0, 0.0, 0.0), &Vector3::new(1.0, 0.0, 0.0), Some(INFINITY), Some(0.0), None);
    let mut r1 = Ray::init(&Point3::new(5.0, 0.0, 0.0), &Vector3::new(-1.0, 0.0, 0.0), Some(INFINITY), Some(0.0), None);

    let start = Instant::now();
    let hits = [primitive.intersect_p(&mut r0), primitive.intersect_p(&mut r1)];
    let dur = start.elapsed();

    println!("Time taken by {name}: {:?}", dur);
    for i in 0..2 {
        if hits[i] {
            println!("r{i} hits the {name}!");
        } else {
            println!("r{i} does not hit the {name}!");
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut primitives: Vec<Arc<GeometricPrimitive>> = Vec::new();
    let mut rng = rand::rng();

    let mut ray_d = Vector3::new(0.0, 0.0, 0.0);
    for _i in 0..500 {
        let x: Float = rng.random();
        let y: Float = rng.random();
        let z: Float = rng.random();
        let r: Float = rng.random();

        let object_to_world = na::Similarity3::new(Vector3::new(x, y, z), Vector3::identity() * 0.0, 1.0);
        let world_to_object = object_to_world.inverse();

        let sph = Sphere::init(
            Arc::from(object_to_world),
            Arc::from(world_to_object),
            false,
            r,
            -r,
            r,
            360.0
        );
        let prim = GeometricPrimitive::init(Arc::from(sph), None, None, None);

        primitives.push(Arc::from(prim));

        ray_d = Vector3::new(x, y, z);
    }

    let mut start = Instant::now(); 
    let mut bvh = BVHAccel::init(255, SplitMethod::SAH);

    for i in 0..primitives.len() {
        bvh.add_primitive(primitives[i].clone());
    }
    println!("About to Build!");
    bvh.build();
    println!("Built!");
    
    let duration = start.elapsed();
    println!("\nTime taken to build BVH: {:?}", duration);

    runtime_test(&bvh, "BVH", Some(ray_d));
    
    start = Instant::now();
    let mut brute = BruteForceAggregate::new();
    for i in 0..primitives.len() {
        brute.add_primitive(primitives[i].clone());
    }
    let duration = start.elapsed(); 
    println!("\nTime taken build brute: {:?}", duration);

    runtime_test(&brute, "Brute Forcing", Some(ray_d));
}
