use phenopackets::schema::v2::core::OntologyClass;
use crate::builder::builders::ontology_class_builder;

/// Static constants for common allelic states.
pub struct AllelicState;

impl AllelicState {
    /// Heterozygous state (GENO:0000135).
    pub fn heterozygous() -> OntologyClass {
        OntologyClass {
            id: "GENO:0000135".to_string(),
            label: "heterozygous".to_string()
        }
    }

    /// Homozygous state (GENO:0000136).
    pub fn homozygous() -> OntologyClass {
        OntologyClass {
            id: "GENO:0000136".to_string(),
            label: "homozygous".to_string()
        }
    }

    /// Hemizygous state (GENO:0000134).
    pub fn hemizygous() -> OntologyClass {
        OntologyClass {
            id: "GENO:0000134".to_string(),
            label: "hemizygous".to_string()
        }
    }

    /// Unspecified zygosity (GENO:0000137).
    pub fn unspecified_zygosity() -> OntologyClass {
        OntologyClass {
            id: "GENO:0000137".to_string(),
            label: "unspecified zygosity".to_string()
        }
    }
}
