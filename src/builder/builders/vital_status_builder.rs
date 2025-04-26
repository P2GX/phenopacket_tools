use phenopackets::schema::v2::core::{vital_status::Status, OntologyClass, TimeElement, VitalStatus};
use crate::error::{self, Error, Result};

use super::ontology_class_builder;
/// VitalStatusBuilder
/// 
/// - status (Status, REQUIRED) one of UNKNOWN_STATUS, ALIVE, DECEASED. REQUIRED.
/// - time_of_death (TimeElement: 0..1) Should be left blank if patient not known to be deceased
/// - cause_of_death (OntologyClass; 0..1) Should be left blank if patient not known to be deceased
/// - survival_time_in_days (integer; 0..1) Number of days the patient was alive after their primary diagnosis
#[derive(Default)]
pub struct VitalStatusBuilder {
    status: Option<i32>,
    time_of_death: Option<TimeElement>,
    cause_of_death: Option<OntologyClass>,
    survival_time_in_days: Option<i32>,
}

impl VitalStatusBuilder {
    /// Begin building a `VitalStatus` with the given status (e.g., ALIVE or DECEASED).
    pub fn new(status: phenopackets::schema::v2::core::vital_status::Status) -> Self {
        Self {
            status: Some(status as i32),
            ..Default::default()
        }
    }

    pub fn alive() -> Self {
        Self::new(phenopackets::schema::v2::core::vital_status::Status::Deceased)
    }

    pub fn deceased() -> Self {
        Self::new(phenopackets::schema::v2::core::vital_status::Status::Deceased)
    }

    pub fn time_of_death(mut self, time: TimeElement) -> Result<Self> {
        self.time_of_death = Some(time);
        Ok(self)
    }

    pub fn cause_of_death(mut self, cause: OntologyClass) -> Result<Self> {
        self.cause_of_death = Some(cause);
        Ok(self)
    }

    pub fn cause_of_death_str(mut self, id: &str, label: &str) -> Result<Self> {
        self.cause_of_death = Some(ontology_class_builder::ontology_class(id, label)?);
        Ok(self)
    }

    pub fn survival_time_in_days(mut self, days: i32) -> Self {
        self.survival_time_in_days = Some(days);
        self
    }

    pub fn build(self) -> Result<VitalStatus> {
        Ok(VitalStatus {
            status: self.status.unwrap_or(phenopackets::schema::v2::core::vital_status::Status::UnknownStatus as i32),
            time_of_death: self.time_of_death,
            cause_of_death: self.cause_of_death,
            survival_time_in_days: self.survival_time_in_days.unwrap_or(0) as u32,
        })
    }
}



#[cfg(test)]
mod test {
    use crate::builder::builders::{ontology_class_builder, time_elements::time_element_from_str};

    use super::*;
    use phenopackets::schema::v2::core::time_element;
    use rstest::{fixture, rstest};


    #[rstest]
    fn test_vstatus() -> Result<()> {
        let o_clz = ontology_class_builder::ontology_class("NCIT:C2926", "Lung Non-Small Cell Carcinoma")?;
        let time_elem = time_element_from_str("P72Y")?;
        let vstatus = VitalStatusBuilder::deceased()
            .survival_time_in_days(20)
            .cause_of_death(o_clz.clone())?
            .time_of_death(time_elem)?
            .build()?;
        assert_eq!(20, vstatus.survival_time_in_days);
        match vstatus.time_of_death {
            Some(te) => match te.element {
                Some(time_element::Element::Age(ref iso)) => {
                    assert_eq!("P72Y", iso.iso8601duration);
                }
                _ => panic!("Expected TimeElement::Age"),
            },
            None => panic!("Expected Some(TimeElement)"),
        }
        assert_eq!(o_clz, vstatus.cause_of_death.unwrap());
        Ok(())       
    }

}