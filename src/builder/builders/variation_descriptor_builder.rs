use phenopackets::ga4gh::vrs::v1::Variation;
use phenopackets::ga4gh::vrsatile::v1::{VariationDescriptor, MoleculeContext, GeneDescriptor, VcfRecord, Expression};
use phenopackets::schema::v2::core::OntologyClass;


use crate::constants::allelic_state::AllelicState;

use super::extensions::Extensions;
use super::vcf_record_builder::VcfRecordBuilder;

pub struct VariationDescriptorBuilder {
    builder: VariationDescriptor,
}

impl VariationDescriptorBuilder {

    pub fn builder(id: impl Into<String>) -> Self {
        Self {
            builder: VariationDescriptor {
                id: id.into(),
                ..Default::default()
            },
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.builder.label = label.into();
        self
    }

    pub fn variation(mut self, variation: Variation) -> Self {
        self.builder.variation = Some(variation);
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.builder.description = desc.into();
        self
    }

    pub fn gene_context(mut self, gene: GeneDescriptor) -> Self {
        self.builder.gene_context = Some(gene);
        self
    }

    pub fn vcf_record(mut self, vcf: VcfRecord) -> Self {
        self.builder.vcf_record = Some(vcf);
        self
    }

    pub fn add_xref(mut self, xref: impl Into<String>) -> Self {
        self.builder.xrefs.push(xref.into());
        self
    }

    pub fn add_all_xrefs(mut self, xrefs: impl IntoIterator<Item = String>) -> Self {
        self.builder.xrefs.extend(xrefs);
        self
    }

    pub fn add_alternate_label(mut self, label: impl Into<String>) -> Self {
        self.builder.alternate_labels.push(label.into());
        self
    }

    pub fn add_all_alternate_labels(mut self, labels: impl IntoIterator<Item = String>) -> Self {
        self.builder.alternate_labels.extend(labels);
        self
    }

    pub fn genomic(mut self) -> Self {
        self.builder.molecule_context = MoleculeContext::Genomic as i32;
        self
    }

    pub fn protein(mut self) -> Self {
        self.builder.molecule_context = MoleculeContext::Protein as i32;
        self
    }

    pub fn transcript(mut self) -> Self {
        self.builder.molecule_context = MoleculeContext::Transcript as i32;
        self
    }

    pub fn molecule_context(mut self, context: MoleculeContext) -> Self {
        self.builder.molecule_context = context as i32;
        self
    }

    pub fn structural_type(mut self, structural_type: OntologyClass) -> Self {
        self.builder.structural_type = Some(structural_type);
        self
    }

    pub fn heterozygous(mut self) -> Self {
        self.builder.allelic_state = Some(AllelicState::heterozygous());
        self
    }

    pub fn homozygous(mut self) -> Self {
        self.builder.allelic_state = Some(AllelicState::homozygous());
        self
    }

    pub fn hemizygous(mut self) -> Self {
        self.builder.allelic_state = Some(AllelicState::hemizygous());
        self
    }

    pub fn unspecified_zygosity(mut self) -> Self {
        self.builder.allelic_state = Some(AllelicState::unspecified_zygosity());
        self
    }

    pub fn zygosity(mut self, zygosity: OntologyClass) -> Self {
        self.builder.allelic_state = Some(zygosity);
        self
    }

    pub fn hgvs(mut self, value: impl Into<String>) -> Self {
        let expr = Expression {
            syntax: "hgvs".to_string(),
            value: value.into(),
            ..Default::default()
        };
        self.builder.expressions.push(expr);
        self
    }

    pub fn spdi(mut self, value: impl Into<String>) -> Self {
        let expr = Expression {
            syntax: "spdi".to_string(),
            value: value.into(),
            ..Default::default()
        };
        self.builder.expressions.push(expr);
        self
    }

    pub fn iscn(mut self, value: impl Into<String>) -> Self {
        let expr = Expression {
            syntax: "iscn".to_string(),
            value: value.into(),
            ..Default::default()
        };
        self.builder.expressions.push(expr);
        self
    }

    pub fn add_expression(mut self, expr: Expression) -> Self {
        self.builder.expressions.push(expr);
        self
    }

    pub fn vcf_hg38(mut self, chromosome: impl Into<String>, position: u64, ref_allele: impl Into<String>, alt_allele: impl Into<String>) -> Self {
        let vcf = VcfRecordBuilder::of("GRCh38", chromosome, position, ref_allele, alt_allele);
        self.builder.vcf_record = Some(vcf);
        self
    }

    pub fn vcf_hg37(mut self, chromosome: impl Into<String>, position: u64, ref_allele: impl Into<String>, alt_allele: impl Into<String>) -> Self {
        let vcf = VcfRecordBuilder::of("GRCh37", chromosome, position, ref_allele, alt_allele);
        self.builder.vcf_record = Some(vcf);
        self
    }

    pub fn mosaicism(mut self, percentage: f64) -> Self {
        let extension = Extensions::mosaicism(percentage);
        self.builder.extensions.push(extension);
        self
    }

    pub fn allele_frequency(mut self, frequency: f64) -> Self {
        let extension = Extensions::allele_frequency(frequency);
        self.builder.extensions.push(extension);
        self
    }

    pub fn build(self) -> VariationDescriptor {
        self.builder
    }
}


#[cfg(test)]
mod tests {
    use crate::builder::builders::{expressions::Expressions, gene_descriptor_builder::GeneDescriptorBuilder, ontology_class_builder, time_elements::time_element_from_str};

    use super::*;
    use phenopackets::ga4gh::{vrs::v1::{abundance, variation}, vrsatile::v1::Extension};
    use rstest::rstest;
    use serde_json::de;


    #[rstest]
    fn test_clinvar_13294() {
        let identifier = "var_AlnzRCyPurQrLcCeHYebXZRUb";
        let value_id = "HGNC:25230";
        let gene_symbol = "AHDC1";
        let gene_desc = GeneDescriptorBuilder::of(value_id, gene_symbol);
        let hgvs_c = "NM_001371928.1:c.2898del";
        let hgvs_c_expr = Expressions::hgvs_cdna(hgvs_c);
        let hgvs_g = "NC_000001.11:g.27549219del";
        let hgvs_g_expr = Expressions::hgvs_genomic(hgvs_g);
        let ref_allele = "AG";
        let alt_allele = "A";
        let pos = 27549217 as u64;
        let chromosome = "chr1";
        let assembly = "hg38";
        let vcf = VcfRecordBuilder::of(assembly, chromosome, pos, ref_allele, alt_allele);
        let molecule_context = MoleculeContext::Genomic;
        let allelic_state = AllelicState::heterozygous();
        
        let variation_descriptor = VariationDescriptorBuilder::builder(identifier)
            .gene_context(gene_desc.clone())
            .add_expression(hgvs_c_expr)
            .add_expression(hgvs_g_expr)
            .vcf_record(vcf.clone())
            .molecule_context(molecule_context)
            .heterozygous()
            .build();
        assert_eq!(gene_desc, variation_descriptor.gene_context.unwrap());
        assert_eq!(2, variation_descriptor.expressions.len());
        assert_eq!(vcf, variation_descriptor.vcf_record.unwrap());
        assert_eq!(molecule_context as i32, variation_descriptor.molecule_context);
        assert_eq!(allelic_state, variation_descriptor.allelic_state.unwrap());
       
    }


}