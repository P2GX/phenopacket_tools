use phenopackets::ga4gh::vrsatile::v1::VariationDescriptor;
use phenopackets::schema::v2::core::{
    AcmgPathogenicityClassification, TherapeuticActionability, VariantInterpretation,
};

pub struct VariantInterpretationBuilder {
    builder: VariantInterpretation,
}

impl VariantInterpretationBuilder {
    pub fn new(descriptor: VariationDescriptor) -> Self {
        Self {
            builder: VariantInterpretation {
                variation_descriptor: Some(descriptor),
                ..Default::default()
            },
        }
    }

  

    pub fn acmg_not_provided(mut self) -> Self {
        self.builder.acmg_pathogenicity_classification =
            AcmgPathogenicityClassification::NotProvided.into();
        self
    }

    pub fn benign(mut self) -> Self {
        self.builder.acmg_pathogenicity_classification =
            AcmgPathogenicityClassification::Benign.into();
        self
    }

    pub fn likely_benign(mut self) -> Self {
        self.builder.acmg_pathogenicity_classification =
            AcmgPathogenicityClassification::LikelyBenign.into();
        self
    }

    pub fn uncertain_significance(mut self) -> Self {
        self.builder.acmg_pathogenicity_classification =
            AcmgPathogenicityClassification::UncertainSignificance.into();
        self
    }

    pub fn likely_pathogenic(mut self) -> Self {
        self.builder.acmg_pathogenicity_classification =
            AcmgPathogenicityClassification::LikelyPathogenic.into();
        self
    }

    pub fn pathogenic(mut self) -> Self {
        self.builder.acmg_pathogenicity_classification =
            AcmgPathogenicityClassification::Pathogenic.into();
        self
    }

    pub fn actionability_unknown(mut self) -> Self {
        self.builder.therapeutic_actionability =
            TherapeuticActionability::UnknownActionability.into();
        self
    }

    pub fn not_actionable(mut self) -> Self {
        self.builder.therapeutic_actionability =
            TherapeuticActionability::NotActionable.into();
        self
    }

    pub fn actionable(mut self) -> Self {
        self.builder.therapeutic_actionability =
            TherapeuticActionability::Actionable.into();
        self
    }

    pub fn build(self) -> VariantInterpretation {
        self.builder
    }

    /// Equivalent of `of(VariationDescriptor, AcmgPathogenicityClassification)`
    pub fn of(
        descriptor: VariationDescriptor,
        acmg: AcmgPathogenicityClassification,
    ) -> VariantInterpretation {
        VariantInterpretation {
            variation_descriptor: Some(descriptor),
            acmg_pathogenicity_classification: acmg.into(),
            therapeutic_actionability: TherapeuticActionability::UnknownActionability.into(),
            ..Default::default()
        }
    }

    /// Equivalent of `of(VariationDescriptor, AcmgPathogenicityClassification, TherapeuticActionability)`
    pub fn of_with_actionability(
        descriptor: VariationDescriptor,
        acmg: AcmgPathogenicityClassification,
        actionability: TherapeuticActionability,
    ) -> VariantInterpretation {
        VariantInterpretation {
            variation_descriptor: Some(descriptor),
            acmg_pathogenicity_classification: acmg.into(),
            therapeutic_actionability: actionability.into(),
            ..Default::default()
        }
    }
}
