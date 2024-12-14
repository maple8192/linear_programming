use std::{fmt::Display, iter::{once, repeat}};

use crate::util::objective::Objective;

mod input;

#[derive(Debug)]
pub struct Problem {
    objective: Objective,
    function: Vec<isize>,
    constraints: Vec<Constraint>,
}

impl Problem {
    pub fn new(objective: Objective, function: Vec<isize>, constraints: Vec<Constraint>) -> Self {
        Self { objective, function, constraints }
    }

    pub fn objective(&self) -> Objective {
        self.objective
    }

    pub fn function(&self) -> &[isize] {
        &self.function
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "# Objective Function")?;
        writeln!(f)?;
        writeln!(f, "$$")?;
        write!(f, "x_0 = ")?;
        for (i, (&a, first)) in self.function.iter().zip(once(true).chain(repeat(false))).enumerate() {
            if a == 0 { continue; }
            write!(f, "{} {} x_{{{}}} ", if a.is_negative() { "-" } else if first { "" } else { "+" }, if a == 1 { "".to_string() } else { a.abs().to_string() }, i + 1)?;
        }
        writeln!(f, "\\longrightarrow \\text{{{}}}", match self.objective { Objective::Maximize => "最大化", Objective::Minimize => "最小化" })?;
        writeln!(f, "$$")?;
        writeln!(f)?;
        writeln!(f, "## Constraints")?;
        writeln!(f)?;
        writeln!(f, "$$")?;
        writeln!(f, "\\begin{{align}}")?;
        for constraint in &self.constraints {
            for (i, (&c, first)) in constraint.lhs.iter().zip(once(true).chain(repeat(false))).enumerate() {
                if c == 0 { write!(f, "&& ")?; continue; }
                write!(f, "&{}& {} x_{{{}}} ", if c.is_negative() { "-" } else if first { "" } else { "+" }, if c == 1 { "".to_string() } else { c.abs().to_string() }, i + 1)?;
            }
            writeln!(f, "&{}& {} \\\\", match constraint.relation { Relation::Le => "\\le", Relation::Eq => "=", Relation::Ge => "\\ge" }, constraint.rhs)?;
        }
        writeln!(f, "\\end{{align}}")?;
        writeln!(f, "$$")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Constraint {
    lhs: Vec<isize>,
    relation: Relation,
    rhs: isize,
}

impl Constraint {
    pub fn new(lhs: Vec<isize>, relation: Relation, rhs: isize) -> Self {
        Self { lhs, relation, rhs }
    }

    pub fn lhs(&self) -> &[isize] {
        &self.lhs
    }

    pub fn relation(&self) -> Relation {
        self.relation
    }

    pub fn rhs(&self) -> isize {
        self.rhs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    Le,
    Eq,
    Ge,
}