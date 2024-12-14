use std::{io::BufRead, str::{FromStr, SplitWhitespace}};

use anyhow::{anyhow, bail};

use crate::util::objective::Objective;

use super::{Constraint, Problem, Relation};

impl Problem {
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
}