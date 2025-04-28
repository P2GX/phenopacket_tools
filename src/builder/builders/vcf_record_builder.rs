use std::sync::Mutex;

use phenopackets::ga4gh::vrsatile::v1::VcfRecord;

pub struct VcfRecordBuilder {
    builder: VcfRecord,
    pass_is_set: bool,
    filters: Option<Vec<String>>,
}

impl VcfRecordBuilder {
    pub fn builder(
        assembly: impl Into<String>,
        chromosome: impl Into<String>,
        position: u64,
        ref_allele: impl Into<String>,
        alt_allele: impl Into<String>,
    ) -> Self {
        Self {
            builder: VcfRecord {
                genome_assembly: assembly.into(),
                chrom: chromosome.into(),
                pos: position,
                r#ref: ref_allele.into(),
                alt: alt_allele.into(),
                ..Default::default()
            },
            pass_is_set: false,
            filters: None,
        }
    }

    pub fn of(
        assembly: impl Into<String>,
        chromosome: impl Into<String>,
        position: u64,
        ref_allele: impl Into<String>,
        alt_allele: impl Into<String>,
    ) -> VcfRecord {
        VcfRecord {
            genome_assembly: assembly.into(),
            chrom: chromosome.into(),
            pos: position,
            r#ref: ref_allele.into(),
            alt: alt_allele.into(),
            ..Default::default()
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.builder.id = id.into();
        self
    }

    pub fn qual(mut self, qual: impl Into<String>) -> Self {
        self.builder.qual = qual.into();
        self
    }

    pub fn pass(mut self) -> Self {
        self.pass_is_set = true;
        if let Some(ref mut filters) = self.filters {
            filters.clear();
        }
        self
    }

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        let filter = filter.into();
        self.pass_is_set = false;

        if self.filters.is_none() {
            self.filters = Some(Vec::new());
        }

        if filter.eq_ignore_ascii_case("PASS") {
            return self.pass();
        }

        if filter.contains(';') {
            for field in filter.split(';') {
                let trimmed = field.trim();
                if !trimmed.is_empty() {
                    self = self.filter(trimmed);
                }
            }
        } else if let Some(ref mut filters) = self.filters {
            filters.push(filter);
        }

        self
    }

    pub fn info(mut self, info: impl Into<String>) -> Self {
        self.builder.info = info.into();
        self
    }

    pub fn build(mut self) -> VcfRecord {
        if self.pass_is_set {
            self.builder.filter = "PASS".to_string();
        } else if let Some(filters) = self.filters {
            if !filters.is_empty() {
                self.builder.filter = filters.join(";");
            }
        }
        self.builder
    }
}


#[cfg(test)]
mod tests {
    use crate::builder::builders::{ontology_class_builder, time_elements::time_element_from_str};

    use super::*;
    use phenopackets::ga4gh::vrsatile::v1::VcfRecord;
    use rstest::rstest;



    #[rstest]
    #[case("GRCh37", "chr1", 123_456, "C", "G")]
    fn test_ctor(
        #[case] assembly: &str, 
        #[case] chr: &str, 
        #[case] position: u64, 
        #[case] ref_allele: &str, 
        #[case] alt_allele: &str, 
    ) {
        let vcf = VcfRecordBuilder::builder(assembly, chr, position, ref_allele, alt_allele)
                .build();
        assert_eq!(assembly, vcf.genome_assembly);
    }

}