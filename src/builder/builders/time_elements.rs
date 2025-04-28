//use std::sync::mpsc::RecvTimeoutError;

use crate::error::{self, Error, Result};
use crate::constants::onset::{self, *};
use lazy_static::lazy_static;
use phenopackets::ga4gh::vrs::v1::repeated_sequence_expression;
use phenopackets::schema::v2::core::{time_element, Age, AgeRange, GestationalAge, TimeInterval};
use phenopackets::schema::{v2::core::OntologyClass, v2::core::TimeElement};
use regex::Regex;
use chrono::{DateTime, Utc};
use prost_types::Timestamp;

lazy_static! {
    pub static ref ISO8601_RE: Regex = Regex::new(r"^P(?:(\d+)Y)?(?:(\d+)M)?(?:(\d+)D)?$").unwrap();
    pub static ref GESTATIONAL_AGE_RE: Regex = Regex::new(r"^(\d+)w(\d+)d$").unwrap();
    
}



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

    fn invalid_weeks(weeks: i32) -> Self {
        Error::TimeElementError { msg: format!("Invalid weeks ({weeks}) for GestationalAge") }
    }

    fn invalid_iso8601(age_string: &str) -> Self {
        Error::TimeElementError { msg: format!("Invalid iso8601 string ({age_string}) for Age") }
    }

    fn unrecognized_onset(age_string: &str) -> Self {
        Error::TimeElementError { msg: format!("Malformed onset string ({age_string})") }
    }

    fn invalid_gestational_age(age_string: &str) -> Self {
        Error::TimeElementError { msg: format!("Malformed GestationalAlge string ({age_string})") }
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
    if weeks < 0 {
        return Err(Error::invalid_weeks(weeks));
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


fn parse_gestational_age(input: &str) -> Option<(i32, i32)> {
    // This would usually be a `lazy_static!` or `once_cell` static
    let re = Regex::new(r"^(\d+)w(\d+)d$").unwrap();
    if let Some(captures) = re.captures(input) {
        let weeks = captures.get(1)?.as_str().parse().ok()?;
        let days = captures.get(2)?.as_str().parse().ok()?;
        Some((weeks, days))
    } else {
        None
    }
}


/// parse strings representing gestational_age, age, ontology_class, or timestamp
pub fn time_element_from_str(value: &str) 
    -> Result<TimeElement> 
{
    if value.ends_with("Z") {
        return timestamp_from_str(value);
    }
    if value.starts_with("P") {
        return age(value);
    }
    if let Some(captures) = GESTATIONAL_AGE_RE.captures(value) {
        let weeks: i32 = captures
            .get(1)
            .ok_or_else(|| Error::invalid_gestational_age(value))?  
            .as_str()
            .parse()
            .map_err(|_| Error::invalid_gestational_age(value))?; 
        let days = captures
            .get(2)
            .ok_or_else(|| Error::invalid_gestational_age(value))?  
            .as_str()
            .parse()
            .map_err(|_| Error::invalid_gestational_age(value))?; 
        return gestational_age(weeks, days);
    } 
    if let Some(onset_clz) = onset::get_onset_by_label(value) {
        return Ok(TimeElement{element: Some(phenopackets::schema::v2::core::time_element::Element::OntologyClass(onset_clz.clone()))});
    }
   Err(Error::unrecognized_onset(value))
}




pub fn timestamp_from_iso8601(iso8601_str: &str) -> Result<Timestamp> {
    let datetime: DateTime<Utc> = iso8601_str.parse()
        .map_err(|e| Error::TimeElementError { msg:format!( "Could not parse {iso8601_str}") })?;
    Ok(Timestamp {
        seconds: datetime.timestamp(),
        nanos: datetime.timestamp_subsec_nanos() as i32,
    })
}


#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};


    #[rstest]
    #[case(32,4)]
    #[case(7,2)]
    #[case(42,6)]
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

    #[rstest]
    #[case(32,8, "Invalid days (8) for GestationalAge")]
    #[case(7,12, "Invalid days (12) for GestationalAge")]
    #[case(-12,6, "Invalid weeks (-12) for GestationalAge")]
    fn test_invalid_gestational_age(
        #[case] weeks: i32, 
        #[case] days: i32,
        #[case] error_msg: &str
    ) {
        let ga_elem = gestational_age(weeks, days);
        assert!(ga_elem.is_err());
        assert!(matches!(&ga_elem, Err(Error::TimeElementError { .. })));
        assert_eq!(error_msg, ga_elem.unwrap_err().to_string());
    }

    #[rstest]
    #[case("33w2d", 33, 2)]
    #[case("12w0d", 12, 0)]
    #[case("8w5d", 8, 5)]
    fn test_valid_gestational_age_from_str(
        #[case] gestational_age: &str,
        #[case] weeks: i32, 
        #[case] days: i32)
    {
        let result = time_element_from_str(gestational_age);
        assert!(result.is_ok());    
        let gestational_age_time_elem = result.unwrap();
        match gestational_age_time_elem.element {
            Some(time_element::Element::GestationalAge(ga)) => {
                assert_eq!(ga.weeks, weeks);
                assert_eq!(ga.days, days);
            }
            _ => panic!("Expected GestationalAge element"),
        }
    }



    
    
    #[rstest]
    #[case("HP:0030674", "Antenatal onset")]
    #[case("HP:0011460", "Embryonal onset")]
    #[case("HP:0011461", "Fetal onset")]
    #[case("HP:0034199", "Late first trimester onset")]
    #[case("HP:0034198", "Second trimester onset")]
    #[case("HP:0034197", "Third trimester onset")]
    #[case("HP:0003577", "Congenital onset")]
    #[case("HP:0003623", "Neonatal onset")]
    #[case("HP:0003593", "Infantile onset")]
    #[case("HP:0011463", "Childhood onset")]
    #[case("HP:0003621", "Juvenile onset")]
    #[case("HP:0003581", "Adult onset")]
    #[case( "HP:0011462", "Young adult onset")]
    #[case("HP:0025708", "Early young adult onset")]
    #[case("HP:0025709", "Intermediate young adult onset")]
    #[case("HP:0025710", "Late young adult onset")]
    #[case("HP:0003596", "Middle age onset")]
    #[case("HP:0003584", "Late onset")]
    fn test_valid_o_class_from_str(
        #[case] ontology_clz_id: &str,
        #[case] ontology_clz_label: &str)   
    {
        let result = time_element_from_str(ontology_clz_label);
        assert!(result.is_ok());    
        let oclass_time_elem = result.unwrap();
        match oclass_time_elem.element {
            Some(time_element::Element::OntologyClass(clz)) => {
                assert_eq!(clz.label, ontology_clz_label);
                assert_eq!(clz.id, ontology_clz_id);
            }
            _ => panic!("Expected OntologyClass onset element"),
        }
    }

    #[rstest]
    #[case("P32Y")]
    #[case("P32Y2M3D")]
    #[case("P1D")]
    #[case("P3M2D")]
    #[case("P1M")]
    #[case("P42Y")]
    fn test_valid_iso8601_from_str(
        #[case] iso8601: &str
    ) {
        let result = time_element_from_str(iso8601);
        assert!(result.is_ok());    
        let oclass_time_elem = result.unwrap();
        match oclass_time_elem.element {
            Some(time_element::Element::Age(age)) => {
                assert_eq!(age.iso8601duration, iso8601);
            }
            _ => panic!("Expected Age onset element"),
        }
    }
    

}