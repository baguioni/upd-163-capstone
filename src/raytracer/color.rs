use crate::{primitives::{Vector3, Ray, Point3}, geometry::{RGBColor, LightStack, ShapeType, LightType, PointLight, Material}};

use super::{Scene, IntersectData, Intersectable, intersect_scene_from_shape};

fn get_light_intensity(
    light_dir: Vector3,
    light_color: RGBColor,
    surf_norm: Vector3,
    half_vec: Vector3,
    diffusion: RGBColor,
    specular: RGBColor,
    shininess: f64,
) -> RGBColor {
    let lambert = diffusion * surf_norm.dot(&light_dir).max(0.0);
    let phong = specular * surf_norm.dot(&half_vec).max(0.0).powf(shininess);
    
    (lambert + &phong) * &light_color
}

pub fn get_color_recursive(ray: Ray, scene: &Scene, intersect_pt: IntersectData, now_recurse_depth: usize) -> RGBColor {
    if now_recurse_depth > scene.max_recurse_depth {
        RGBColor::new_empty()
    }
    else {
        // Create reflection ray
		// Transform normals to world space
        let (vec_norm, specular) = match intersect_pt.shape {
            ShapeType::Sphere(s) => {
                // For spheres, get intersection point in object space before finding the normal
                let intersect_obj_space = s.transform.mat_invtf_point3(&intersect_pt.coords);

                (
                    s.transform.mat_invtf_norm_vec3(&(intersect_obj_space - &s.center).norm()),
                    s.material.specular,
                )
            },
            ShapeType::Triangle(t) => {
                // For triangles, we can directly get the normal using the vertices
                (
                    (t.vertices[1] - &t.vertices[0]).cross(&(t.vertices[2] - &t.vertices[0])).norm(),
                    t.material.specular,
                )
            },
            _ => (Vector3::new_empty(), Vector3::new_empty())
        };

        let reflect_ray = Ray {
            position: intersect_pt.coords,
            direction: (ray.direction - &(vec_norm * (2.0 * ray.direction.dot(&vec_norm)))).norm(),
        };

        let new_color = if let Some(intersected_obj) = intersect_scene_from_shape(reflect_ray, scene, intersect_pt) {
            // Recurse here
            let reflect_color = get_color_recursive(reflect_ray, scene, intersected_obj, now_recurse_depth + 1);

            let mut m_light_stack = LightStack::new();
            m_light_stack.lights.push(LightType::Point(PointLight {
                position: intersected_obj.coords,
                color: reflect_color,
            }));

            get_color(ray, scene, intersect_pt, &m_light_stack) * &specular
        }
        else {
            RGBColor::new_empty()
        };

        new_color + &get_color(ray, scene, intersect_pt, &scene.lights)
    }
}

pub fn get_color(ray: Ray, scene: &Scene, intersect_pt: IntersectData, lights: &LightStack) -> RGBColor {
    // Transform normals to world space
    let (vec_norm, material) = match intersect_pt.shape {
        ShapeType::Sphere(s) => {
            // For spheres, get intersection point in object space before finding the normal
            let intersect_obj_space = s.transform.mat_invtf_point3(&intersect_pt.coords);

            (
                s.transform.mat_invtf_norm_vec3(&(intersect_obj_space - &s.center).norm()),
                s.material,
            )
        },
        ShapeType::Triangle(t) => {
            // For triangles, we can directly get the normal using the vertices
            (
                (t.vertices[1] - &t.vertices[0]).cross(&(t.vertices[2] - &t.vertices[0])).norm(),
                t.material,
            )
        },
        _ => (Vector3::new_empty(), Material::new())
    };

    let eye_dir = (ray.position - &intersect_pt.coords).norm();

    let diffspec: &RGBColor = &lights.lights.iter().fold(RGBColor::new_empty(), |acc, ls| {
        match ls {
            LightType::Directional(d) => {
                let light_dir_i = d.direction.norm();
                let half_vec_i = (light_dir_i + &eye_dir).norm();

                acc + &get_light_intensity(light_dir_i, d.color, vec_norm, half_vec_i, material.diffuse, material.specular, material.shininess)
            },
            LightType::Point(p) => {
                let p_position_sub_intersect_pt = p.position - &intersect_pt.coords;
                if !test_shadows(intersect_pt, p.position, scene) {
                    let light_dist = p_position_sub_intersect_pt.len();
                    let attenuation = 1.0 / (lights.attenuation[0] + lights.attenuation[1] * light_dist + lights.attenuation[2] * light_dist * light_dist);

                    let light_dir_i = p_position_sub_intersect_pt.norm();
                    let half_vec_i = (light_dir_i + &eye_dir).norm();

                    acc + &(get_light_intensity(light_dir_i, p.color, vec_norm, half_vec_i, material.diffuse, material.specular, material.shininess) * attenuation)
                }
                else {
                    RGBColor::new_empty()
                }
            }
        }
    });

    *diffspec + &material.ambient + &material.emission
}

// NOTE: Excluding intersected object only works for convex surfaces
fn test_shadows(intersect_pt: IntersectData, light_pos: Point3, scene: &Scene) -> bool {
    let ray = Ray {
        position: intersect_pt.coords,
        direction: (light_pos - &intersect_pt.coords).norm(),
    };

    for (i, each_shape) in scene.shapes.0.iter().enumerate() {
        match each_shape {
            ShapeType::Sphere(d) => {
                if matches!(intersect_pt.shape, ShapeType::Sphere(_)) && intersect_pt.index == i {
                    continue;
                }
                
                if let Some(id) = d.intersect(&ray) {
                    // Check if intersected surface is beyond the light source
                    let id_coord_sub_ray_position = id.coords - &ray.position;
                    let light_pos_sub_ray_position = light_pos - &ray.position;
                    if id_coord_sub_ray_position.dot(&id_coord_sub_ray_position).abs() > light_pos_sub_ray_position.dot(&light_pos_sub_ray_position).abs() {
                        continue;
                    }
                    else {
                        return true;
                    }
                }
            },
            ShapeType::Triangle(t) => {
                if matches!(intersect_pt.shape, ShapeType::Triangle(_)) && intersect_pt.index == i {
                    continue;
                }
                
                if let Some(id) = t.intersect(&ray) {
                    if (id.coords - &ray.position).dot(&(id.coords - &ray.position)).abs() > (light_pos - &ray.position).dot(&(light_pos - &ray.position)).abs() {
                        continue;
                    }
                    else {
                        return true;
                    }
                }
            }
            _ => ()
        }
    }

    false
}