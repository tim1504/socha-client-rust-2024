use std::{fmt, ops::{Add, Sub, Mul, Div, DivAssign, MulAssign, AddAssign, SubAssign, Neg}};

use crate::util::{Element, Error, Result, Vec2};

use super::CubeDir;

/// A cube coordinate vector (or position).
/// (see https://www.redblobgames.com/grids/hexagons/#coordinates-cube).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubeVec {
    r: i32,
    q: i32,
    s: i32,
}

impl Default for CubeVec {
    fn default() -> Self {
        Self::ZERO
    }
}

impl CubeVec {
    /// The coordinate origin or zero direction vector, i.e. (0, 0, 0).
    pub const ZERO: Self = Self::new(0, 0, 0);

    /// Creates a new vector from the given cube components.
    #[inline]
    pub const fn new(r: i32, q: i32, s: i32) -> Self {
        Self { r, q, s }
    }

    /// Creates a new vector from the given r/q components.
    #[inline]
    pub const fn rq(r: i32, q: i32) -> Self {
        Self { r, q, s: -q - r }
    }

    /// The squared length of this vector.
    #[inline]
    pub fn squared_length(self) -> i32 { self.r * self.r + self.q * self.q + self.s * self.s }

    /// The length of this vector.
    #[inline]
    pub fn length(self) -> f32 { (self.squared_length() as f32).sqrt() }

    /// The first component of this vector.
    #[inline]
    pub fn r(self) -> i32 { self.r }

    /// The second component of this vector.
    #[inline]
    pub fn q(self) -> i32 { self.q }

    /// The third component of this vector.
    #[inline]
    pub fn s(self) -> i32 { self.s }

    /// Rotates by vector by the given amount of turns to the right.
    pub fn rotated_by(self, turns: i32) -> CubeVec {
        let components: [i32; 3] = self.into();
        let vec = CubeVec::new(
            components[turns.rem_euclid(3) as usize],
            components[(turns + 1).rem_euclid(3) as usize],
            components[(turns + 2).rem_euclid(3) as usize],
        );
        if turns % 2 == 0 { vec } else { -vec }
    }

    /// Fetches the 6 hex neighbors.
    pub fn hex_neighbors(self) -> [Self; 6] {
        CubeDir::ALL.map(|v| self + v)
    }
}

impl Add for CubeVec {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.r + rhs.r, self.q + rhs.q, self.s + rhs.s)
    }
}

impl Add<CubeDir> for CubeVec {
    type Output = Self;

    fn add(self, rhs: CubeDir) -> Self::Output {
        self + Self::from(rhs)
    }
}

impl AddAssign<CubeVec> for CubeVec {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.q += rhs.q;
        self.s += rhs.s;
    }
}

impl AddAssign<CubeDir> for CubeVec {
    fn add_assign(&mut self, rhs: CubeDir) {
        *self += Self::from(rhs);
    }
}

impl Sub for CubeVec {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.r - rhs.r, self.q - rhs.q, self.s - rhs.s)
    }
}

impl Sub<CubeDir> for CubeVec {
    type Output = Self;

    fn sub(self, rhs: CubeDir) -> Self::Output {
        self - Self::from(rhs)
    }
}

impl SubAssign<CubeVec> for CubeVec {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.q -= rhs.q;
        self.s -= rhs.s;
    }
}

impl SubAssign<CubeDir> for CubeVec {
    fn sub_assign(&mut self, rhs: CubeDir) {
        *self -= Self::from(rhs);
    }
}

impl Mul<i32> for CubeVec {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self::new(self.r * rhs, self.q * rhs, self.s * rhs)
    }
}

impl Mul<CubeVec> for i32 {
    type Output = CubeVec;

    fn mul(self, rhs: CubeVec) -> CubeVec {
        CubeVec::new(self * rhs.r, self * rhs.q, self * rhs.s)
    }
}

impl MulAssign<i32> for CubeVec {
    fn mul_assign(&mut self, rhs: i32) {
        self.r *= rhs;
        self.q *= rhs;
        self.s *= rhs;
    }
}

impl Div<i32> for CubeVec {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Self::new(self.r / rhs, self.q / rhs, self.s / rhs)
    }
}

impl DivAssign<i32> for CubeVec {
    fn div_assign(&mut self, rhs: i32) {
        self.r /= rhs;
        self.q /= rhs;
        self.s /= rhs;
    }
}

impl Neg for CubeVec {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.r, -self.q, -self.s)
    }
}

impl fmt::Display for CubeVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.q, self.s)
    }
}

impl From<CubeDir> for CubeVec {
    fn from(dir: CubeDir) -> Self {
        match dir {
            CubeDir::Right => Self::rq(1, 0),
            CubeDir::DownRight => Self::rq(0, 1),
            CubeDir::DownLeft => Self::rq(-1, 1),
            CubeDir::Left => Self::rq(-1, 0),
            CubeDir::UpLeft => Self::rq(0, -1),
            CubeDir::UpRight => Self::rq(1, -1),
        }
    }
}

impl<T> From<Vec2<T>> for CubeVec where T: Into<i32> {
    /// Converts local coordinates to cube coordinates.
    fn from(vec: Vec2<T>) -> Self {
        let x: i32 = vec.x.into();
        let y: i32 = vec.y.into();
        let r = y - 2;
        CubeVec::rq(x - 1 - r.min(0), r)
    }
}

impl From<CubeVec> for [i32; 3] {
    fn from(vec: CubeVec) -> Self {
        [vec.r, vec.q, vec.s]
    }
}

impl TryFrom<&Element> for CubeVec {
    type Error = Error;

    fn try_from(elem: &Element) -> Result<Self> {
        Ok(CubeVec::new(
            elem.attribute("r")?.parse()?,
            elem.attribute("q")?.parse()?,
            elem.attribute("s")?.parse()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{util::assert_xml_parse, game::CubeVec};

    #[test]
    fn test_xml_parses() {
        assert_xml_parse!(
            r#"<position r="23" q="0" s="-2" />"#,
            CubeVec::new(23, 0, -2)
        );
    }
}
