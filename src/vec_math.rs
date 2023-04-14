use notan::math::Vec3;

pub const PI: f32 = std::f32::consts::PI;
pub const INFINITY: f32 = f32::INFINITY;

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
	Vec3::new(u.y * v.z - u.z * v.y, u.z * v.x - u.x * v.z, u.x * v.y - u.y * v.x)
}

pub fn unit_vector(vec: &Vec3) -> Vec3 {
    *vec / vec.length()
}

pub fn random_vector() -> Vec3 {
	Vec3::new(fastrand::f32(), fastrand::f32(), fastrand::f32())
}

pub fn random_vector_bounded(min: f32, max: f32) -> Vec3 {
	Vec3::new(rand_f32(min, max), rand_f32(min, max), rand_f32(min, max))
}

pub fn rand_f32(min: f32, max: f32) -> f32 {
	fastrand::f32() * (max - min) + min
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
	let in_unit_sphere = random_in_unit_sphere();
	if dot(&in_unit_sphere, normal) > 0. {
		return in_unit_sphere;
	}
	-in_unit_sphere
}

pub fn random_in_unit_sphere() -> Vec3 {
	loop {
		let p = random_vector_bounded(-1., 1.);
		if p.length_squared() >= 1. {
			continue;
		}
		return p;
	}
}

pub fn random_unit_vector() -> Vec3 {
	unit_vector(&random_in_unit_sphere())
}

pub fn near_zero(vec: &Vec3) -> bool {
	let s = 1e-8;
	
	vec.x.abs() < s && vec.y.abs() < s && vec.z.abs() < s
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
	*v - 2. * dot(v, n) * *n
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
	degrees * PI / 180.
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
	if x < min {
		return min;
	} else if x > max {
		return max;
	}
	x
}