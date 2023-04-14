use notan::math::Vec3;

use crate::{ray::Ray, hittable::HitRecord, vec_math::{random_in_hemisphere, near_zero, reflect, unit_vector, dot}};

#[derive(Clone, Copy)]
pub enum Material {
	Lambertian(Vec3), Metal(Vec3, f32)
}

impl Material {
	pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
		match self {
		    Material::Lambertian(albedo) => {
				let mut scatter_direction = random_in_hemisphere(&rec.normal);

				if near_zero(&scatter_direction) {
					scatter_direction = rec.normal;
				}
				
				*scattered = Ray::new(rec.p, scatter_direction);
				*attenuation = albedo.to_owned();

				true
			},
		    Material::Metal(albedo, fuzz) => {
				let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
				*scattered = Ray::new(rec.p, reflected + *fuzz * random_in_hemisphere(&rec.normal));
				*attenuation = albedo.to_owned();
				
				dot(scattered.direction(), &rec.normal) > 0.
			},
		}
	}
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian(Vec3::new(1., 1., 1.))
    }
}
