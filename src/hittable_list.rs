use crate::{hittable::{Sphere, HitRecord}, ray::Ray};

pub struct HittableList {
	pub objects: Vec<Sphere>
}

impl HittableList {
	pub fn new() -> Self {
		let objects = Vec::<Sphere>::new();
		Self {
			objects
		}
	}

	pub fn add(&mut self, object: Sphere) {
		self.objects.push(object);
	}

	pub fn clear(&mut self) {
		self.objects.clear();
	}

	pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
		let mut temp_rec = HitRecord::default();
		let mut hit_anything = false;
		let mut closest_so_far = t_max;

		for object in &self.objects {
			if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
				hit_anything = true;
				closest_so_far = temp_rec.t;
				*rec = temp_rec.clone();
			}
		}

		hit_anything
	}
}
