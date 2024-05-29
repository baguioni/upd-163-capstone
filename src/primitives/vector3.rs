use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub vec: [f64; 3],
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            vec: [x, y, z],
        }
    }

    pub fn new_empty() -> Self {
        Self::new_with_value(0.0)
    }

    pub fn new_with_value(val: f64) -> Self {
        Self::new(val, val, val)
    }
}

impl Vector3 {
    pub fn dot(self, other: &Vector3) -> f64 {
        self.vec.into_iter().zip(other.vec).fold(0_f64, |acc, elm| acc + (elm.0 * elm.1))
    }

    pub fn cross(self, other: &Vector3) -> Vector3 {
        Self {
            vec: [
                self.vec[1] * other.vec[2] - self.vec[2] * other.vec[1],
                self.vec[2] * other.vec[0] - self.vec[0] * other.vec[2],
                self.vec[0] * other.vec[1] - self.vec[1] * other.vec[0],
            ],
        }
    }

    pub fn len(&self) -> f64 {
        *(&self.dot(&self).sqrt())
    }

    pub fn norm(&self) -> Vector3 { // REVISED: REMOVED .map()
        let mut ans: Vec<f64> = vec![0.0, 0.0, 0.0];
        let vec_len = self.len();

        ans[0] = self.vec[0] / vec_len;
        ans[1] = self.vec[1] / vec_len;
        ans[2] = self.vec[2] / vec_len;

        Self {
            vec: [ans[0], ans[1], ans[2]]
        }
    }
}

impl ops::Add <&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: &Vector3) -> Vector3 {  // REVISED: REMOVED .map()
        let mut ans: Vec <f64> = vec![0.0, 0.0, 0.0];

        // add each element
        ans[0] = self.vec[0] + other.vec[0];
        ans[1] = self.vec[1] + other.vec[1];
        ans[2] = self.vec[2] + other.vec[2];
        
        Self {
            vec: [ans[0], ans[1], ans[2]],
        }
    }
}

impl ops::Sub <&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: &Vector3) -> Vector3 {
        self + &((*other) * -1.0)
    }
}

impl ops::Mul <&Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: &Vector3) -> Vector3 {  // REVISED: REMOVED .map()
        let mut ans: Vec <f64> = vec![0.0, 0.0, 0.0];

        // mul each element
        ans[0] = self.vec[0] * other.vec[0];
        ans[1] = self.vec[1] * other.vec[1];
        ans[2] = self.vec[2] * other.vec[2];
        
        Self {
            vec: [ans[0], ans[1], ans[2]],
        }
    }
}

impl ops::Mul <f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 { // REVISED: REMOVED .map()
        let mut ans: Vec <f64> = vec![0.0, 0.0, 0.0];
        
        // mul each element
        ans[0] = self.vec[0] * other;
        ans[1] = self.vec[1] * other;
        ans[2] = self.vec[2] * other;

        Self {
            vec: [ans[0], ans[1], ans[2]]
        }
    }
}

impl ops::Div <f64> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f64) -> Vector3 {
        self * (1.0 / other)
    }
}

impl ops::Index <usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl ops::IndexMut <usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}