use std::ops::{
	Neg, 
	Index, 
	AddAssign, 
	IndexMut, 
	MulAssign, 
	DivAssign, 
	Add,
	Sub,
	Mul,
	Div
};
use std::string::ToString;

pub struct Vec3 {
    e : [f64; 3]
}

impl Vec3 {
	pub fn zero() -> Vec3 {
		Vec3 {e: [0.0, 0.0, 0.0]}
	}

	pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
		Vec3 {e: [x, y, z]}
	}

	pub fn x(&self) -> f64 {
		self.e[0]
	}

	pub fn y(&self) -> f64 {
		self.e[1]
	}

	pub fn z(&self) -> f64 {
		self.e[2]
	}
}

impl Neg for &Vec3 {
	type Output = Vec3;
	fn neg(self) -> Self::Output {
		Vec3 { e: [
			-self.e[0],
			-self.e[1],
			-self.e[2]
		] }
	}
}
enum AxisIndex {
	X,
	Y,
	Z
}

impl Index<AxisIndex> for Vec3 {
	type Output = f64;
	fn index(&self, color: AxisIndex) -> &Self::Output {
		match color {
			AxisIndex::X => &self.e[0],
			AxisIndex::Y => &self.e[1],
			AxisIndex::Z => &self.e[2],
		}
	}
}

impl IndexMut<AxisIndex> for Vec3 {
	fn index_mut(&mut self, color: AxisIndex) -> &mut f64 {
		match color {
			AxisIndex::X => &mut self.e[0],
			AxisIndex::Y => &mut self.e[1],
			AxisIndex::Z => &mut self.e[2],
		}
	}
}

impl AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Self){
		self.e[0] += rhs.e[0];
		self.e[1] += rhs.e[1];
		self.e[2] += rhs.e[2];
	}
}

impl MulAssign<f64> for Vec3 {
	fn mul_assign(&mut self, rhs: f64) {
		self.e[0] *= rhs;
		self.e[1] *= rhs;
		self.e[2] *= rhs;
	}
}

impl DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, rhs: f64) {
		self.e[0] /= rhs;
		self.e[1] /= rhs;
		self.e[2] /= rhs;
	}
}

impl Vec3 {
	fn length_squared(&self) -> f64 {
		self.e[0] * self.e[0] +
		self.e[1] * self.e[1] +
		self.e[2] * self.e[2]
	}

	fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}
}

type Point3 = Vec3;

impl ToString for Vec3{
	fn to_string(&self) -> String{
		format!("{} {} {} ", self.e[0], self.e[1], self.e[2])
	}
}

impl Add for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 { 
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2]
                ]
         }
    }
}

impl Sub for &Vec3 {
	type Output = Vec3;
	fn sub(self, rhs: &Vec3) -> Self::Output {
	    Vec3 {
		    e: [
			self.e[0] - rhs.e[0],
			self.e[1] - rhs.e[1],
			self.e[2] - rhs.e[2]
		    ]
	    }
	}
}

impl Mul for &Vec3 {
	type Output = Vec3;
	fn mul(self, rhs: &Vec3) -> Self::Output {
	    Vec3 {
		    e: [
			self.e[0] * rhs.e[0],
			self.e[1] * rhs.e[1],
			self.e[2] * rhs.e[2]
		    ]
	    }
	}
}

impl Mul<f64> for &Vec3 {
	type Output = Vec3;
	fn mul(self, rhs: f64) -> Self::Output {
	    Vec3 {
		    e: [
			self.e[0] * rhs,
			self.e[1] * rhs,
			self.e[2] * rhs
		    ]
	    }
	}
}

impl Div<f64> for &Vec3 {
	type Output = Vec3;
	fn div(self, rhs: f64) -> Self::Output {
	    Vec3 {
		    e: [
			self.e[0] / rhs,
			self.e[1] / rhs,
			self.e[2] / rhs
		    ]
	    }
	}
}

impl Vec3 {
	pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
		u.e[0] * v.e[0] +
		u.e[1] * v.e[1] +
		u.e[2] * v.e[2]
	}
    
	pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
		Vec3 { e: [
			u.e[1] * v.e[2] - u.e[2] * v.e[1],
			u.e[2] * v.e[0] - u.e[0] * v.e[2],
			u.e[0] * v.e[1] - u.e[1] * v.e[0]
		] }
	}

	pub fn unit_vector(v: &Self) -> Self {
		v / v.length()
	}
}