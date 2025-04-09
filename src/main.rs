use std::time::Instant;

pub mod geometry;
pub mod medium;
pub mod interaction;
pub mod shape;
pub mod shading;
pub mod math;

pub mod common;

use common::*;

fn main() {
    println!("Hello, world!");
    let object_to_world = Arc::from(na::Similarity::identity());
    let world_to_object = Arc::from(object_to_world.inverse());
    let sph = Sphere::init(object_to_world, world_to_object, false, 1.0, -5.0, 5.0, 360.0);

    let mut r = Ray::init(&Point3::new(5.0, 0.0, 0.0), &Vector3::new(1.0, 0.0, 0.0), Some(INFINITY), Some(0.0), None);
    let mut r1 = Ray::init(&Point3::new(5.0, 0.0, 0.0), &Vector3::new(-1.0, 0.0, 0.0), Some(INFINITY), Some(0.0), None);

    let hit = sph.intersect_p(&r, false);
    let hit1 = sph.intersect_p(&r1, false);

    if hit {
        println!("Ray 0 hits sphere!!");
    } else {
        println!("Ray 0 does not hit sphere!!");
    }

    if hit1 {
        println!("Ray 1 hits sphere!!");
    } else {
        println!("Ray 1 does not hit sphere!!");
    }

    let mut start = Instant::now(); 
    let mut bvh = BVHAccel::init(255, SplitMethod::SAH);
    let prim = GeometricPrimitive::init(Arc::from(sph), None, None, None);
    bvh.add_primitive(Arc::from(prim));
    bvh.build();
    let duration = start.elapsed(); 

    println!("Time taken to build BVH: {:?}", duration);

    start = Instant::now();
    let bvh_hit: bool = bvh.intersect_p(&mut r);
    let bvh_hit1: bool = bvh.intersect_p(&mut r1);
    let duration = start.elapsed(); 

    println!("Time taken test with BVH: {:?}", duration);

    if bvh_hit {
        println!("Ray 0 hits bvh!!");
    } else {
        println!("Ray 0 does not hit bvh!!");
    }

    if bvh_hit1 {
        println!("Ray 1 hits bvh!!");
    } else {
        println!("Ray 1 does not hit bvh!!");
    }

}
