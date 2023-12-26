use std::ops::RangeInclusive;

use log::debug;
use ranges::Ranges;

use crate::day05::{Inputs, Mappings};
use crate::AocError;
use crate::DailyInput;

fn input_to_mappings(input: DailyInput) -> Result<(Inputs, Mappings), AocError> {
    let lines = input.get_input_lines()?;
    let inputs = Inputs::from(&lines);
    Ok((inputs.clone(), Mappings::from(&inputs)))
}

pub fn part2(input: DailyInput) -> Result<String, AocError> {
    let (inputs, mappings) = input_to_mappings(input)?;
    debug!("{:33} {:?}", "Seeds", inputs.seeds);

    let seed_ranges: Vec<RangeInclusive<i64>> = inputs
        .seeds
        .chunks(2)
        .map(|vals| RangeInclusive::new(vals[0], vals[0] + vals[1]))
        .collect::<Vec<_>>();
    debug!("{:33} {:?}", "Seed Ranges", seed_ranges);

    let outlet_edge_points = mappings.outlet_edge_points();
    debug!("{:33} {:?}", "Mappings Outlet Edge Points", outlet_edge_points);

    let all_ranges: Vec<RangeInclusive<i64>> =
        seed_ranges.iter().chain(mappings.outlet_ranges().iter()).cloned().collect();
    debug!("{:33} {:?}", "All Ranges", all_ranges);

    let all_ranges_to_check: Ranges<i64> = Ranges::from(all_ranges);
    debug!("{:33} {:?}", "All Ranges To Check", all_ranges_to_check);

    let seed_rangess = Ranges::from(seed_ranges);

    let mut min = i64::MAX;
    'outer: for rs in all_ranges_to_check.as_ref() {
        for i in rs.into_iter() {
            let back_mapping = mappings.map_reverse(i);
            let in_seeds = seed_rangess.contains(&back_mapping);
            debug!("   end: {i}, start: {back_mapping}, in_seeds={in_seeds}");
            if in_seeds {
                min = i;
                break 'outer;
            }
        }
    }

    Ok(min.to_string())
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use ranges::Ranges;

    use crate::day05::part2::input_to_mappings;
    use crate::DailyInput;
    use crate::InputType::Example;

    fn r(range: Range<i32>) -> Ranges<i32> {
        Ranges::from(range)
    }

    #[test]
    fn test_part2_example_map() {
        let (_inputs, mappings) = input_to_mappings(DailyInput {
            day: 5,
            number: None,
            input_type: Example,
        })
        .unwrap();
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
