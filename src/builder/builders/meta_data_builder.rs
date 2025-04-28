use phenopackets::schema::v2::core::{MetaData, Resource, Update, ExternalReference};
use prost_types::Timestamp;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::builder::builders::time_elements;
use crate::error::{self, Error, Result};
pub struct MetaDataBuilder {
    builder: MetaData,
}

impl MetaDataBuilder {
    const SCHEMA_VERSION: &'static str = "2.0.2";

    /// Create builder from ISO8601 string
    pub fn from_created(created: Timestamp, created_by: impl Into<String>) -> Self {
        let mut metadata = MetaData::default();
        metadata.created = Some(created);
        metadata.created_by = created_by.into();
        metadata.phenopacket_schema_version = Self::SCHEMA_VERSION.to_string();
        Self { builder: metadata }
    }

    /// Create builder from ISO8601 string manually (you will need to implement a `from_iso8601` parser separately)
    pub fn builder_from_iso8601(created_iso: &str, created_by: impl Into<String>) -> Result<Self> {
        let created = time_elements::timestamp_from_iso8601(created_iso)?;
        Ok(Self::from_created(created, created_by))
    }

    /// Create a builder with the current time
    pub fn builder_now(created_by: impl Into<String>) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = Timestamp {
            seconds: now.as_secs() as i64,
            nanos: now.subsec_nanos() as i32,
        };
        Self::from_created(timestamp, created_by)
    }

    pub fn submitted_by(mut self, submitter: impl Into<String>) -> Self {
        self.builder.submitted_by = submitter.into();
        self
    }

    pub fn add_resource(mut self, resource: Resource) -> Self {
        self.builder.resources.push(resource);
        self
    }

    pub fn add_update(mut self, update: Update) -> Self {
        self.builder.updates.push(update);
        self
    }

    pub fn add_external_reference(mut self, reference: ExternalReference) -> Self {
        self.builder.external_references.push(reference);
        self
    }

    pub fn add_external_reference_by_id(mut self, id: impl Into<String>, description: impl Into<String>) -> Self {
        let reference = ExternalReference {
            id: id.into(),
            description: description.into(),
            reference: String::default()
        };
        self.add_external_reference(reference)
    }

    pub fn build(self) -> MetaData {
        self.builder
    }
}






#[cfg(test)]
mod tests {
    use crate::builder::builders::{external_reference_builder::ExternalReferenceBuilder, resources};

    use super::*;
    use phenopackets::schema::v2::core::Resource;
    use crate::builder::builders::resources::Resources;
    use crate::error::{self, Error, Result};
    use rstest::rstest;

    #[rstest]
    fn test_create_metadata() -> Result<()> {
        let created= "2019-07-21T00:25:54.662Z";
        let created_by = "Earnest B. Biocurator";
        let phenopacket_schema_version = "2.0.2";
        let hpo = Resources::hpo_version("v2025-03-03");
        let geno = Resources::geno_version("2023-10-08");
        let ext_res = ExternalReferenceBuilder::new()
            .id("PMID:30808312")
            .description("Bao M, et al. COL6A1 mutation leading to Bethlem myopathy with recurrent hematuria: a case report. BMC Neurol. 2019;19(1):32.")
            .build();
        let mdata = MetaDataBuilder::builder_from_iso8601(created, created_by)?
            .add_resource(hpo)
            .add_resource(geno)
            .add_external_reference(ext_res)
            .build();
        assert_eq!(created_by, mdata.created_by);
        assert_eq!(phenopacket_schema_version, mdata.phenopacket_schema_version);
        assert_eq!(2, mdata.resources.len());
        Ok(())
    }

}


/*

  
  resources:
      - id: "hp"
      name: "human phenotype ontology"
      url: "http://purl.obolibrary.org/obo/hp.owl"
      version: "2018-03-08"
      namespacePrefix: "HP"
      iriPrefix: "hp"
      - id: "geno"
      name: "Genotype Ontology"
      url: "http://purl.obolibrary.org/obo/geno.owl"
      version: "2023-10-08"
      namespacePrefix: "GENO"
      iriPrefix: "geno"
      - id: "pubmed"
      name: "PubMed"
      url: "https://www.ncbi.nlm.nih.gov/pubmed/"
      namespacePrefix: "PMID"
 
  externalReferences:
      - id: 
      description: 
 */