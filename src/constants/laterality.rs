
/// Constants for laterality of findings.

use phenopackets::schema::v2::core::OntologyClass;


pub fn right() -> OntologyClass {
    OntologyClass {
        id: "HP:0012834".to_string(),
        label: "Right".to_string()
    }
}

pub fn left() -> OntologyClass {
    OntologyClass {
        id: "HP:0012835".to_string(),
        label: "Left".to_string()
    }
}

pub fn unilateral() -> OntologyClass {
    OntologyClass {
        id: "HP:0012833".to_string(),
        label: "Unilateral".to_string()
    }
}

pub fn bilateral() -> OntologyClass {
    OntologyClass {
        id: "HP:0012832".to_string(),
        label: "Bilateral".to_string()
    }
}



