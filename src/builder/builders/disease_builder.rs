use phenopackets::schema::v2::core::{Disease, OntologyClass, TimeElement};

pub struct DiseaseBuilder {
    term: OntologyClass,
    excluded: bool,
    onset: Option<TimeElement>, 
    resolution: Option<TimeElement>, 
    disease_stage: Vec<OntologyClass>,
    clinical_tnm_finding: Vec<OntologyClass>,
    primary_site: Option<OntologyClass>,
    laterality: Option<OntologyClass>,
}

impl DiseaseBuilder {
    pub fn of_term(term: OntologyClass) -> Disease {
        Disease {
            term: Some(term),
            excluded: false,
            onset: None, 
            resolution: None,
            disease_stage: vec![],
            clinical_tnm_finding: vec![],
            primary_site: None,
            laterality: None,
        }
    }

    pub fn of(id: impl Into<String>, label: impl Into<String>) -> Disease {
        let term = OntologyClass { id: id.into(), label: label.into() };
        Self::of_term(term)
    }


    pub fn builder(id: impl Into<String>, label: impl Into<String>) -> Self {
        let term = OntologyClass { id: id.into(), label: label.into() };
        Self { 
            term: term,
            excluded: false,
            onset: None, 
            resolution: None,
            disease_stage: vec![],
            clinical_tnm_finding: vec![],
            primary_site: None,
            laterality: None,
        }
    }

    pub fn excluded(mut self) -> Self {
        self.excluded = true;
        self
    }

    pub fn onset(mut self, onset: TimeElement) -> Self {
        self.onset= Some(onset);
        self
    }

    pub fn resolution(mut self, resolution: TimeElement) -> Self {
        self.resolution = Some(resolution);
        self
    }

    pub fn add_disease_stage(mut self, stage: OntologyClass) -> Self {
        self.disease_stage.push(stage);
        self
    }

    pub fn add_clinical_tnm_finding(mut self, tnm_finding: OntologyClass) -> Self {
        self.clinical_tnm_finding.push(tnm_finding);
        self
    }

    pub fn primary_site(mut self, site: OntologyClass) -> Self {
        self.primary_site = Some(site);
        self
    }

    pub fn laterality(mut self, laterality: OntologyClass) -> Self {
        self.laterality = Some(laterality);
        self
    }

    pub fn build(self) -> Disease {
        Disease { term: Some(self.term), 
            excluded: self.excluded, 
            onset: self.onset, 
            resolution: self.resolution, 
            disease_stage: self.disease_stage, 
            clinical_tnm_finding: self.clinical_tnm_finding, 
            primary_site: self.primary_site, 
            laterality: self.laterality
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{builder::builders::time_elements, constants::disease_stage::DiseaseStage};

    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    fn cardiomyopathy() {
        let disease_id = "MONDO:0004994";
        let disease_label = "cardiomyopathy";
        let disease = DiseaseBuilder::of(disease_id, disease_label);
        assert_eq!(disease_id, disease.term.unwrap().id);
    }

    #[rstest]
    fn cardiomyopathy_with_stage() {
        let disease_id = "MONDO:0004994";
        let disease_label = "cardiomyopathy";
        let stage = "New York Heart Association Class IV";
        let disease = DiseaseBuilder::builder(disease_id, disease_label)
            .add_disease_stage(DiseaseStage::nyha_class_iv())
            .build();
        let disease_term = disease.term.unwrap();
        assert_eq!(disease_id, disease_term.id);
        assert_eq!(disease_label, disease_term.label);
        assert_eq!(stage, disease.disease_stage[0].label);
    }

    #[rstest]
    fn sca_with_onset() {
        let disease_id = "OMIM:164400";
        let disease_label = "Spinocerebellar ataxia 1";
        let onset = time_elements::age("P38Y7M").unwrap();
        let disease = DiseaseBuilder::builder(disease_id, disease_label)
            .onset(onset.clone())
            .build();
        let disease_term = disease.term.unwrap();
        assert_eq!(disease_id, disease_term.id);
        assert_eq!(disease_label, disease_term.label);
        assert!(disease.onset.is_some());
        let onset_age = disease.onset.unwrap();
        assert_eq!(onset_age, onset);
    }

}

