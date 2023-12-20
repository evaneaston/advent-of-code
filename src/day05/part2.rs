use std::ops::RangeInclusive;

use log::debug;

use crate::AocError;
use crate::DailyInput;
use crate::day05::{Inputs, Mappings};

//
// impl NumericMapping {
//     pub fn map_ranges(&self, ranges: Ranges<i64>) -> Ranges<i64> {
//         let unshifted = ranges.clone().difference(self.ranges.clone());
//         println!("          unshifted={unshifted}");
//         let shifted = Ranges::from(
//             self.ranges.clone().intersect(ranges).as_ref().iter().map(|gr| {
//                 let start = match gr.start_bound() {
//                     Bound::Included(v) => *v,
//                     Bound::Excluded(v) => panic!("Don't know what to do with excluded start {} (from {})", v, gr),
//                     Bound::Unbounded => panic!("Don't know what to when start is unbounded (from {})", gr),
//                 };
//                 let end = match gr.end_bound() {
//                     Bound::Excluded(v) => *v,
//                     Bound::Included(v) => if *v == start { start + 1 } else { panic!("Don't know what to do with include end of {} when it doesn't match the start of {} (from {})", v, start, gr) },
//                     Bound::Unbounded => panic!("Don't know what to when end is unbounded (from {})", gr),
//                 };
//                 let r = (start + self.offset)..(end + self.offset);
//                 println!("            {gr} --({})--> {}", self.offset, Ranges::from(r.clone()));
//                 r
//             }).collect::<Vec<_>>()
//         );
//         println!("          shifted={shifted}");
//         let union = unshifted.union(shifted);
//         println!("          union={union}");
//         union
//     }
// }
//
//     fn map_ranges(&self, a: &str, b: &str, input_ranges: Ranges<i64>) -> Ranges<i64> {
//         println!("\n{a} to {b}:");
//         let mapper = self.get_mapper_for(a, b);
//         let mapping_string = mapper.numeric_mappings.iter().map(|nm| format!("{}:{}", nm.ranges, nm.offset)).collect::<Vec<_>>().join(",");
//         println!("  Mapping {input_ranges} using {mapping_string}");
//         let mapped_ranges = mapper.numeric_mappings.iter().map(|nm| {
//             let mapped_ranges = nm.map_ranges(input_ranges.clone());
//             mapped_ranges
//         }).fold(Ranges::new(), |rs1, rs2| rs1.union(rs2));
//         println!("  {a} to {b} {input_ranges} --({mapping_string})--> {mapped_ranges}");
//         mapped_ranges
//     }
//
//     fn map_single(&self, a: &str, b: &str, v: i64) -> i64 {
//         let mapper = self.get_mapper_for(a, b);
//         let mapping_string = mapper.numeric_mappings.iter().map(|nm| format!("{}:{}", nm.ranges, nm.offset)).collect::<Vec<_>>().join(",");
//         println!("  Mapping {v} using {mapping_string}");
//         let mapped_ranges = mapper.numeric_mappings.iter().map(|nm| {
//             nm.map2(v)
//         }).fold(Ranges::new(), |rs1, rs2| rs1.union(rs2));
//         println!("  {a} to {b} {input_ranges} --({mapping_string})--> {mapped_ranges}");
//         mapped_ranges
//     }
// }
//  impl Inputs {
//
//  }
fn input_to_mappings(input: DailyInput) -> Result<(Inputs, Mappings), AocError> {
    let lines = input.get_input_lines()?;
    let inputs = Inputs::from(&lines);
    Ok((inputs.clone(), Mappings::from(&inputs)))
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let (inputs, mappings) = input_to_mappings(input)?;
    debug!("seeds: {:?}", inputs.seeds);

    let seed_ranges: Vec<RangeInclusive<i64>> =
        inputs.seeds.chunks(2).map(|vals| RangeInclusive::new(vals[0], vals[0] + vals[1])).collect::<Vec<_>>();

    debug!("seed ranges: {:?}", seed_ranges);


    // debug!("Edge points {:?}", inputs.edge_points());
    //
    // let points: Vec<i64> = inputs.edge_points().iter()
    //     .map(|&v| vec![v - 1, v, v + 1]).flatten()
    //     .filter(|&v| seed_ranges.iter().any(|r| r.contains(&v)))
    //     .collect();
    // debug!("Filtered Edge points {:?}", points);
    // let min = points.iter().map(|&p| model.map(p)).min().unwrap();


    let min = seed_ranges.iter().map(|range| range.clone().map(|v| {
        let mapped_to = mappings.map(v);
        debug!("{v} mapped to {mapped_to}");
        mapped_to
    })).flatten().min().unwrap();

    Ok(min.to_string())
}


#[cfg(test)]
mod tests {
    use std::ops::Range;

    use ranges::Ranges;

    use crate::DailyInput;
    use crate::day05::part2::input_to_mappings;
    use crate::InputType::Example;

    fn r(range: Range<i32>) -> Ranges<i32> { Ranges::from(range) }

    #[test]
    fn test_part2_example_map() {
        let (_inputs, mappings)  =input_to_mappings(DailyInput{day:5, number: None, input_type: Example}).unwrap();
        let mapped_to = mappings.map(82);
        println!("82 mapped to {mapped_to}");
        assert_eq!(mapped_to, 46);
    }

    #[test]
    fn test_range_subtract() {
        assert_eq!(r(1..10).difference(r(-5..0)), r(1..10));
        assert_eq!(r(1..10).difference(r(-5..1)), r(1..10));
        assert_eq!(r(1..10).difference(r(-5..2)), r(2..10));
        assert_eq!(r(1..10).difference(r(1..2)), r(2..10));
        assert_eq!(r(1..10).difference(r(2..3)), Ranges::from(vec![1..2, 3..10]));
        assert_eq!(r(1..10).difference(r(2..10)), Ranges::from(vec![1..2]));
        assert_eq!(r(1..10).difference(r(2..11)), Ranges::from(vec![1..2]));
        assert_eq!(r(1..10).difference(r(9..12)), Ranges::from(vec![1..9]));
        assert_eq!(r(1..10).difference(r(10..12)), Ranges::from(vec![1..10]));
        assert_eq!(r(1..10).difference(r(11..12)), Ranges::from(vec![1..10]));
        assert_eq!(r(1..10).difference(r(1..10)), Ranges::new());
        assert_eq!(r(1..10).difference(r(-5..12)), Ranges::new());
    }

    #[test]
    fn test_range_intersect() {
        assert_eq!(r(1..10).intersect(r(-5..0)), Ranges::new());
        assert_eq!(r(1..10).intersect(r(-5..1)), Ranges::from(vec![1..1]));
        assert_eq!(r(1..10).intersect(r(-5..2)), Ranges::from(vec![1..2]));
        assert_eq!(r(1..10).intersect(r(1..2)), Ranges::from(vec![1..2]));
        assert_eq!(r(1..10).intersect(r(2..3)), Ranges::from(vec![2..3]));
        assert_eq!(r(1..10).intersect(r(2..10)), Ranges::from(vec![2..10]));
        assert_eq!(r(1..10).intersect(r(2..11)), Ranges::from(vec![2..10]));
        assert_eq!(r(1..10).intersect(r(9..12)), Ranges::from(vec![9..10]));
        assert_eq!(r(1..10).intersect(r(10..12)), Ranges::new());
        assert_eq!(r(1..10).intersect(r(11..12)), Ranges::new());
    }
}
