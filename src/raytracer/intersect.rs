use crate::primitives::*;
use super::data::*;
use crate::geometry::*;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option <IntersectData>;
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option <IntersectData> {
        let ray_trans = self.transform.mat_invtf_ray(&ray);
        let ray_trans_direction = ray_trans.direction;
        let ray_trans_position = ray_trans.position;
        let [a, b, c] = self.vertices; // Assignment for convenience
        let tri_norm = (b - &a).cross(&(c - &a));
        let tri_norm_u1 = tri_norm.norm();
        let divisor = ray_trans_direction.dot(&tri_norm_u1);

        // Check intersection point
        if ((divisor * 1000000.0).abs() as isize) < 1 {
            return None;
        }

        let intersect = ((a - &ray_trans_position).dot(&tri_norm_u1)) / divisor;

        if intersect < 1e-7 {
            return None;
        }

        let intersect_pt = ray_trans_position + &(ray_trans_direction * intersect);

        // Check barycentric coordinates. They're always positive
        let norm_a = (c - &b).cross(&(intersect_pt - &b));
        let norm_b = (a - &c).cross(&(intersect_pt - &c));

        let tri_norm_dot = tri_norm.dot(&tri_norm);
        let alpha = tri_norm.dot(&norm_a) / tri_norm_dot;
        let beta = tri_norm.dot(&norm_b) / tri_norm_dot;
        let gamma = 1.0 - alpha - beta;
        
        let a_eps = (alpha * 1000000.0) as isize;
        let b_eps = (beta * 1000000.0) as isize;
        let g_eps = (gamma * 1000000.0) as isize;

        // Check if coordinates satisfy 0 <= a <= 1
        if alpha <= 1.0 && beta <= 1.0 && gamma <= 1.0
            && a_eps >= 0 && b_eps >= 0 && g_eps >= 0 {
            Some(IntersectData {
                index: 0,
                coords: self.transform * &intersect_pt,
                shape: ShapeType::Triangle(*self),
            })
        }
        else {
            None
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option <IntersectData> {
        let transform_mat = self.transform;
        let ray_trans = transform_mat.mat_invtf_ray(&ray);
        let ray_trans_direction = ray_trans.direction;
        let ray_trans_position = ray_trans.position;
        let center_pos = ray_trans.position - &self.center;

        let a = ray_trans_direction.dot(&ray_trans_direction);
        let b = ray_trans_direction.dot(&center_pos) * 2.0;
        let c = center_pos.dot(&center_pos) - (self.radius * self.radius);
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (a * 2.0);
        let t2 = (-b + discriminant.sqrt()) / (a * 2.0);

        if t2 < 0.0 {
            return None;
        }

        let inter_t = if t1 < 0.0 || t2 < t1 {
            t2
        }
        else {
            t1
        };

        Some(IntersectData {
            index: 0,
            coords: transform_mat * &(ray_trans_position + &(ray_trans_direction * inter_t)),
            shape: ShapeType::Sphere(*self),
        })
    }
}

pub fn intersect_scene_from_view(ray: Ray, scene: &Scene) -> Option <IntersectData> {
    let mut min_dist = 1000000.0; // Let initial minimum distance be at infinity (10^6)
    let mut nearest_shape = IntersectData::new();
    let ray_pos = ray.position;

    // Iterate through each object and check lesser t
    for (i, each_shape) in scene.shapes.0.iter().enumerate() {
        match each_shape {
            ShapeType::Sphere(s) => {
                if let Some(mut intersect_data) = s.intersect(&ray) {
                    let vec_dist = (ray_pos - &intersect_data.coords).len();

                    intersect_data.index = i;

                    if vec_dist < min_dist {
                        min_dist = vec_dist;
                        nearest_shape = intersect_data;
                    }
                }
            },
            ShapeType::Triangle(t) => {
                if let Some(mut intersect_data) = t.intersect(&ray) {
                    let vec_dist = (ray_pos - &intersect_data.coords).len();

                    intersect_data.index = i;

                    if vec_dist < min_dist {
                        min_dist = vec_dist;
                        nearest_shape = intersect_data;
                    }
                }
            },
            _ => ()
        }
    }

    if min_dist < 1000000.0 {
        Some(nearest_shape)
    }
    else {
        None
    }
}

pub fn intersect_scene_from_shape(ray: Ray, scene: &Scene, origin: IntersectData) -> Option <IntersectData> {
    let mut min_dist = 1000000.0; // Let initial minimum distance be at infinity (10^6)
    let mut nearest_shape = IntersectData::new();

    // Iterate through each object and check lesser t
    for (i, each_shape) in scene.shapes.0.iter().enumerate() {
        match each_shape {
            ShapeType::Sphere(s) => {
                if let Some(mut intersect_data) = s.intersect(&ray) {
                    let vec_dist = (ray.position - &intersect_data.coords).len();

                    intersect_data.index = i;

                    if matches!(origin.shape, ShapeType::Sphere(_)) && origin.index != intersect_data.index && vec_dist < min_dist {
                        min_dist = vec_dist;
                        nearest_shape = intersect_data;
                    }
                }
            },
            ShapeType::Triangle(t) => {
                if let Some(mut intersect_data) = t.intersect(&ray) {
                    let vec_dist = (ray.position - &intersect_data.coords).len();

                    intersect_data.index = i;

                    if matches!(origin.shape, ShapeType::Triangle(_)) && origin.index != intersect_data.index && vec_dist < min_dist {
                        min_dist = vec_dist;
                        nearest_shape = intersect_data;
                    }
                }
            },
            _ => ()
        }
    }

    if min_dist < 1000000.0 {
        Some(nearest_shape)
    }
    else {
        None
    }
}