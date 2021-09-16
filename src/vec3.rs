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

struct Vec3 {
    e : [f64; 3]
}

enum ColorIndex {
	R,
	G,
	B
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

impl Index<ColorIndex> for Vec3 {
	type Output = f64;
	fn index(&self, color: ColorIndex) -> &Self::Output {
		match color {
			ColorIndex::R => &self.e[0],
			ColorIndex::G => &self.e[1],
			ColorIndex::B => &self.e[2],
		}
	}
}

impl IndexMut<ColorIndex> for Vec3 {
	fn index_mut(&mut self, color: ColorIndex) -> &mut f64 {
		match color {
			ColorIndex::R => &mut self.e[0],
			ColorIndex::G => &mut self.e[1],
			ColorIndex::B => &mut self.e[2],
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
type Color = Vec3;

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
	fn dot(u: &Vec3, v: &Vec3) -> f64 {
		u.e[0] * v.e[0] +
		u.e[1] * v.e[1] +
		u.e[2] * v.e[2]
	}
    
	fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
		Vec3 { e: [
			u.e[1] * v.e[2] - u.e[2] * v.e[1],
			u.e[2] * v.e[0] - u.e[0] * v.e[2],
			u.e[0] * v.e[1] - u.e[1] * v.e[0]
		] }
	}

	fn unit_vector(v: &Self) -> Self {
		v / v.length()
	}
}
