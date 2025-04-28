use std::fmt::format;

use chrono::NaiveDate;
use chrono::Datelike;
use phenopackets::schema::v2::core::vital_status;
use phenopackets::schema::v2::core::Individual;
use phenopackets::schema::v2::core::TimeElement;
use phenopackets::schema::{v2::core::OntologyClass, v2::core::{KaryotypicSex, Sex, VitalStatus}};
use prost_types::Timestamp;
use crate::error::{self, Error, Result};
use crate::constants::onset;

use super::ontology_class_builder;
use super::time_elements;
use super::time_elements::time_element_from_str;
use super::vital_status_builder::VitalStatusBuilder;


/// IndividualBuilder 
/// 
/// Builder to create GA4GH Phenopacket Schema Individual
/// - id (string, REQUIRED)
/// - alternate_ids (list of CURIE; 0..*):A list of alternative identifiers for the individual
/// - date_of_birth (timestamp; 0..1) A timestamp either exact or imprecise
/// - time_at_last_encounter (TimeElement; 0..1) The age or age range of the individual when last encountered. RECOMMENDED.
/// - vital_status (VitalStatus; 0..1) The vital status of the individual e.g. whether they are alive or the time and cause of death. RECOMMENDED.
/// - sex (Sex; 0..1) Observed apparent sex of the individual
/// - karyotypic_sex (KaryotypicSex; 0..1) The karyotypic sex of the individual
/// - gender (OntologyClass; 0..1) Self-identified gender
/// - taxonomy (OntologyClass; 0..1) an OntologyClass representing the species (e.g., NCBITaxon:9615)
pub struct IndividualBuilder {
    id: String,
    alternate_ids: Vec<String>,
    date_of_birth: Option<Timestamp>,
    time_at_last_encounter: Option<TimeElement>,
    vital_status: Option<VitalStatus>,
    sex: Sex,
    karyotypic_sex: KaryotypicSex,
    gender: Option<OntologyClass>,
    taxonomy: Option<OntologyClass>
}

impl Error {
    fn individual_error(msg: &str) -> Self {
        Error::IndividualError { msg: format!("{msg}") }
    }
}



pub fn timestamp(year: i32, month: u32, day: u32) -> Timestamp {
    let naive = chrono::NaiveDate::from_ymd_opt(year, month, day)
        .expect("Invalid date")
        .and_hms_opt(0, 0, 0)
        .expect("Invalid time");

    Timestamp {
        seconds: naive.and_utc().timestamp(),
        nanos: naive.and_utc().timestamp_subsec_nanos() as i32,
    }
}


impl IndividualBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self{
            id: id.into(),
            alternate_ids: vec![],
            date_of_birth: None,
            time_at_last_encounter: None,
            vital_status: None,
            sex: phenopackets::schema::v2::core::Sex::UnknownSex,
            karyotypic_sex: phenopackets::schema::v2::core::KaryotypicSex::UnknownKaryotype,
            gender: None,
            taxonomy: None
        } 
    }

    pub fn add_alternate_id(mut self, alt_id: impl Into<String>) -> Self {
        &self.alternate_ids.push(alt_id.into());
        self
    }

    pub fn add_all_alternate_ids(mut self, alt_id_list: &Vec<String>) -> Self {
        &self.alternate_ids.extend(alt_id_list.clone());
        self
    }

    pub fn date_of_birth(mut self, date: NaiveDate) -> Self {
        let ts: Timestamp = timestamp(date.year(), date.month(), date.day());
        self.date_of_birth = Some(ts);
        self
    }
    
    pub fn timestamp_at_last_encounter(mut self, timestamp_str: impl Into<String>) -> Result<Self> {
        let ts = time_element_from_str(&timestamp_str.into())?;
        self.time_at_last_encounter = Some(ts);
        Ok(self)
    }

    pub fn age_at_last_encounter(mut self, time_element: TimeElement ) -> Self {
        self.time_at_last_encounter = Some(time_element);
        self
    }

    pub fn alive(mut self) -> Result<Self>
    {
        let vstatus = VitalStatusBuilder::alive().build()?;
        self.vital_status = Some(vstatus);
        Ok(self)
    }

    pub fn deceased(mut self) -> Result<Self>
    {
        let vstatus = VitalStatusBuilder::deceased().build()?;
        self.vital_status = Some(vstatus);
        Ok(self)
    }

    pub fn vital_status(mut self, vital_status: VitalStatus) -> Self
    {
        self.vital_status = Some(vital_status);
        self
    }

    pub fn male(mut self) -> Self {
        self.sex = Sex::Male;
        self
    }

    pub fn female(mut self) -> Self {
        self.sex = Sex::Female;
        self
    }

    pub fn unknown_sex(mut self) -> Self {
        self.sex = Sex::UnknownSex;
        self
    }

    pub fn other_sex(mut self) -> Self {
        self.sex = Sex::OtherSex;
        self
    }

   

    pub fn homo_sapiens(mut self) -> Result<Self> {
        let ontology_clz = ontology_class_builder::ontology_class("NCBITaxon:9606", "Homo sapiens")?;
        self.taxonomy = Some(ontology_clz);
        Ok(self)
    }

    pub fn build(self) -> Result<Individual> {
        Ok(Individual { 
            id: self.id,
            alternate_ids: self.alternate_ids,
            date_of_birth: self.date_of_birth,
            time_at_last_encounter: self.time_at_last_encounter,
            vital_status: self.vital_status,
            sex: self.sex as i32,
            karyotypic_sex: self.karyotypic_sex as i32,
            gender: self.gender,
            taxonomy: self.taxonomy,
        })
    }
}

#[allow(non_snake_case)]
impl IndividualBuilder {
    pub fn XX(mut self) -> Self {
        self.karyotypic_sex = KaryotypicSex::Xx;
        self
    }

    pub fn XY(mut self) -> Self {
        self.karyotypic_sex = KaryotypicSex::Xy;
        self
    }

    pub fn XO(mut self) -> Self {
        self.karyotypic_sex = KaryotypicSex::Xo;
        self
    }

    pub fn XXY(mut self) -> Self {
        self.karyotypic_sex = KaryotypicSex::Xxy;
        self
    }

    pub fn XXX(mut self) -> Self {
        self.karyotypic_sex = KaryotypicSex::Xxx;
        self
    }

    pub fn XXYY(mut self) -> Self {
        self.karyotypic_sex = KaryotypicSex::Xxyy;
        self
    }

    pub fn XXXY(mut self) -> Self {
        self.karyotypic_sex = KaryotypicSex::Xxxy;
        self
    }

    pub fn XXXX(mut self) -> Self {
        self.karyotypic_sex = KaryotypicSex::Xxxx;
        self
    }
}


#[cfg(test)]
mod tests {
    use crate::builder::builders::{ontology_class_builder, time_elements::time_element_from_str};

    use super::*;
    use phenopackets::schema::v2::core::time_element;
    use rstest::rstest;

    #[rstest]
    fn test_ctor() {
        let individual_id = "II:1";
        let last_encounter = time_element_from_str("P2Y").unwrap();
        let result = IndividualBuilder::new(individual_id)
            .XX()
            .age_at_last_encounter(last_encounter.clone())
            .build();
        assert!(result.is_ok());
        let individual = result.unwrap();
        assert_eq!(last_encounter, individual.time_at_last_encounter.unwrap());
        assert_eq!(individual_id, individual.id);
        assert_eq!(KaryotypicSex::Xx as i32, individual.karyotypic_sex);
    }


}