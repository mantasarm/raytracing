use std::f32::INFINITY;

use notan::math::Vec3;

use crate::{hittable_list::HittableList, vec_math::unit_vector, hittable::HitRecord};

pub struct Ray {
	orig: Vec3,
	dir: Vec3
}

impl Ray {
	pub fn new(orig: Vec3, dir: Vec3) -> Self {
		Ray {
			orig,
			dir
		}
	}
	
	pub fn at(&self, t: f32) -> Vec3 {
		self.orig + t * self.dir
	}

	pub fn direction(&self) -> &Vec3 {
		&self.dir
	}

	pub fn origin(&self) -> &Vec3 {
		&self.orig
	}
}

pub fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    let mut rec: crate::hittable::HitRecord = HitRecord::default();

    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mut attenuation = Vec3::new(1., 1., 1.);
        if rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1)
        }
        return Vec3::new(0., 0., 0.)
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.3, 0.5, 1.);
}