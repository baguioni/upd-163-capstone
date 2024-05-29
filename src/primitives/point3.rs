use super::Vector3;

use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Point3 {
    pub point: [f64; 3],
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            point: [x, y, z],
        }
    }

    pub fn new_empty() -> Self {
        Self::new_with_value(0.0)
    }

    pub fn new_with_value(val: f64) -> Self {
        Self::new(val, val, val)
    }

    pub fn pos_rep(self) -> Vector3 {
        Vector3 {
            vec: self.point,
        }
    }
}

impl ops::Add <&Point3> for Point3 {
    type Output = Vector3;

    fn add(self, other: &Point3) -> Vector3 { // REVISED: REMOVED .map()
        let mut ans: Vec <f64> = vec![0.0, 0.0, 0.0];

        // add each element
        ans[0] = self.point[0] + other.point[0];
        ans[1] = self.point[1] + other.point[1];
        ans[2] = self.point[2] + other.point[2];

        Vector3 {
            vec: [ans[0], ans[1], ans[2]]
        }
    }
}

impl ops::Add <&Vector3> for Point3 {
    type Output = Point3;

    fn add(self, other: &Vector3) -> Point3 {
        Point3 {
            point: (self + &(Point3 { point: other.vec })).vec
        }
    }
}

impl ops::Sub <&Point3> for Point3 {
    type Output = Vector3;

    fn sub(self, other: &Point3) -> Vector3 {
        self + &((*other) * -1.0)
    }
}

impl ops::Sub <&Vector3> for Point3 {
    type Output = Point3;

    fn sub(self, other: &Vector3) -> Point3 {
        Point3 {
            point: (self - &(Point3 { point: other.vec })).vec
        }
    }
}

impl ops::Mul <f64> for Point3 {
    type Output = Point3;
    
    fn mul(self, other: f64) -> Point3 { // REVISED: REMOVED .map()
        let mut ans: Vec <f64> = vec![0.0, 0.0, 0.0];
        
        // mul each element
        ans[0] = self.point[0] * other;
        ans[1] = self.point[1] * other;
        ans[2] = self.point[2] * other;

        Self {
            point: [ans[0], ans[1], ans[2]]
        }
    }
}

impl ops::Div <f64> for Point3 {
    type Output = Point3;

    fn div(self, other: f64) -> Point3 {
        self * (1.0 / other)
    }
}

impl ops::Index <usize> for Point3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.point[index]
    }
}

impl ops::IndexMut <usize> for Point3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.point[index]
    }
}