use lazy_static::lazy_static;
use phenopackets::schema::v2::core::OntologyClass;
use crate::builder::builders::ontology_class_builder;

/// Macro to define static OntologyClass constants for HPO Onset terms
macro_rules! define_onset {
    ($($const_name:ident, $id:expr, $label:expr);* $(;)?) => {
        lazy_static! {
            $(
                pub static ref $const_name: OntologyClass = ontology_class_builder::ontology_class($id, $label).unwrap();
            )*
        }
    };
}

define_onset! {
    ANTENATAL_ONSET, "HP:0030674", "Antenatal onset";
    EMBRYONAL_ONSET, "HP:0011460", "Embryonal onset";
    FETAL_ONSET, "HP:0011461", "Fetal onset";
    LATE_FIRST_TRIMESTER_ONSET, "HP:0034199", "Late first trimester onset";
    SECOND_TRIMESTER_ONSET, "HP:0034198", "Second trimester onset";
    THIRD_TRIMESTER_ONSET, "HP:0034197", "Third trimester onset";
    CONGENITAL_ONSET, "HP:0003577", "Congenital onset";
    NEONATAL_ONSET, "HP:0003623", "Neonatal onset";
    INFANTILE_ONSET, "HP:0003593", "Infantile onset";
    CHILDHOOD_ONSET, "HP:0011463", "Childhood onset";
    JUVENILE_ONSET, "HP:0003621", "Juvenile onset";
    ADULT_ONSET, "HP:0003581", "Adult onset";
    YOUNG_ADULT_ONSET, "HP:0011462", "Young adult onset";
    EARLY_YOUNG_ADULT_ONSET, "HP:0025708", "Early young adult onset";
    INTERMEDIATE_YOUNG_ADULT_ONSET, "HP:0025709", "Intermediate young adult onset";
    LATE_YOUNG_ADULT_ONSET, "HP:0025710", "Late young adult onset";
    MIDDLE_AGE_ONSET, "HP:0003596", "Middle age onset";
    LATE_ONSET, "HP:0003584", "Late onset";
}

   		
