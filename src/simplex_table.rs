use std::fmt::Display;

use equation::Equation;
use fraction::{Fraction, Zero};
use inf_num::InfNum;

use crate::util::objective::Objective;

mod equation;
mod inf_num;
mod from_problem;

#[derive(Debug)]
pub struct SimplexTable {
    objective: Objective,
    constraints: Vec<(usize, Equation<Fraction>)>,
    function: Equation<InfNum<Fraction>>,
}

impl SimplexTable {
    pub fn step(&mut self) -> bool {
        if self.objective == Objective::Maximize && self.function.lhs().iter().all(|&x| x >= InfNum::zero()) { return true; }
        if self.objective == Objective::Minimize && self.function.lhs().iter().all(|&x| x <= InfNum::zero()) { return true; }

        let coef_min_or_max = match self.objective {
            Objective::Maximize => self.function.lhs().iter().min(),
            Objective::Minimize => self.function.lhs().iter().max(),
        }.unwrap();
        let pivot_i = self.function.lhs().iter().enumerate().find(|&(_, x)| x == coef_min_or_max).unwrap().0;
        let theta = self.constraints.iter().map(|(_, c)| c.rhs() / c.lhs()[pivot_i]);
        let theta_min = theta.clone().filter(|&x| x >= Fraction::zero()).min().unwrap();
        let pivot_j = theta.enumerate().find(|&(_, f)| f == theta_min).unwrap().0;
        let pivot = self.constraints[pivot_j].1.lhs()[pivot_i];
        self.constraints[pivot_j].0 = pivot_i + 1;

        self.constraints[pivot_j].1 *= Fraction::from(1) / pivot;

        let pivot_constraint = self.constraints[pivot_j].1.clone();
        for (j, (_, constraint)) in self.constraints.iter_mut().enumerate() {
            if j == pivot_j { continue; }
            *constraint -= pivot_constraint.clone() * constraint.lhs()[pivot_i];
        }
        let func_scale = self.function.lhs()[pivot_i];
        self.function -= Equation::new(pivot_constraint.lhs().iter().map(|&x| func_scale * x).collect(), func_scale * *pivot_constraint.rhs());

        if self.objective == Objective::Maximize && self.function.lhs().iter().all(|&x| x >= InfNum::zero()) { return true; }
        if self.objective == Objective::Minimize && self.function.lhs().iter().all(|&x| x <= InfNum::zero()) { return true; }
        false
    }
}

impl Display for SimplexTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "||{}|RHS|", (0..self.function.lhs().len()).map(|i| format!("$x_{}$", i + 1)).collect::<Vec<_>>().join("|"))?;
        writeln!(f, "|{}|", (0..self.function.lhs().len() + 2).map(|_| ":-:").collect::<Vec<_>>().join("|"))?;
        for constraint in &self.constraints {
            writeln!(f, "|$x_{{{}}}$|{}|${}$|", constraint.0, constraint.1.lhs().iter().map(|x| format!("${x}$")).collect::<Vec<_>>().join("|"), constraint.1.rhs())?;
        }
        write!(f, "|$x_0$|{}|{}|", self.function.lhs().iter().map(|x| x.to_string()).collect::<Vec<_>>().join("|"), self.function.rhs())?;
        Ok(())
    }
}