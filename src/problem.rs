use std::{fmt::Display, io::BufRead, iter::{once, repeat}, str::{FromStr, SplitWhitespace}};

use anyhow::{anyhow, bail};

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

    pub fn read_from_input(mut input: impl BufRead) -> anyhow::Result<Self> {
        let (dimension, constraints_size, objective) = Self::read_problem_info(&mut input)?;
        let function = Self::read_objective(&mut input, dimension)?;
        let constraints = Self::read_constraints(&mut input, dimension, constraints_size)?;
        Ok(Self::new(objective, function, constraints))
    }

    fn parse_next<T: FromStr>(iter: &mut SplitWhitespace) -> anyhow::Result<T> {
        let next = iter.next().ok_or(anyhow!("syntax error"))?;
        next.parse().map_err(|_| anyhow!("parse error"))
    }

    fn read_problem_info(input: &mut impl BufRead) -> anyhow::Result<(usize, usize, Objective)> {
        let mut buf = String::new();
        input.read_line(&mut buf)?;
        let mut line = buf.split_whitespace();
        let n = Self::parse_next(&mut line)?;
        let m = Self::parse_next(&mut line)?;
        let objective = match line.next().ok_or(anyhow!("syntax error"))? {
            "+" => Objective::Maximize,
            "-" => Objective::Minimize,
            _ => bail!("syntax error"),
        };
        Ok((n, m, objective))
    }

    fn read_objective(input: &mut impl BufRead, dimension: usize) -> anyhow::Result<Vec<isize>> {
        let mut buf = String::new();
        input.read_line(&mut buf)?;
        let line = buf.split_whitespace();
        let mut coefficients = Vec::with_capacity(dimension);
        for s in line.take(dimension) {
            coefficients.push(s.parse().map_err(|_| anyhow!("parse error"))?);
        }
        Ok(coefficients)
    }

    fn read_constraints(input: &mut impl BufRead, dimension: usize, constraints_size: usize) -> anyhow::Result<Vec<Constraint>> {
        let mut constraints = Vec::with_capacity(constraints_size);
        for _ in 0.. constraints_size {
            constraints.push(Self::read_constraint(input, dimension)?);
        }
        Ok(constraints)
    }

    fn read_constraint(input: &mut impl BufRead, dimension: usize) -> anyhow::Result<Constraint> {
        let mut buf = String::new();
        input.read_line(&mut buf)?;
        let mut line = buf.split_whitespace();
        let mut lhs = Vec::with_capacity(dimension);
        for _ in 0..dimension {
            lhs.push(Self::parse_next(&mut line)?);
        }
        let relation = match line.next().ok_or(anyhow!("syntax error"))? {
            "<=" => Relation::Le,
            "=" => Relation::Eq,
            ">=" => Relation::Ge,
            _ => bail!("syntax error"),
        };
        let rhs = Self::parse_next(&mut line)?;
        Ok(Constraint::new(lhs, relation, rhs))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Objective {
    Maximize,
    Minimize,
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