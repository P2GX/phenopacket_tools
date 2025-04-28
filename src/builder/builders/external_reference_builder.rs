use phenopackets::schema::v2::core::ExternalReference;

pub struct ExternalReferenceBuilder {
    builder: ExternalReference,
}

impl ExternalReferenceBuilder {
    /// Create a new empty builder
    pub fn new() -> Self {
        Self {
            builder: ExternalReference::default(),
        }
    }

    /// Quickly create an ExternalReference with id and description
    pub fn of(id: impl Into<String>, description: impl Into<String>) -> ExternalReference {
        ExternalReference {
            id: id.into(),
            description: description.into(),
            ..Default::default()
        }
    }

    /// Set the `id` field
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.builder.id = id.into();
        self
    }

    /// Set the `reference` field
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.builder.reference = reference.into();
        self
    }

    /// Set the `description` field
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.builder.description = description.into();
        self
    }

    /// Finalize and return the ExternalReference
    pub fn build(self) -> ExternalReference {
        self.builder
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use phenopackets::schema::v2::core::ExternalReference;
    use rstest::rstest;


    #[test]
    fn test_two_element_external_reference()  {
        let id = "PMID:30962759";
        let description = "Recurrent Erythema Nodosum in a Child with a SHOC2 Gene Mutation";
       let ext_ref = ExternalReferenceBuilder::of(id, description);
       assert_eq!(id, ext_ref.id);
       assert_eq!(description, ext_ref.description);
       assert_eq!("", ext_ref.reference);
    
    }

    #[test]
    fn test_three_element_external_reference()  {
        let id = "PMID:30962759";
        let description = "Recurrent Erythema Nodosum in a Child with a SHOC2 Gene Mutation";
        let reference = "https://pubmed.ncbi.nlm.nih.gov/30962759";
       let ext_ref = ExternalReferenceBuilder::new()
            .id(id)
            .description(description)
            .reference(reference)
            .build();
       assert_eq!(id, ext_ref.id);
       assert_eq!(description, ext_ref.description);
       assert_eq!(reference, ext_ref.reference);
    
    }

}