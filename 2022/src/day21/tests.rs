use super::{eval::Evaluator, parser::load_inputs, reorg::reorganize_assignments};
use crate::{common::InputType};

#[test]
fn test_part1() {
    let assignments = load_inputs(InputType::Example).unwrap();
    let answer = Evaluator::new(&assignments).evaluate("root");
    assert_eq!(answer, 152);
}

#[test]
fn test_part2() {
    let assignments = load_inputs(InputType::Example).unwrap();

    println!("==== Original Assignments");
    for assignment in &assignments {
        println!(" {}", assignment);
    }

    let mut e = Evaluator::new(&assignments);
    println!("  root={}", e.evaluate("root"));
    println!("  pppw={}", e.evaluate("pppw"));
    println!("  sjmn={}", e.evaluate("sjmn"));
    println!("  humn={}", e.evaluate("humn"));

    let restructured_assignments = reorganize_assignments(&assignments);

    println!("==== Restructured Assignments");
    for assignment in &restructured_assignments {
        println!(" {}", assignment);
    }

    let mut e = Evaluator::new(&restructured_assignments);
    let answer = e.evaluate("humn");
    assert_eq!(answer, 301);

    println!("  pppw={}", e.evaluate("pppw"));
    println!("  sjmn={}", e.evaluate("sjmn"));
    println!("  humn={}", e.evaluate("humn"));
}
