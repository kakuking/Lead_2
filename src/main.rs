use std::time::Instant;
use rand::Rng;

pub mod geometry;
pub mod medium;
pub mod interaction;
pub mod shape;
pub mod shading;
pub mod math;
pub mod spectrum;
pub mod camera;

pub mod common;

use common::*;

fn runtime_test(primitive: &dyn Primitive, name: &str, r: Option<Vector3>) {
    if let Some(ray_d) = r {
        let ray = &mut Ray::init(&Point3::new(0.0, 0.0, 0.0), &ray_d, Some(INFINITY), Some(0.0), None);

        let start = Instant::now();
        let hit = primitive.intersect_p(ray);
        let dur = start.elapsed();

        print!("{hit},{:?}{name}", dur.as_micros());
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
    // println!("Hello, world!");

    let mut primitives: Vec<Arc<GeometricPrimitive>> = Vec::new();
    let mut rng = rand::rng();

    let mut sphere_positions: Vec<Vector3> = Vec::new();

    const NUM_SPHERE: usize = 500;

    for _i in 0..NUM_SPHERE {
        let x: Float = rng.random();
        let y: Float = rng.random();
        let z: Float = rng.random();
        let r: Float = rng.random();

        let object_to_world = na::Projective3::identity();
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

        sphere_positions.push(Vector3::new(x, y, z));
    }

    let mut bvh = BVHAccel::init(255, SplitMethod::SAH);

    for i in 0..primitives.len() {
        bvh.add_primitive(primitives[i].clone());
    }
    bvh.build();

    let mut brute = BruteForceAggregate::new();
    for i in 0..primitives.len() {
        brute.add_primitive(primitives[i].clone());
    }

    println!("BVH_hit,BVH_time,Brute_hit,Brute_time");

    let mut ray_d;
    const NUM_TESTS: usize = 1;
    for _ in 0..NUM_TESTS {
        let toss: Float = rng.random();
        if toss < 0.5 {
            let idx: usize = rng.random_range(0..NUM_SPHERE);
            ray_d = sphere_positions[idx];
        } else {
            let x: Float = rng.random();
            let y: Float = rng.random();
            let z: Float = rng.random();
            ray_d = Vector3::new(x, y, z);
        }

        runtime_test(&bvh, ",", Some(ray_d));
        runtime_test(&brute, "\n", Some(ray_d));
    }
}
