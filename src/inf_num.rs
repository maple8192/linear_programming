use std::{fmt::Display, ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use fraction::{Fraction, One, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InfNum<T> {
    inf: T,
    num: T,
}

impl<T> InfNum<T> {
    pub fn new(inf: T, num: T) -> Self {
        Self { inf, num }
    }

    pub fn inf(&self) -> &T {
        &self.inf
    }

    pub fn num(&self) -> &T {
        &self.num
    }
}

impl<T: Default> From<T> for InfNum<T> {
    fn from(value: T) -> Self {
        Self { inf: T::default(), num: value }
    }
}

impl Display for InfNum<Fraction> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "$({}, {})$", self.inf, self.num)
    }
}

impl<T: Zero> Zero for InfNum<T> {
    fn zero() -> Self {
        Self { inf: T::zero(), num: T::zero() }
    }

    fn is_zero(&self) -> bool {
        self.inf.is_zero() && self.num.is_zero()
    }
}

impl<T: One + Zero> InfNum<T> {
    pub fn one() -> Self {
        Self { inf: T::zero(), num: T::one() }
    }

    pub fn one_inf() -> Self {
        Self { inf: T::one(), num: T::zero() }
    }
}

impl<S, T: Add<S, Output = T>> Add<InfNum<S>> for InfNum<T> {
    type Output = Self;
    fn add(self, rhs: InfNum<S>) -> Self::Output {
        Self { inf: self.inf + rhs.inf, num: self.num + rhs.num }
    }
}

impl<S, T: Sub<S, Output = T>> Sub<InfNum<S>> for InfNum<T> {
    type Output = Self;
    fn sub(self, rhs: InfNum<S>) -> Self::Output {
        Self { inf: self.inf - rhs.inf, num: self.num - rhs.num }
    }
}

impl<S, T: Copy, U: Mul<T, Output = S>> Mul<T> for InfNum<U> {
    type Output = InfNum<S>;
    fn mul(self, rhs: T) -> Self::Output {
        InfNum { inf: self.inf * rhs, num: self.num * rhs }
    }
}

impl<T: Neg<Output = T>> Neg for InfNum<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { inf: -self.inf, num: -self.num }
    }
}

impl<S, T: AddAssign<S>> AddAssign<InfNum<S>> for InfNum<T> {
    fn add_assign(&mut self, rhs: InfNum<S>) {
        self.inf += rhs.inf;
        self.num += rhs.num;
    }
}

impl<S, T: SubAssign<S>> SubAssign<InfNum<S>> for InfNum<T> {
    fn sub_assign(&mut self, rhs: InfNum<S>) {
        self.inf -= rhs.inf;
        self.num -= rhs.num;
    }
}

impl<S: Copy, T: MulAssign<S>> MulAssign<S> for InfNum<T> {
    fn mul_assign(&mut self, rhs: S) {
        self.inf *= rhs;
        self.num *= rhs;
    }
}
