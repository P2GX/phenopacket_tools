use phenopackets::schema::v2::core::{genomic_interpretation, Diagnosis, GenomicInterpretation, OntologyClass};

use super::ontology_class_builder;
use crate::error::{self, Error, Result};
pub struct DiagnosisBuilder {
    disease: OntologyClass,
    genomic_interpretations: Vec<GenomicInterpretation>
}

impl DiagnosisBuilder {
    pub fn new(disease: OntologyClass) -> Self {
        Self {
            disease: disease,
            genomic_interpretations: vec![]
        }
    }

    pub fn builder(id: impl Into<String>, label: impl Into<String>) -> Result<Self> {
        let clz = ontology_class_builder::ontology_class(id, label)?;
        Ok(DiagnosisBuilder::new(clz))
    }

    pub fn add_genomic_interpretation(mut self, interpretation: GenomicInterpretation ) -> Self {
        self.genomic_interpretations.push(interpretation);
        self
    }

    pub fn build(self) -> Diagnosis {
        Diagnosis { 
            disease: Some(self.disease), 
            genomic_interpretations: self.genomic_interpretations
        }
    }
       
}


