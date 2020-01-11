use std::ops::{Sub, Add};
use chrono::prelude::*;
use chrono::Duration;

extern crate chrono;

pub struct Airac {
    pub effective: NaiveDate,
    pub year: u16,
    pub ordinal: u8,
    pub value: String
}

impl Airac {
    pub fn new(date: &str) -> Airac {
        let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("Wrong date format");
        let epoch = NaiveDate::from_ymd(1901, 1, 10);
        let cycle = Duration::weeks(4);
        let got = (naive_date.sub(epoch) / cycle.num_seconds() as i32).num_seconds();
        let effective = epoch.add(cycle * got as i32);
        let year = effective.year() as u16;
        let ordinal = ((effective.ordinal()-1)/28 + 1) as u8;
        Airac { effective, year, ordinal, value: format!("{:02}{:02}", year%100, ordinal) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AiracTest {
        date: String,
        value: String
    }

    #[test]
    fn get_airac() {
        let mut airac_tests: Vec<AiracTest> = Vec::new();
        airac_tests.push(AiracTest { date: "1998-01-29".to_string(), value: "9802".to_string() });
        airac_tests.push(AiracTest { date: "2003-01-23".to_string(), value: "0301".to_string() });

        airac_tests.push(AiracTest { date: "2004-01-21".to_string(), value: "0313".to_string() });
        airac_tests.push(AiracTest { date: "2004-01-22".to_string(), value: "0401".to_string() });

        airac_tests.push(AiracTest { date: "2005-01-19".to_string(), value: "0413".to_string() });
        airac_tests.push(AiracTest { date: "2005-01-20".to_string(), value: "0501".to_string() });

        airac_tests.push(AiracTest { date: "2006-01-18".to_string(), value: "0513".to_string() });
        airac_tests.push(AiracTest { date: "2006-01-19".to_string(), value: "0601".to_string() });

        airac_tests.push(AiracTest { date: "2007-01-17".to_string(), value: "0613".to_string() });
        airac_tests.push(AiracTest { date: "2007-01-18".to_string(), value: "0701".to_string() });

        airac_tests.push(AiracTest { date: "2008-01-16".to_string(), value: "0713".to_string() });
        airac_tests.push(AiracTest { date: "2008-01-17".to_string(), value: "0801".to_string() });

        airac_tests.push(AiracTest { date: "2009-01-14".to_string(), value: "0813".to_string() });
        airac_tests.push(AiracTest { date: "2009-01-15".to_string(), value: "0901".to_string() });

        airac_tests.push(AiracTest { date: "2010-01-13".to_string(), value: "0913".to_string() });
        airac_tests.push(AiracTest { date: "2010-01-14".to_string(), value: "1001".to_string() });

        airac_tests.push(AiracTest { date: "2011-01-12".to_string(), value: "1013".to_string() });
        airac_tests.push(AiracTest { date: "2011-01-13".to_string(), value: "1101".to_string() });

        airac_tests.push(AiracTest { date: "2012-01-11".to_string(), value: "1113".to_string() });
        airac_tests.push(AiracTest { date: "2012-01-12".to_string(), value: "1201".to_string() });

        airac_tests.push(AiracTest { date: "2013-01-09".to_string(), value: "1213".to_string() });
        airac_tests.push(AiracTest { date: "2013-01-10".to_string(), value: "1301".to_string() });

        airac_tests.push(AiracTest { date: "2020-01-01".to_string(), value: "1913".to_string() });
        airac_tests.push(AiracTest { date: "2020-01-02".to_string(), value: "2001".to_string() });

        for airac_test in airac_tests {
            let airac = Airac::new(airac_test.date.as_str());
            assert_eq!(airac.value, airac_test.value);
        }
    }
}
