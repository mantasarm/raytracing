use notan::math::Vec3;

use crate::{ray::Ray, vec_math::dot, material::Material};

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
	pub p: Vec3,
	pub normal: Vec3,
	pub material: Material,
	pub t: f32,
	pub front_face:bool
}

impl HitRecord {
	pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
		self.front_face = dot(r.direction(), outward_normal) < 0.;
		
		self.normal = if self.front_face {
			*outward_normal
		} else {
			-*outward_normal
		}
	}
}

pub struct Sphere {
	center: Vec3,
	radius: f32,
	material: Material
}

impl Sphere {
	pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
		Self {
			center,
			radius,
			material
		}
	}

	pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
		let oc = *r.origin() - self.center;
		let a = r.direction().length_squared();
		let half_b = dot(&oc, r.direction());
		let c = oc.length_squared() - self.radius * self.radius;

		let discriminant = half_b * half_b - a * c;
		if discriminant < 0. {
			return false;
		}

		let sqrtd = discriminant.sqrt();

		let mut root = (-half_b - sqrtd) / a;
		if root < t_min || t_max < root {
			root = (-half_b + sqrtd) / a;
			if root < t_min || t_max < root {
				return false;
			}
		}

		rec.t = root;
		rec.p = r.at(rec.t);
		rec.normal = (rec.p - self.center) / self.radius;
		rec.material = self.material;

		true
	}
}