use std::ops::{Sub, Add};
use chrono::prelude::*;
use chrono::Duration;

mod error;
use error::*;

pub struct Airac {
    pub effective: NaiveDate,
    pub year: u16,
    pub ordinal: u8,
    pub value: u16
}

impl Airac {

    fn epoch() -> NaiveDate { NaiveDate::from_ymd(1901, 1, 10) }

    pub fn from_date_str(date: &str, week_cycle: i64) -> Airac {
        let duration = Duration::weeks(week_cycle);
        let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("Wrong date format");
        let got = (naive_date.sub(Airac::epoch()) / duration.num_seconds() as i32).num_seconds();
        let effective = Airac::epoch().add(duration * got as i32);
        Airac::from_date(effective, duration)
    }

    pub fn from_airac_str(yyoo: u16, week_cycle: i64) -> Result<Airac, AiracError> {
        let duration = Duration::weeks(week_cycle);
        let (year, ordinal) = Airac::get_identifiers(yyoo)?;
        let date = NaiveDate::from_ymd((year - 1) as i32, 12, 31);
        let last_airac_of_previous_year = date.sub(Airac::epoch()) / duration.num_seconds() as i32;
        let effective = Airac::epoch().add(duration * (last_airac_of_previous_year.num_seconds() as i32 + ordinal as i32));
        Ok(Airac::from_date(effective, duration))
    }

    fn from_date(effective: NaiveDate, duration: Duration) -> Airac {
        let year = effective.year() as u16;
        let ordinal = ((effective.ordinal()-1)/duration.num_days() as u32 + 1) as u8;
        let value = format!("{:02}{:02}", year%100, ordinal).parse::<u16>().expect("parse error");
        Airac { effective, year, ordinal, value }
    }

    fn get_identifiers(yyoo: u16) -> Result<(u16, u8), AiracError> {
        if yyoo > 9999 {
            let error = AiracError::new(format!("illegal AIRAC id {}", yyoo).as_str());
            return Err(error)
        }
        let mut year = (yyoo/100) + 1900;
        if year <= 1963 {
            year += 100
        }
        let ordinal: u8 = (yyoo%100) as u8;
        Ok((year, ordinal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AiracTest {
        date: String,
        value: u16
    }

    #[test]
    fn get_identifiers() {
        let (year, ordinal) = Airac::get_identifiers(2001).unwrap();
        assert_eq!(year, 2020);
        assert_eq!(ordinal, 1);
    }

    #[test]
    fn from_string_month() {
        let airac = Airac::from_airac_str(1913, 4).unwrap();
        assert_eq!(airac.effective, NaiveDate::from_ymd(2019,12,05));
    }

    #[test]
    fn from_string_week() {
        let airac = Airac::from_airac_str(2001, 1).unwrap();
        assert_eq!(airac.effective, NaiveDate::from_ymd(2020,01,02));
        let airac = Airac::from_airac_str(2005, 1).unwrap();
        assert_eq!(airac.effective, NaiveDate::from_ymd(2020,01,30));
    }

    #[test]
    fn from_date_week() {
        let airac = Airac::from_date_str("2020-01-02", 1);
        assert_eq!(airac.ordinal, 1);

        let airac = Airac::from_date_str("2020-01-30", 1);
        assert_eq!(airac.ordinal, 5);
    }

    #[test]
    fn from_date_month() {
        let mut airac_tests: Vec<AiracTest> = Vec::new();
        airac_tests.push(AiracTest { date: "1998-01-29".to_string(), value: 9802 });
        airac_tests.push(AiracTest { date: "2003-01-23".to_string(), value: 0301 });

        airac_tests.push(AiracTest { date: "2004-01-21".to_string(), value: 0313 });
        airac_tests.push(AiracTest { date: "2004-01-22".to_string(), value: 0401 });

        airac_tests.push(AiracTest { date: "2005-01-19".to_string(), value: 0413 });
        airac_tests.push(AiracTest { date: "2005-01-20".to_string(), value: 0501 });

        airac_tests.push(AiracTest { date: "2006-01-18".to_string(), value: 0513 });
        airac_tests.push(AiracTest { date: "2006-01-19".to_string(), value: 0601 });

        airac_tests.push(AiracTest { date: "2007-01-17".to_string(), value: 0613 });
        airac_tests.push(AiracTest { date: "2007-01-18".to_string(), value: 0701 });

        airac_tests.push(AiracTest { date: "2008-01-16".to_string(), value: 0713 });
        airac_tests.push(AiracTest { date: "2008-01-17".to_string(), value: 0801 });

        airac_tests.push(AiracTest { date: "2009-01-14".to_string(), value: 0813 });
        airac_tests.push(AiracTest { date: "2009-01-15".to_string(), value: 0901 });

        airac_tests.push(AiracTest { date: "2010-01-13".to_string(), value: 0913 });
        airac_tests.push(AiracTest { date: "2010-01-14".to_string(), value: 1001 });

        airac_tests.push(AiracTest { date: "2011-01-12".to_string(), value: 1013 });
        airac_tests.push(AiracTest { date: "2011-01-13".to_string(), value: 1101 });

        airac_tests.push(AiracTest { date: "2012-01-11".to_string(), value: 1113 });
        airac_tests.push(AiracTest { date: "2012-01-12".to_string(), value: 1201 });

        airac_tests.push(AiracTest { date: "2013-01-09".to_string(), value: 1213 });
        airac_tests.push(AiracTest { date: "2013-01-10".to_string(), value: 1301 });

        airac_tests.push(AiracTest { date: "2020-01-01".to_string(), value: 1913 });
        airac_tests.push(AiracTest { date: "2020-01-02".to_string(), value: 2001 });

        for airac_test in airac_tests {
            let airac = Airac::from_date_str(airac_test.date.as_str(), 4);
            assert_eq!(airac.value, airac_test.value);
        }
    }
}
