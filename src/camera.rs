use notan::math::Vec3;

use crate::{ray::Ray, vec_math::{degrees_to_radians, cross, unit_vector}};

#[derive(Clone, Copy)]
pub struct Camera {
	origin: Vec3,
	lower_left_corner: Vec3,
	horizontal: Vec3,
	vertical: Vec3
}

impl Camera {
	pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Self {
		let theta = degrees_to_radians(vfov);
		let h = (theta / 2.).tan();
		let viewport_height = 2. * h;
		let viewport_width = aspect_ratio * viewport_height;
		
	    //let focal_length = 1.0;

		let w = unit_vector(&(lookfrom - lookat));
		let u = unit_vector(&cross(&vup, &w));
		let v = cross(&w, &u);

	    let origin = lookfrom;
	    let horizontal = viewport_width * u;
	    let vertical = viewport_height * v;
	    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - w;

		Self {
			origin,
			lower_left_corner,
			horizontal,
			vertical
		}
	}

	pub fn get_ray(&self, s: f32, t: f32) -> Ray {
		Ray::new(self.origin, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
	}
}
