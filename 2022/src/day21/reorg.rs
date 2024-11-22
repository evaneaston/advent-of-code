use super::model::{Assignment, Expression, Operation};
use log::info;

/**
 * root: pppw + sjmn    ==5==>    pppw: sjmn + ZERO    and add     ZERO: 0
 * dbpl: 5
 * cczh: sllz + lgvd    ==3==>    lgvd: cczh - sllz
 * zczc: 2
 * ptdq: humn - dvpt    ==1==>    humn: ptdq + dvpt
 * dvpt: 3
 * lfqf: 4
 * humn: 5
 * ljgn: 2
 * sjmn: drzm * dbpl
 * sllz: 4
 * pppw: cczh / lfqf    ==4==>    cczh: pppw * lfqf
 * lgvd: ljgn * ptdq    ==2==>    ptdq: lgvd / ljgn
 * drzm: hmdt - zczc
 * hmdt: 32
 */
pub(super) fn reorganize_assignments(assignments: &Vec<Assignment>) -> Vec<Assignment> {
    let mut current_unknown = "humn".to_string();

    let mut assignments = assignments.clone();
    assignments.push(Assignment {
        symbol: "ZERO".to_string(),
        expression: Expression::Number(0),
    });
    let mut new_assignments = Vec::<Assignment>::with_capacity(assignments.len());

    loop {
        let (new_assignment, next_unknown) = reorg_to_solve_for(current_unknown, &mut assignments);
        new_assignments.push(new_assignment);
        match next_unknown {
            Some(unknown) => current_unknown = unknown,
            None => break,
        };
    }
    [assignments, new_assignments].concat()
}

pub(super) fn reorg_to_solve_for(
    unknown: String,
    assignments: &mut Vec<Assignment>,
) -> (Assignment, Option<String>) {
    info!(
        "Restructuring an assignment to move {} to the left hand side",
        unknown
    );
    for i in 0..assignments.len() {
        if assignments[i].symbol == "root" {
            match &assignments[i].expression {
                Expression::Number(_) => {
                    panic!("root should always be assignment to a binary expression")
                }
                Expression::BinaryExpression(left, _, right) => {
                    if left == &unknown || right == &unknown {
                        let other = if left == &unknown {
                            right.clone()
                        } else {
                            left.clone()
                        };
                        assignments.remove(i);

                        info!("At root assignment {} is equal to {}", unknown, other);

                        return (
                            Assignment {
                                symbol: unknown.clone(),
                                expression: Expression::BinaryExpression(
                                    other,
                                    Operation::Add,
                                    "ZERO".to_string(),
                                ),
                            },
                            None,
                        ); // yay done!
                    }
                }
            }
        } else {
            // only worry about checking right side of assignments with a binary expression for our unknown
            if let Expression::BinaryExpression(left, _, right) = &assignments[i].expression {
                if left == &unknown || right == &unknown {
                    let new_unknown = Some(assignments[i].symbol.clone());

                    let new_assignment = if left == &unknown {
                        solve_for_left_operand(&assignments[i])
                    } else {
                        solve_for_right_operand(&assignments[i])
                    };

                    info!("Replacing {} with {}", assignments[i], new_assignment);

                    assignments.remove(i);

                    return (new_assignment, new_unknown);
                }
                // otherwise the current_unknown is not on the right hand side of assignment to binary operation
            }
        }
    }

    panic!(
        "We were expected to find the assignment with {} on the right hand side and didn't",
        unknown
    );
}

pub(super) fn solve_for_left_operand(assignment: &Assignment) -> Assignment {
    match &assignment.expression {
        Expression::Number(_) => {
            panic!("Caller should have only passed a BinaryOperation variant")
        }
        Expression::BinaryExpression(left, operator, right) => Assignment {
            symbol: left.clone(),
            expression: match operator {
                Operation::Add => Expression::BinaryExpression(
                    assignment.symbol.clone(),
                    Operation::Subtract,
                    right.clone(),
                ),
                Operation::Subtract => Expression::BinaryExpression(
                    assignment.symbol.clone(),
                    Operation::Add,
                    right.clone(),
                ),
                Operation::Multiply => Expression::BinaryExpression(
                    assignment.symbol.clone(),
                    Operation::Divide,
                    right.clone(),
                ),
                Operation::Divide => Expression::BinaryExpression(
                    assignment.symbol.clone(),
                    Operation::Multiply,
                    right.clone(),
                ),
            },
        },
    }
}

pub(super) fn solve_for_right_operand(assignment: &Assignment) -> Assignment {
    match &assignment.expression {
        Expression::Number(_) => {
            panic!("Caller should have only passed a BinaryOperation variant")
        }
        Expression::BinaryExpression(left, operator, right) => Assignment {
            symbol: right.clone(),
            expression: match operator {
                Operation::Add => Expression::BinaryExpression(
                    assignment.symbol.clone(),
                    Operation::Subtract,
                    left.clone(),
                ),
                Operation::Subtract => Expression::BinaryExpression(
                    left.clone(),
                    Operation::Subtract,
                    assignment.symbol.clone(),
                ),
                Operation::Multiply => Expression::BinaryExpression(
                    assignment.symbol.clone(),
                    Operation::Divide,
                    left.clone(),
                ),
                Operation::Divide => Expression::BinaryExpression(
                    left.clone(),
                    Operation::Divide,
                    assignment.symbol.clone(),
                ),
            },
        },
    }
}
