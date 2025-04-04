use super::hittable::{HitRecord, Hittable};
use super::interval::Interval;
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Default)]
pub struct Sphere<M> {
  pub center: Vec3,
  pub radius: f64,
  pub material: M,
}

impl<M> Sphere<M>
where
  M: Material,
{
  pub fn new(center: Vec3, radius: f64, material: M) -> Sphere<M> {
    Sphere {
      center,
      radius,
      material,
    }
  }
}

impl<M> Hittable for Sphere<M>
where
  M: Material,
{
  fn hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
    let oc = self.center - ray.origin;
    let a = ray.direction.len_squared();
    let h = ray.direction.dot(oc);
    let c = oc.len_squared() - self.radius.powf(2f64);
    let discriminant = h * h - a * c;
    if discriminant < 0. {
      return None;
    }

    let sqrt_d = discriminant.sqrt();
    let mut root = (h - sqrt_d) / a;
    if !interval.surrounds(root) {
      root = (h + sqrt_d) / a;
      if !interval.surrounds(root) {
        return None;
      }
    }

    let point = ray.at(root);
    Some(HitRecord::new(point, root, (point - self.center) / self.radius, ray, &self.material))
  }
}
