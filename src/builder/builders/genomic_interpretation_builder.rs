use phenopackets::{ga4gh::vrsatile::v1::GeneDescriptor, schema::v2::core::{genomic_interpretation::InterpretationStatus, GenomicInterpretation, VariantInterpretation}};
use phenopackets::schema::v2::core::genomic_interpretation::Call; // Make sure you import the correct Call!

use crate::error::{self, Error, Result};


pub struct GenomicInterpretationBuilder {
    subject_or_biosample_id: String,
    interpretation_status: InterpretationStatus,
    gene_descriptor: Option<GeneDescriptor>,
    variant_interpretation: Option<VariantInterpretation>
}


impl Error {
    fn double_oneof() -> Self {
        Error::GenomicInterpretationError { msg: format!("Can only use one of GeneDescriptor or VariantInterpretation") }
    }

    fn no_call() -> Self {
        Error::GenomicInterpretationError { msg: format!("Neither GeneDescriptor or VariantInterpretation was initialized") }
    }

    fn no_status() -> Self {
        Error::GenomicInterpretationError { msg: format!("Interpretation status not initialized") }
    }
}

impl GenomicInterpretationBuilder {
    
    pub fn builder(identifier: impl Into<String>) -> Self {
        Self {
            subject_or_biosample_id: identifier.into(),
            interpretation_status: InterpretationStatus::UnknownStatus,
            gene_descriptor: None,
            variant_interpretation: None
        }
    }

    pub fn rejected(mut self) -> Self {
        self.interpretation_status = InterpretationStatus::Rejected;
        self
    }

    pub fn candidate(mut self) -> Self {
        self.interpretation_status = InterpretationStatus::Candidate;
        self
    }

    pub fn contributory(mut self) -> Self {
        self.interpretation_status = InterpretationStatus::Contributory;
        self
    }

    pub fn causative(mut self) -> Self {
        self.interpretation_status = InterpretationStatus::Causative;
        self
    }

    pub fn unknown(mut self) -> Self {
        self.interpretation_status = InterpretationStatus::UnknownStatus;
        self
    }

    pub fn gene_descriptor(mut self, gdesc: GeneDescriptor) -> Result<Self> {
        if self.variant_interpretation.is_some() {
            return Err(Error::double_oneof());
        } else {
            self.gene_descriptor = Some(gdesc);
        }
        Ok(self)
    }

    pub fn variant_interpretation(mut self, vinterp: VariantInterpretation) -> Result<Self> {
        if self.gene_descriptor.is_some() {
            return Err(Error::double_oneof()); // cannot add GeneDescriptor if we already have VariantInterpretation
        } else {
            self.variant_interpretation = Some(vinterp);
        }
        Ok(self)
    }

    pub fn build(self) -> Result<GenomicInterpretation> {
        if self.interpretation_status == InterpretationStatus::UnknownStatus {
            return Err(Error::no_status());
        }
        if self.gene_descriptor.is_some() {
            return Ok(GenomicInterpretation { 
                subject_or_biosample_id:self.subject_or_biosample_id, 
                interpretation_status: self.interpretation_status.into(), 
                call: Some(Call::Gene(self.gene_descriptor.unwrap())) 
            });
        } else if self.variant_interpretation.is_some() {
            return Ok(GenomicInterpretation{ 
                subject_or_biosample_id:self.subject_or_biosample_id, 
                interpretation_status: self.interpretation_status.into(), 
                call: Some(Call::VariantInterpretation(self.variant_interpretation.unwrap()))
            });
        } else {
            return Err(Error::no_call());
        }
    }

}

