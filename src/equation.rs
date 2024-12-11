use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct Equation<T> {
    lhs: Vec<T>,
    rhs: T
}

impl<T> Equation<T> {
    pub fn new(lhs: Vec<T>, rhs: T) -> Self {
        Self { lhs, rhs }
    }

    pub fn lhs(&self) -> &[T] {
        &self.lhs
    }

    pub fn rhs(&self) -> &T {
        &self.rhs
    }
}

impl<S, T: Add<S, Output = T> + Copy> Add<Equation<S>> for Equation<T> {
    type Output = Self;
    fn add(self, rhs: Equation<S>) -> Self::Output {
        Self { lhs: self.lhs.into_iter().zip(rhs.lhs).map(|(x, y)| x + y).collect(), rhs: self.rhs + rhs.rhs }
    }
}

impl<S, T: Sub<S, Output = T> + Copy> Sub<Equation<S>> for Equation<T> {
    type Output = Self;
    fn sub(self, rhs: Equation<S>) -> Self::Output {
        Self { lhs: self.lhs.into_iter().zip(rhs.lhs).map(|(x, y)| x - y).collect(), rhs: self.rhs - rhs.rhs }
    }
}

impl<S: Copy, T: Mul<S, Output = T> + Copy> Mul<S> for Equation<T> {
    type Output = Self;
    fn mul(self, rhs: S) -> Self::Output {
        Self { lhs: self.lhs.into_iter().map(|x| x * rhs).collect(), rhs: self.rhs * rhs }
    }
}

impl<T: Neg<Output = T>> Neg for Equation<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { lhs: self.lhs.into_iter().map(|x| -x).collect(), rhs: -self.rhs }
    }
}

impl<S, T: AddAssign<S>> AddAssign<Equation<S>> for Equation<T> {
    fn add_assign(&mut self, rhs: Equation<S>) {
        self.lhs.iter_mut().zip(rhs.lhs).for_each(|(x, y)| *x += y);
        self.rhs += rhs.rhs;
    }
}

impl<S, T: SubAssign<S>> SubAssign<Equation<S>> for Equation<T> {
    fn sub_assign(&mut self, rhs: Equation<S>) {
        self.lhs.iter_mut().zip(rhs.lhs).for_each(|(x, y)| *x -= y);
        self.rhs -= rhs.rhs;
    }
}

impl<S: Copy, T: MulAssign<S>> MulAssign<S> for Equation<T> {
    fn mul_assign(&mut self, rhs: S) {
        self.lhs.iter_mut().for_each(|x| *x *= rhs);
        self.rhs *= rhs;
    }
}