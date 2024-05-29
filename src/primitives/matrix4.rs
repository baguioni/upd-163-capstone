use std::{ops};

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    mat: [[f64; 4]; 4],
}

impl Matrix4 {
    pub fn new_empty() -> Self {
        Self::new_with_value(0.0)
    }

    pub fn new_with_value(val: f64) -> Self {
        Self::new(
            val, val, val, val,
            val, val, val, val,
            val, val, val, val,
            val, val, val, val,
        )
    }

    pub fn new_on_diag(val: f64) -> Self {
        Self::new(
            val, 0.0, 0.0, 0.0,
            0.0, val, 0.0, 0.0,
            0.0, 0.0, val, 0.0,
            0.0, 0.0, 0.0, val,
        )
    }

    pub fn new_with_vec3(c0: &Vector3, c1: &Vector3, c2: &Vector3) -> Self {
        Self::new(
            c0[0], c1[0], c2[0], 0.0,
            c0[1], c1[1], c2[1], 0.0,
            c0[2], c1[2], c2[2], 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn new(
        e00: f64, e01: f64, e02: f64, e03: f64,
        e10: f64, e11: f64, e12: f64, e13: f64,
        e20: f64, e21: f64, e22: f64, e23: f64,
        e30: f64, e31: f64, e32: f64, e33: f64,
    ) -> Self {
        Self {
            mat: [
                [e00, e01, e02, e03],
                [e10, e11, e12, e13],
                [e20, e21, e22, e23],
                [e30, e31, e32, e33],
            ],
        }
    }
}

impl Matrix4 {
    pub fn new_translate(new_x: f64, new_y: f64, new_z: f64) -> Self {
        let mut ans = Matrix4::new_on_diag(1.0);

        ans[[0, 3]] = new_x;
        ans[[1, 3]] = new_y;
        ans[[2, 3]] = new_z;

        ans
    }

    pub fn new_scale(dx: f64, dy: f64, dz: f64) -> Self {
        let mut ans = Matrix4::new_on_diag(1.0);

        ans[[0, 0]] = dx;
        ans[[1, 1]] = dy;
        ans[[2, 2]] = dz;

        ans
    }

    pub fn new_rotate(axis: &Vector3, degrees: f64) -> Self {
        let axis_normed = axis.norm();
        let rads = degrees.to_radians();

        let id4 = Matrix4::new_on_diag(1.0);
        let outermat = Matrix4::new(
            axis_normed[0] * axis_normed[0], axis_normed[0] * axis_normed[1], axis_normed[0] * axis_normed[2], 0.0,
            axis_normed[0] * axis_normed[1], axis_normed[1] * axis_normed[1], axis_normed[1] * axis_normed[2], 0.0,
            axis_normed[0] * axis_normed[2], axis_normed[1] * axis_normed[2], axis_normed[2] * axis_normed[2], 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        let crossmat = Matrix4::new(
            0.0, -axis[2], axis[1], 0.0,
            axis[2], 0.0, -axis[0], 0.0,
            -axis[1], axis[0], 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let mut ans = id4 * rads.cos();
        ans = ans + &(outermat * (1.0 - rads.cos()));
        ans = ans + &(crossmat * rads.sin());
        ans[[3, 3]] = 1.0;

        ans
    }
}

impl Matrix4 {
    pub fn mat_invtf_ray(self, my_ray: &Ray) -> Ray {
        let x0 = Vector3::new(self[[0, 0]], self[[1, 0]], self[[2, 0]]);
        let x1 = Vector3::new(self[[0, 1]], self[[1, 1]], self[[2, 1]]);
        let x2 = Vector3::new(self[[0, 2]], self[[1, 2]], self[[2, 2]]);
        let x3 = Vector3::new(self[[0, 3]], self[[1, 3]], self[[2, 3]]);

        let determ = x0.dot(&x1.cross(&x2));
        
        let ans = if determ.abs() > 1e-7 {
            let y0 = x1.cross(&x2);
            let y1 = x2.cross(&x0);
            let y2 = x0.cross(&x1);

            let mut blockmat = Matrix4::new_on_diag(1.0);

            for i in 0..3 {
                blockmat[[0, i]] = y0[i] / determ;
                blockmat[[1, i]] = y1[i] / determ;
                blockmat[[2, i]] = y2[i] / determ;
            }

            let new_ray = Ray {
                position: my_ray.position - &x3,
                direction: my_ray.direction,
            };

            blockmat * &new_ray
        }
        else {
            *my_ray
        };

        ans
    }

    pub fn mat_invtf_point3(self, my_point: &Point3) -> Point3 {
        self.mat_invtf_ray(&Ray::no_direction(my_point)).position
    }

    pub fn mat_invtf_vec3(self, my_vec: &Vector3) -> Vector3 {
        self.mat_invtf_ray(&Ray::from_origin(my_vec)).direction
    }

    pub fn mat_invtf_norm_vec3(self, normed_vec: &Vector3) -> Vector3 {
        let x0 = Vector3::new(self[[0, 0]], self[[1, 0]], self[[2, 0]]);
        let x1 = Vector3::new(self[[0, 1]], self[[1, 1]], self[[2, 1]]);
        let x2 = Vector3::new(self[[0, 2]], self[[1, 2]], self[[2, 2]]);

        let determ = x0.dot(&x1.cross(&x2));
        
        let ans = if determ.abs() > 1e-7 {
            let y0 = x1.cross(&x2);
            let y1 = x2.cross(&x0);
            let y2 = x0.cross(&x1);

            let mut blockmat = Matrix4::new_on_diag(1.0);

            for i in 0..3 {
                blockmat[[i, 0]] = y0[i] / determ;
                blockmat[[i, 1]] = y1[i] / determ;
                blockmat[[i, 2]] = y2[i] / determ;
            }

            blockmat * normed_vec
        }
        else {
            *normed_vec
        };

        ans.norm()
    }
}

impl ops::Add <&Matrix4> for Matrix4 {
    type Output = Matrix4;
    
    fn add(self, other: &Matrix4) -> Matrix4 { // REVISED: REMOVED .map()
        let mut ans: Vec <Vec <f64> > = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ];

        // add each element
        for i in 0..4 {
            for j in 0..4 {
                ans[i][j] = self.mat[i][j] + other.mat[i][j];
            }
        }

        Self {
            mat: [
                [ans[0][0], ans[0][1], ans[0][2], ans[0][3]],
                [ans[1][0], ans[1][1], ans[1][2], ans[1][3]],
                [ans[2][0], ans[2][1], ans[2][2], ans[2][3]],
                [ans[3][0], ans[3][1], ans[3][2], ans[3][3]],
            ],
        }
    }
}

impl ops::Sub <&Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn sub(self, other: &Matrix4) -> Matrix4 {
        self + &(*other * -1.0)
    }
}

impl ops::Mul <&Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: &Matrix4) -> Matrix4 {
        let mut ans = Matrix4::new_empty();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    ans[[i, j]] += self[[i, k]] * other[[k, j]];
                }
            }
        }

        ans
    }
}

impl ops::Mul <&Ray> for Matrix4 {
    type Output = Ray;

    fn mul(self, other: &Ray) -> Ray {
        let mut ans = Ray::new();

        let x4 = self[[3, 0]] * other.position[0]
            + self[[3, 1]] * other.position[1]
            + self[[3, 2]] * other.position[2]
            + self[[3, 3]];

        for i in 0..3 {
            ans.position[i] = 0.0;
            ans.direction[i] = 0.0;

            for k in 0..3 {
                ans.position[i] += self.mat[i][k] * other.position[k];
                ans.direction[i] += self.mat[i][k] * other.direction[k];
            }

            ans.position[i] += self.mat[i][3];
            ans.position[i] /= x4;
        }

        ans
    }
}

impl ops::Mul <&Point3> for Matrix4 {
    type Output = Point3;

    fn mul(self, other: &Point3) -> Point3 {
        let mut ans = Ray::no_direction(other);
        ans = self * &ans;

        ans.position
    }
}

impl ops::Mul <&Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, other: &Vector3) -> Vector3 {
        let mut ans = Ray::from_origin(other);
        ans = self * &ans;

        ans.direction
    }
}


impl ops::Mul <f64> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: f64) -> Matrix4 { // REVISED: REMOVED .map()
        let mut ans: Vec <Vec <f64> > = vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ];

        // mul each element
        for i in 0..4 {
            for j in 0..4 {
                ans[i][j] = self.mat[i][j] * other;
            }
        }

        Self {
            mat: [
                [ans[0][0], ans[0][1], ans[0][2], ans[0][3]],
                [ans[1][0], ans[1][1], ans[1][2], ans[1][3]],
                [ans[2][0], ans[2][1], ans[2][2], ans[2][3]],
                [ans[3][0], ans[3][1], ans[3][2], ans[3][3]],
            ],
        }
    }
}

impl ops::Div <f64> for Matrix4 {
    type Output = Matrix4;

    fn div(self, other: f64) -> Matrix4 {
        self * (1.0 / other)
    }
}

impl ops::Index <[usize; 2]> for Matrix4 {
    type Output = f64;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.mat[index[0]][index[1]]
    }
}

impl ops::IndexMut <[usize; 2]> for Matrix4 {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.mat[index[0]][index[1]]
    }
}