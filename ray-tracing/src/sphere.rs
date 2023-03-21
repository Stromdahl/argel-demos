use super::hittable::HitRecord;

use super::vec3::{Point3, Vec3};
use super::hittable::Hittable;
use super::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut record = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::new(0.0, 0.0, 0.0),
            fron_face: false,
        };
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(r, outward_normal);

        Some(record)
    }
}
