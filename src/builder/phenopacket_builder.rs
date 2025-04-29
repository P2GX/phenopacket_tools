use phenopackets::schema::v2::core::{Biosample, Disease, File, Individual, Interpretation, Measurement, MedicalAction, MetaData, PhenotypicFeature};
use phenopackets::schema::v2::Phenopacket;
use crate::error::{self, Error, Result};
use crate::builder::builders::ontology_class_builder;

use super::builders::phenotypic_feature_builder::PhenotypicFeatureBuilder;


pub struct PhenopacketBuilder {
    id: String,
    subject: Option<Individual>,
    phenotypic_features: Vec<PhenotypicFeature>,
    measurements: Vec<Measurement>,
    biosamples: Vec<Biosample>, 
    interpretations: Vec<Interpretation>,
    diseases: Vec<Disease>, 
    medical_actions: Vec<MedicalAction>,
    files: Vec<File>,
    meta_data: MetaData
}

impl PhenopacketBuilder {
    pub fn builder(id: impl Into<String>, metadata: MetaData) -> Self {
        Self {
            id: id.into(),
            subject: None,
            phenotypic_features: vec![],
            measurements: vec![],
            biosamples: vec![],
            interpretations: vec![],
            diseases: vec![],
            medical_actions: vec![],
            files: vec![],
            meta_data: metadata
        }
    }

    pub fn individual(mut self, subject: Individual) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn add_phenotypic_feature_from_str(mut self, term_id: impl Into<String>, term_label: impl Into<String>) -> Result<Self> {
        let clz = ontology_class_builder::ontology_class(term_id, term_label)?;
        let pf = PhenotypicFeatureBuilder::of(clz);
        self.phenotypic_features.push(pf);
        Ok(self)
    }

    pub fn add_phenotypic_feature(mut self, feature: PhenotypicFeature) -> Self {
        self.phenotypic_features.push(feature);
        self
    }
    pub fn add_phenotypic_features(mut self, features: Vec<PhenotypicFeature>) -> Self {
        self.phenotypic_features.extend(features);
        self
    }

    pub fn add_measurement(mut self, measurement: Measurement) -> Self {
        self.measurements.push(measurement);
        self
    }

    pub fn add_measurements(mut self, measurements: Vec<Measurement>) -> Self {
        self.measurements.extend(measurements);
        self
    }

    pub fn add_biosample(mut self, sample: Biosample) -> Self {
        self.biosamples.push(sample);
        self
    }

    pub fn add_biosamples(mut self, samples: Vec<Biosample>) -> Self {
        self.biosamples.extend(samples);
        self
    }

    pub fn add_interpretation(mut self, interpretation: Interpretation) -> Self {
        self.interpretations.push(interpretation);
        self
    }

    pub fn add_disease(mut self, disease: Disease) -> Self {
        self.diseases.push(disease);
        self
    }

    pub fn add_diseases(mut self, diseases: Vec<Disease>) -> Self {
        self.diseases.extend(diseases);
        self
    }

    pub fn add_medical_action(mut self, action: MedicalAction) -> Self {
        self.medical_actions.push(action);
        self
    }
    pub fn add_medical_actions(mut self, actions: Vec<MedicalAction>) -> Self {
        self.medical_actions.extend(actions);
        self
    }

    pub fn add_file(mut self, file: File) -> Self {
        self.files.push(file);
        self
    }

    pub fn build(self) -> Phenopacket {
        Phenopacket { 
            id: self.id, 
            subject: self.subject, 
            phenotypic_features: self.phenotypic_features, 
            measurements: self.measurements, 
            biosamples: self.biosamples, 
            interpretations: self.interpretations, 
            diseases: self.diseases, 
            medical_actions: self.medical_actions, 
            files: self.files, 
            meta_data: Some(self.meta_data)
         }
    }

}

