use phenopackets::schema::v2::core::{vital_status::Status, OntologyClass, TimeElement, VitalStatus};

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
        Self::new(phenopackets::schema::v2::core::vital_status::Status::Alive)
    }

    pub fn deceased() -> Self {
        Self::new(phenopackets::schema::v2::core::vital_status::Status::Deceased)
    }

    pub fn time_of_death(mut self, time: TimeElement) -> Self {
        self.time_of_death = Some(time);
        self
    }

    pub fn cause_of_death(mut self, cause: OntologyClass) -> Self {
        self.cause_of_death = Some(cause);
        self
    }

    pub fn cause_of_death_str(mut self, id: &str, label: &str) -> Self {
        self.cause_of_death = Some(OntologyClass {
            id: id.to_string(),
            label: label.to_string(),
            ..Default::default()
        });
        self
    }

    pub fn survival_time_in_days(mut self, days: i32) -> Self {
        self.survival_time_in_days = Some(days);
        self
    }

    pub fn build(self) -> VitalStatus {
        VitalStatus {
            status: self.status.unwrap_or(phenopackets::schema::v2::core::vital_status::Status::UnknownStatus as i32),
            time_of_death: self.time_of_death,
            cause_of_death: self.cause_of_death,
            survival_time_in_days: self.survival_time_in_days.unwrap_or(0) as u32,
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};


}