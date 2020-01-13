use std::ops::{Sub, Add};
use chrono::prelude::*;
use chrono::Duration;

mod error;
use error::*;

pub struct Airac {
    pub effective: NaiveDate,
    pub year: u16,
    pub ordinal: u8,
    pub value: String
}

impl Airac {

    fn cycle() -> Duration { Duration::weeks(4) }
    fn epoch() -> NaiveDate { NaiveDate::from_ymd(1901, 1, 10) }

    pub fn from_date_str(date: &str) -> Airac {
        let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("Wrong date format");
        let got = (naive_date.sub(Airac::epoch()) / Airac::cycle().num_seconds() as i32).num_seconds();
        let effective = Airac::epoch().add(Airac::cycle() * got as i32);
        Airac::from_date(effective)
    }

    pub fn from_airac_str(yyoo: &str) -> Result<Airac, AiracError> {
        let (year, ordinal) = Airac::get_identifiers(yyoo)?;
        let date = NaiveDate::from_ymd((year - 1) as i32, 12, 31);
        let last_airac_of_previous_year = date.sub(Airac::epoch()) / Airac::cycle().num_seconds() as i32;
        let effective = Airac::epoch().add(Airac::cycle() * (last_airac_of_previous_year.num_seconds() as i32 + ordinal as i32));
        Ok(Airac::from_date(effective))
    }

    fn from_date(effective: NaiveDate) -> Airac {
        let year = effective.year() as u16;
        let ordinal = ((effective.ordinal()-1)/28 + 1) as u8;
        Airac { effective, year, ordinal, value: format!("{:02}{:02}", year%100, ordinal) }
    }

    fn get_identifiers(yyoo: &str) -> Result<(u16, u8), AiracError> {
        if yyoo.len() != 4 {
            let error = AiracError::new(format!("illegal AIRAC id {}", yyoo).as_str());
            return Err(error)
        }
        let digit: u16 = yyoo.parse()?;
        let mut year = (digit/100) + 1900;
        if year <= 1963 {
            year += 100
        }
        let ordinal: u8 = (digit%100) as u8;
        Ok((year, ordinal))
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
    fn from_string() {
        let airac = Airac::from_airac_str("1913").unwrap();
        assert_eq!(airac.year, 2019);
        assert_eq!(airac.ordinal, 13);
    }

    #[test]
    fn get_identifiers() {
        let (year, ordinal) = Airac::get_identifiers("2001").unwrap();
        assert_eq!(year, 2020);
        assert_eq!(ordinal, 1);
    }

    #[test]
    fn from_date() {
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
            let airac = Airac::from_date_str(airac_test.date.as_str());
            assert_eq!(airac.value, airac_test.value);
        }
    }
}
