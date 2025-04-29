use phenopackets::schema::v2::core::{Evidence, OntologyClass, PhenotypicFeature, TimeElement};

#[derive(Default)]
pub struct PhenotypicFeatureBuilder {
    builder: PhenotypicFeature,
}

impl PhenotypicFeatureBuilder {
    pub fn new(feature: OntologyClass) -> Self {
        Self {
            builder: PhenotypicFeature {
                r#type: Some(feature),
                ..Default::default()
            },
        }
    }

    pub fn of(feature: OntologyClass) -> PhenotypicFeature {
        PhenotypicFeature {
            r#type: Some(feature),
            ..Default::default()
        }
    }

    pub fn of_id_label(id: &str, label: &str) -> PhenotypicFeature {
        Self::of(OntologyClass {
            id: id.to_string(),
            label: label.to_string(),
        })
    }

    pub fn builder(feature: OntologyClass) -> Self {
        Self::new(feature)
    }

    pub fn builder_id_label(id: &str, label: &str) -> Self {
        Self::new(OntologyClass {
            id: id.to_string(),
            label: label.to_string(),
        })
    }

    pub fn onset(mut self, time: TimeElement) -> Self {
        self.builder.onset = Some(time);
        self
    }

    pub fn resolution(mut self, time: TimeElement) -> Self {
        self.builder.resolution = Some(time);
        self
    }

    pub fn severity(mut self, severity: OntologyClass) -> Self {
        self.builder.severity = Some(severity);
        self
    }

    pub fn severity_id_label(mut self, id: &str, label: &str) -> Self {
        self.severity(OntologyClass {
            id: id.to_string(),
            label: label.to_string(),
        })
    }

    pub fn excluded(mut self) -> Self {
        self.builder.excluded = true;
        self
    }

    pub fn add_evidence(mut self, evidence: Evidence) -> Self {
        self.builder.evidence.push(evidence);
        self
    }

    pub fn add_all_evidence(mut self, evidence_list: Vec<Evidence>) -> Self {
        self.builder.evidence.extend(evidence_list);
        self
    }

    pub fn add_modifier(mut self, modifier: OntologyClass) -> Self {
        self.builder.modifiers.push(modifier);
        self
    }

    pub fn add_all_modifiers(mut self, modifiers: Vec<OntologyClass>) -> Self {
        self.builder.modifiers.extend(modifiers);
        self
    }

    pub fn description(mut self, text: &str) -> Self {
        self.builder.description = text.to_string();
        self
    }

    pub fn build(self) -> PhenotypicFeature {
        self.builder
    }
}
