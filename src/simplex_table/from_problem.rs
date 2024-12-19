use fraction::{Fraction, Zero};

use crate::{problem::{Problem, Relation}, util::objective::Objective};

use super::{equation::Equation, inf_num::InfNum, SimplexTable};

impl From<Problem> for SimplexTable {
    fn from(value: Problem) -> Self {
        let objective = value.objective();

        let mut constraints = Vec::new();
        for constraint in value.constraints().iter() {
            let mut lhs = Vec::new();
            let sign = if constraint.rhs().is_negative() { -1 } else { 1 };
            for &c in constraint.lhs() {
                lhs.push((c * sign).into());
            }
            let rhs = constraint.rhs().abs().into();
            let rel = match constraint.relation() {
                Relation::Le => if sign == 1 { Relation::Le } else { Relation::Ge }
                Relation::Eq => Relation::Eq,
                Relation::Ge => if sign == 1 { Relation::Ge } else { Relation::Le }
            };
            constraints.push((lhs, rhs, rel));
        }

        let mut function = {
            let mut lhs = Vec::new();
            for &a in value.function() {
                lhs.push(Fraction::from(-a).into());
            }
            (lhs, Fraction::default().into())
        };

        let mut base = Vec::new();
        for i in 0..constraints.len() {
            if constraints[i].2 == Relation::Le {
                for (j, (lhs, _, _)) in constraints.iter_mut().enumerate() {
                    lhs.push(if i == j { 1 } else { 0 }.into());
                }
                function.0.push(InfNum::zero());
                base.push(function.0.len());
                continue;
            }

            if constraints[i].2 == Relation::Ge {
                for (j, (lhs, _, _)) in constraints.iter_mut().enumerate() {
                    lhs.push(if i == j { -1 } else { 0 }.into());
                }
                function.0.push(Fraction::from(0).into());
            }

            for (j, (lhs, _, _)) in constraints.iter_mut().enumerate() {
                lhs.push(if i == j { 1 } else { 0 }.into());
            }
            function.0.push(InfNum::<Fraction>::one_inf() * if objective == Objective::Minimize { -1 } else { 1 });
            base.push(function.0.len());
            for (func, &cnst) in function.0.iter_mut().zip(&constraints[i].0) {
                *func += InfNum::<Fraction>::one_inf() * (cnst * if objective == Objective::Maximize { -1 } else { 1 });
            }
            function.1 += InfNum::<Fraction>::one_inf() * constraints[i].1 * if objective == Objective::Maximize { -1 } else { 1 };
        }

        let constraints = constraints.into_iter().zip(base).map(|((lhs, rhs, _), base)| (base, Equation::new(lhs, rhs))).collect();
        let function = Equation::new(function.0, function.1);
        Self { objective, constraints, function }
    }
}