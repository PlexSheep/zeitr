use std::process::ExitCode;

use clap::Parser;
use exmex::{
    self, BinOp, ExResult, Express, FlatEx, MakeOperators, MatchLiteral, Operator,
    literal_matcher_from_pattern, ops_factory,
};

use crate::util::naive_time_to_delta;

#[derive(Parser, Debug)]
pub struct CalcInput {
    tokens: Vec<String>, // TODO: add limit, at least one
}

ops_factory!(
    TimeFactory,
    chrono::NaiveTime,
    Operator::make_bin(
        "+",
        BinOp {
            apply: |a, b| a + naive_time_to_delta(b),
            prio: 1,
            is_commutative: true,
        }
    ),
    Operator::make_bin(
        "-",
        BinOp {
            apply: |a, b| a - naive_time_to_delta(b),
            prio: 1,
            is_commutative: true,
        }
    )
);
literal_matcher_from_pattern!(TimeMatcher, r"^\d(\d)?:\d(\d)?(:\d(\d)?)?");

pub type FlatExTime = FlatEx<chrono::NaiveTime, TimeFactory, TimeMatcher>;

pub fn calc(inputs: &CalcInput) -> ExResult<chrono::NaiveTime> {
    #[cfg(debug_assertions)]
    println!("Tokens: {:?}", inputs.tokens);
    let time = FlatExTime::parse(&inputs.tokens.join(" "))?;
    time.eval(&[])
}

pub fn calc_cli(inputs: &CalcInput) -> ExitCode {
    match calc(inputs) {
        Ok(s) => {
            println!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod test {
    use super::FlatExTime;
    use exmex::Express;

    #[test]
    fn test_parse_time_easy() {
        let s = "13:37";
        let ex = FlatExTime::parse(s).unwrap();
        assert_eq!(
            ex.eval(&[]).unwrap(),
            chrono::NaiveTime::from_hms_opt(13, 37, 0).unwrap()
        )
    }

    #[test]
    fn test_parse_time_secs() {
        let s = "13:37:19";
        let ex = FlatExTime::parse(s).unwrap();
        assert_eq!(
            ex.eval(&[]).unwrap(),
            chrono::NaiveTime::from_hms_opt(13, 37, 19).unwrap()
        )
    }

    #[test]
    fn test_parse_time_secs_leave_out() {
        let s = "13:37:9";
        let ex = FlatExTime::parse(s).unwrap();
        assert_eq!(
            ex.eval(&[]).unwrap(),
            chrono::NaiveTime::from_hms_opt(13, 37, 9).unwrap()
        )
    }

    #[test]
    fn test_parse_time_mini_leave_out_1() {
        let s = "3:7";
        let ex = FlatExTime::parse(s).unwrap();
        assert_eq!(
            ex.eval(&[]).unwrap(),
            chrono::NaiveTime::from_hms_opt(3, 7, 0).unwrap()
        )
    }

    #[test]
    fn test_parse_time_mini_leave_out_2() {
        let s = "3:07";
        let ex = FlatExTime::parse(s).unwrap();
        assert_eq!(
            ex.eval(&[]).unwrap(),
            chrono::NaiveTime::from_hms_opt(3, 7, 0).unwrap()
        )
    }

    #[test]
    #[should_panic]
    fn test_parse_time_extra_colon() {
        let s = "3:07:";
        let ex = FlatExTime::parse(s).unwrap();
        assert_eq!(
            ex.eval(&[]).unwrap(),
            chrono::NaiveTime::from_hms_opt(3, 7, 0).unwrap()
        )
    }
}
