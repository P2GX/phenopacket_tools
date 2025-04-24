use lazy_static::lazy_static;
use phenopackets::schema::v2::core::{time_element, Age, AgeRange, GestationalAge, TimeInterval};
use phenopackets::schema::{v2::core::OntologyClass, v2::core::TimeElement};
use regex::Regex;
use chrono::{DateTime, Utc};
use prost_types::Timestamp;

lazy_static! {
    pub static ref ISO8601_RE: Regex = Regex::new(r"^P(?:(\d+)Y)?(?:(\d+)M)?(?:(\d+)D)?$").unwrap();
}


use crate::error::{self, Error, Result};
use crate::constants::onset::{self, *};

/// Convert Ontology Class messages into TimeElements
pub trait ToTimeElement {
    fn to_time_element(&self) -> TimeElement;
}

impl ToTimeElement for OntologyClass {
    fn to_time_element(&self) -> TimeElement {
        TimeElement {
            element: Some(time_element::Element::OntologyClass(self.clone())),
        }
    }
}

impl Error {
    fn invalid_days(days: i32) -> Self {
        Error::TimeElementError { msg: format!("Invalid days ({days}) for GestationalAge") }
    }

    fn invalid_iso8601(age_string: &str) -> Self {
        Error::TimeElementError { msg: format!("Invalid iso8601 string ({age_string}) for Age") }
    }
}


pub fn from_ontology_class(clz: OntologyClass) -> Result<TimeElement> {
    Ok(TimeElement{element: Some(phenopackets::schema::v2::core::time_element::Element::OntologyClass(clz))})
}

macro_rules! define_time_element {
    ($($te_name:ident, $ontology_ref:ident);+ $(;)?) => {
        lazy_static! {
            $(
                pub static ref $te_name: TimeElement = TimeElement {
                    element: Some(time_element::Element::OntologyClass($ontology_ref.clone()))
                };
            )+
        }
    };
}

define_time_element!(
    ANTENATAL_ONSET_ELEMENT, ANTENATAL_ONSET;
    EMBRYONAL_ONSET_ELEMENT, EMBRYONAL_ONSET;
    FETAL_ONSET_ELEMENT, FETAL_ONSET;
    LATE_FIRST_TRIMESTER_ONSET_ELEMENT, LATE_FIRST_TRIMESTER_ONSET;
    SECOND_TRIMESTER_ONSET_ELEMENT, SECOND_TRIMESTER_ONSET;
    THIRD_TRIMESTER_ONSET_ELEMENT, THIRD_TRIMESTER_ONSET;
    CONGENITAL_ONSET_ELEMENT, CONGENITAL_ONSET;
    NEONATAL_ONSET_ELEMENT, NEONATAL_ONSET;
    INFANTILE_ONSET_ELEMENT, INFANTILE_ONSET;
    CHILDHOOD_ONSET_ELEMENT, CHILDHOOD_ONSET;
    JUVENILE_ONSET_ELEMENT, JUVENILE_ONSET;
    ADULT_ONSET_ELEMENT, ADULT_ONSET;
    YOUNG_ADULT_ONSET_ELEMENT, YOUNG_ADULT_ONSET;
    EARLY_YOUNG_ADULT_ONSET_ELEMENT, EARLY_YOUNG_ADULT_ONSET;
    INTERMEDIATE_YOUNG_ADULT_ONSET_ELEMENT, INTERMEDIATE_YOUNG_ADULT_ONSET;
    LATE_YOUNG_ADULT_ONSET_ELEMENT, LATE_YOUNG_ADULT_ONSET;
    MIDDLE_AGE_ONSET_ELEMENT, MIDDLE_AGE_ONSET;
    LATE_ONSET_ELEMENT, LATE_ONSET;
);


pub fn gestational_age(weeks: i32, days: i32) -> Result<TimeElement> {
    if days < 0 || days > 7 {
        return Err(Error::invalid_days(days));
    }
    let ga = GestationalAge {weeks: weeks, days: days};
    Ok(TimeElement{element: Some(phenopackets::schema::v2::core::time_element::Element::GestationalAge(ga))})
}

pub fn age(iso8601duration: impl Into<String>) -> Result<TimeElement> {
    let iso = iso8601duration.into();
    if  ISO8601_RE.is_match(&iso) {
        let age = Age{iso8601duration: iso};
        return Ok(TimeElement{element: Some(phenopackets::schema::v2::core::time_element::Element::Age(age))});
    } else {
        return Err(Error::invalid_iso8601(&iso));
    }
}

pub fn age_range(
    iso8601duration_start: impl Into<String>,
    iso8601duration_end: impl Into<String>
) -> Result<TimeElement> 
{
    let iso_start = iso8601duration_start.into();
    let iso_end = iso8601duration_end.into();
    if ! ISO8601_RE.is_match(&iso_start) {
        return Err(Error::invalid_iso8601(&iso_start));
    }
    if ! ISO8601_RE.is_match(&iso_end) {
        return Err(Error::invalid_iso8601(&iso_end));
    }
    let start_age = Age{iso8601duration: iso_start};
    let end_age = Age{iso8601duration: iso_end};
    let age_range = AgeRange {start: Some(start_age), end: Some(end_age)};
    Ok(TimeElement{element: Some(phenopackets::schema::v2::core::time_element::Element::AgeRange(age_range))})
}

/// Converts a chrono `DateTime<Utc>` to a Protobuf `Timestamp`
fn to_prost_timestamp(datetime: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: datetime.timestamp(),
        nanos: datetime.timestamp_subsec_nanos() as i32,
    }
}

pub fn timestamp_from_datetime(datetime: DateTime<Utc>) -> TimeElement {
    let ts = to_prost_timestamp(datetime);
    TimeElement {
        element: Some(phenopackets::schema::v2::core::time_element::Element::Timestamp(ts)),
    }
}

pub fn timestamp_from_str(s: &str) -> Result<TimeElement> {
    let datetime = s.parse::<DateTime<Utc>>().map_err(|e| e.to_string())?;
    Ok(timestamp_from_datetime(datetime))
}

pub fn interval_from_datetimes(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<TimeElement> {
    let start_ts = to_prost_timestamp(start);
    let end_ts = to_prost_timestamp(end);
    let interval = TimeInterval {
        start: Some(start_ts),
        end: Some(end_ts),
    };
    Ok(TimeElement {
        element: Some(phenopackets::schema::v2::core::time_element::Element::Interval(interval)),
    })
}

pub fn interval_from_strs(start: &str, end: &str) -> Result<TimeElement> {
    let start_dt = start.parse::<DateTime<Utc>>().map_err(|e| e.to_string())?;
    let end_dt = end.parse::<DateTime<Utc>>().map_err(|e| e.to_string())?;
    let interval = interval_from_datetimes(start_dt, end_dt)?;
    Ok(interval)
}


#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};


    #[rstest]
    #[case(32,4)]
    fn test_valid_gestational_age(
        #[case] weeks: i32, 
        #[case] days: i32 
    ) {
        let ga_elem = gestational_age(weeks, days);
        assert!(ga_elem.is_ok());
        let ga_elem = ga_elem.unwrap();
        assert!(ga_elem.element.is_some());
        match ga_elem.element {
            Some(time_element::Element::GestationalAge(ga)) => {
                assert_eq!(ga.weeks, weeks);
                assert_eq!(ga.days, days);
            }
            _ => panic!("Expected GestationalAge element"),
        }
    }




}