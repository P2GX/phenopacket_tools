use std::collections::HashMap;

use lazy_static::lazy_static;
use phenopackets::schema::v2::core::OntologyClass;
use crate::builders::builder::Builder;

/// Macro to define static OntologyClass constants for HPO Onset terms
macro_rules! define_onset {
    ($($const_name:ident, $id:expr, $label:expr);* $(;)?) => {
        lazy_static! {
            $(
                pub static ref $const_name: OntologyClass = Builder::ontology_class($id, $label).unwrap();
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


lazy_static! {
    pub static ref ONSET_CLASSES_BY_LABEL: HashMap<String, OntologyClass> = {
        let mut map = HashMap::new();
        map.insert(ANTENATAL_ONSET.label.to_string(), ANTENATAL_ONSET.clone());
        map.insert(EMBRYONAL_ONSET.label.to_string(), EMBRYONAL_ONSET.clone());
        map.insert(FETAL_ONSET.label.to_string(), FETAL_ONSET.clone());
        map.insert(LATE_FIRST_TRIMESTER_ONSET.label.to_string(), LATE_FIRST_TRIMESTER_ONSET.clone());
        map.insert(SECOND_TRIMESTER_ONSET.label.to_string(), SECOND_TRIMESTER_ONSET.clone());
        map.insert(THIRD_TRIMESTER_ONSET.label.to_string(), THIRD_TRIMESTER_ONSET.clone());
        map.insert(CONGENITAL_ONSET.label.to_string(), CONGENITAL_ONSET.clone());
        map.insert(NEONATAL_ONSET.label.to_string(), NEONATAL_ONSET.clone());
        map.insert(INFANTILE_ONSET.label.to_string(), INFANTILE_ONSET.clone());
        map.insert(CHILDHOOD_ONSET.label.to_string(), CHILDHOOD_ONSET.clone());
        map.insert(JUVENILE_ONSET.label.to_string(), JUVENILE_ONSET.clone());
        map.insert(ADULT_ONSET.label.to_string(), ADULT_ONSET.clone());
        map.insert(YOUNG_ADULT_ONSET.label.to_string(), YOUNG_ADULT_ONSET.clone());
        map.insert(EARLY_YOUNG_ADULT_ONSET.label.to_string(), EARLY_YOUNG_ADULT_ONSET.clone());
        map.insert(INTERMEDIATE_YOUNG_ADULT_ONSET.label.to_string(), INTERMEDIATE_YOUNG_ADULT_ONSET.clone());
        map.insert(LATE_YOUNG_ADULT_ONSET.label.to_string(), LATE_YOUNG_ADULT_ONSET.clone());
        map.insert(MIDDLE_AGE_ONSET.label.to_string(), MIDDLE_AGE_ONSET.clone());
        map.insert(LATE_ONSET.label.to_string(), LATE_ONSET.clone());
        map
    };
}



pub fn get_onset_by_label(label: &str) -> Option<&OntologyClass> {
    ONSET_CLASSES_BY_LABEL.get(label)
}
