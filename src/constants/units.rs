use phenopackets::schema::v2::core::OntologyClass;

pub struct Unit;

impl Unit {
    pub fn degree() -> OntologyClass {
        OntologyClass { id: "UCUM:degree".to_string(), label: "degree (plane angle)".to_string() }
    }

    pub fn diopter() -> OntologyClass {
        OntologyClass { id: "UCUM:[diop]".to_string(), label: "diopter".to_string() }
    }

    pub fn gram() -> OntologyClass {
        OntologyClass { id: "UCUM:g".to_string(), label: "gram".to_string() }
    }

    pub fn gram_per_kilogram() -> OntologyClass {
        OntologyClass { id: "UCUM:g.kg-1".to_string(), label: "gram per kilogram".to_string() }
    }

    pub fn kilogram() -> OntologyClass {
        OntologyClass { id: "UCUM:kg".to_string(), label: "kilogram".to_string() }
    }

    pub fn liter() -> OntologyClass {
        OntologyClass { id: "UCUM:L".to_string(), label: "liter".to_string() }
    }

    pub fn meter() -> OntologyClass {
        OntologyClass { id: "UCUM:m".to_string(), label: "meter".to_string() }
    }

    pub fn microgram() -> OntologyClass {
        OntologyClass { id: "UCUM:ug".to_string(), label: "microgram".to_string() }
    }

    pub fn microgram_per_deciliter() -> OntologyClass {
        OntologyClass { id: "UCUM:ug.dL-1".to_string(), label: "microgram per deciliter".to_string() }
    }

    pub fn microgram_per_liter() -> OntologyClass {
        OntologyClass { id: "UCUM:ug.L-1".to_string(), label: "microgram per liter".to_string() }
    }

    pub fn microliter() -> OntologyClass {
        OntologyClass { id: "UCUM:uL".to_string(), label: "microliter".to_string() }
    }

    pub fn micrometer() -> OntologyClass {
        OntologyClass { id: "UCUM:um".to_string(), label: "micrometer".to_string() }
    }

    pub fn milligram() -> OntologyClass {
        OntologyClass { id: "UCUM:mg".to_string(), label: "milligram".to_string() }
    }

    pub fn milligram_per_day() -> OntologyClass {
        OntologyClass { id: "UCUM:mg.d-1".to_string(), label: "milligram per day".to_string() }
    }

    pub fn milligram_per_deciliter() -> OntologyClass {
        OntologyClass { id: "UCUM:mg.dL-1".to_string(), label: "milligram per deciliter".to_string() }
    }

    pub fn mg_per_kg() -> OntologyClass {
        OntologyClass { id: "UCUM:mg.kg-1".to_string(), label: "milligram per kilogram".to_string() }
    }

    pub fn milliliter() -> OntologyClass {
        OntologyClass { id: "UCUM:mL".to_string(), label: "milliliter".to_string() }
    }

    pub fn millimeter() -> OntologyClass {
        OntologyClass { id: "UCUM:mm".to_string(), label: "millimeter".to_string() }
    }

    pub fn mm_hg() -> OntologyClass {
        OntologyClass { id: "UCUM:mm[Hg]".to_string(), label: "millimetres of mercury".to_string() }
    }

    pub fn millimole() -> OntologyClass {
        OntologyClass { id: "UCUM:mmol".to_string(), label: "millimole".to_string() }
    }

    pub fn mole() -> OntologyClass {
        OntologyClass { id: "UCUM:mol".to_string(), label: "mole".to_string() }
    }

    pub fn mole_per_liter() -> OntologyClass {
        OntologyClass { id: "UCUM:mol.L-1".to_string(), label: "mole per liter".to_string() }
    }

    pub fn mole_per_milliliter() -> OntologyClass {
        OntologyClass { id: "UCUM:mol.mL-1".to_string(), label: "mole per milliliter".to_string() }
    }

    pub fn enzyme_unit_per_liter() -> OntologyClass {
        OntologyClass { id: "UCUM:U.L-1".to_string(), label: "enzyme unit per liter".to_string() }
    }
}
