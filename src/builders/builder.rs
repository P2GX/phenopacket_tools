use std::process::id;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use phenopackets::ga4gh::vrsatile::v1::{Extension, GeneDescriptor, MoleculeContext, VariationDescriptor, VcfRecord};



use phenopackets::schema::v2::core::genomic_interpretation::{Call, InterpretationStatus};
use phenopackets::schema::v2::core::interpretation::ProgressStatus;
use phenopackets::schema::v2::core::{AcmgPathogenicityClassification, Diagnosis, Disease, ExternalReference, GenomicInterpretation, Individual, Interpretation, MetaData, OntologyClass, PhenotypicFeature, TherapeuticActionability, VariantInterpretation, VitalStatus};
use phenopackets::schema::v2::Phenopacket;
use prost_types::Timestamp;

use crate::error::{self, Error, Result};

/// A valid curie must have a non-empty prefix and a non-empty numeric suffic
/// white-space is not allowed.
pub fn check_valid_curie(s: &str) -> Result<()> {
    if s.is_empty() {
        return Err(Error::CurieError {
            msg: "Empty CURIE".to_string(),
        });
    } else if let Some(pos) = s.find(':') {
        if s.chars().any(|c| c.is_whitespace()) {
            return Err(Error::CurieError {
                msg: format!("Contains stray whitespace: '{}'", s),
            });
        } else if s.matches(':').count() != 1 {
            return Err(Error::CurieError {
                msg: format!("Invalid CURIE with more than one colon: '{}", s),
            });
        } else if pos == 0 {
            return Err(Error::CurieError {
                msg: format!("Invalid CURIE with no prefix: '{}'", s),
            });
        } else if pos == s.len() - 1 {
            return Err(Error::CurieError {
                msg: format!("Invalid CURIE with no suffix: '{}'", s),
            });
        } else if let Some((_prefix, suffix)) = s.split_once(':') {
            if !suffix.chars().all(char::is_alphanumeric) {
                return Err(Error::CurieError {
                    msg: format!("Invalid CURIE with non-alphanumeric characters in suffix: '{}'", s),
                });
            }
        }
    } else {
        return Err(Error::CurieError {
            msg: format!("Invalid CURIE with no colon: '{}'", s),
        });
    }
    Ok(())
}








pub struct Builder {
}

impl Builder {
    
    pub fn vcf_record(
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

    pub fn ontology_class(id: impl Into<String>, label: impl Into<String>) -> Result<OntologyClass> {
        let id:String = id.into();
        check_valid_curie(&id)?;
        Ok(OntologyClass {
            id: id,
            label: label.into(),
        })
    }

    pub fn gene_descriptor(value_id: impl Into<String>, symbol: impl Into<String>) -> GeneDescriptor {
        GeneDescriptor { 
            value_id: value_id.into(), 
            symbol: symbol.into(), 
            description: String::default(), 
            alternate_ids: vec![], 
            alternate_symbols: vec![], 
            xrefs: vec![]
        }
    }



    pub fn phenotypic_feature_observed(feature: OntologyClass) -> PhenotypicFeature {
        PhenotypicFeature {
            r#type: Some(feature),
            ..Default::default()
        }
    }

    pub fn phenotypic_feature_excluded(feature: OntologyClass) -> PhenotypicFeature {
        PhenotypicFeature {
            r#type: Some(feature),
            excluded: true,
            ..Default::default()
        }
    }

    /// Quickly create an ExternalReference with id and description
    pub fn external_reference(id: impl Into<String>, description: impl Into<String>) -> ExternalReference {
        ExternalReference {
            id: id.into(),
            description: description.into(),
            ..Default::default()
        }
    }

     /// Create a mosaicism Extension with the given percentage (e.g., 12.5%).
    pub fn mosaicism_extension(percentage: f64) -> Extension {
        let percentage_string = format!("{:.1}%", percentage);
        let MOSAICISM: &str = "mosaicism";
        Extension {
            name: MOSAICISM.to_string(),
            value: percentage_string,
            ..Default::default()
        }
    }

    /// Create an allele frequency Extension with the given frequency (in percent).
    pub fn allele_frequency_extension(frequency: f64) -> Extension {
        let percentage_string = format!("{:.1}%", frequency);
        let ALLELE_FREQUENCY: &str = "allele-frequency";
        Extension {
            name: ALLELE_FREQUENCY.to_string(),
            value: percentage_string,
            ..Default::default()
        }
    }

    pub fn disease(term: OntologyClass) -> Disease {
        Disease {
            term: Some(term),
            excluded: false,
            onset: None, 
            resolution: None,
            disease_stage: vec![],
            clinical_tnm_finding: vec![],
            primary_site: None,
            laterality: None,
        }
    }

    pub fn individual(id: impl Into<String>) -> Individual {
        Individual{
            id: id.into(),
            alternate_ids: vec![],
            date_of_birth: None,
            time_at_last_encounter: None,
            vital_status: None,
            sex: phenopackets::schema::v2::core::Sex::UnknownSex.into(),
            karyotypic_sex: phenopackets::schema::v2::core::KaryotypicSex::UnknownKaryotype.into(),
            gender: None,
            taxonomy: None
        } 
    }

    /// Create builder from ISO8601 string
    pub fn meta_data(created: Timestamp, created_by: impl Into<String>) -> MetaData {
        let schema_version: &'static str = "2.0.2"; // latest Phenopacket Schema version
        let mut metadata = MetaData::default();
        metadata.created = Some(created);
        metadata.created_by = created_by.into();
        metadata.phenopacket_schema_version = schema_version.to_string();
        metadata 
    }

    /// Create a MetaData message with the current time
    pub fn meta_data_now(created_by: impl Into<String>) -> MetaData {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = Timestamp {
            seconds: now.as_secs() as i64,
            nanos: now.subsec_nanos() as i32,
        };
        Self::meta_data(timestamp, created_by)
    }

    fn interpretation(identifier: impl Into<String>, diagnosis: Diagnosis, status: ProgressStatus) -> Interpretation {
        Interpretation { 
            id: identifier.into(), 
            progress_status: status.into(), 
            diagnosis: Some(diagnosis), 
            summary: String::default() 
        }
    }

    pub fn solved_interpretation(identifier: impl Into<String>, diagnosis: Diagnosis) -> Interpretation {
        Self::interpretation(identifier, diagnosis, ProgressStatus::Solved)
    }

    pub fn unsolved_interpretation(identifier: impl Into<String>, diagnosis: Diagnosis) -> Interpretation {
        Self::interpretation(identifier, diagnosis, ProgressStatus::Unsolved)
    }

    pub fn completed_interpretation(identifier: impl Into<String>, diagnosis: Diagnosis) -> Interpretation {
        Self::interpretation(identifier, diagnosis, ProgressStatus::Completed)
    }

    pub fn genomic_interpretation_from_gene_descriptor(
        identifier: impl Into<String>, 
        interpretation_status: InterpretationStatus,
        gdesc: GeneDescriptor) -> GenomicInterpretation {
        GenomicInterpretation { 
                subject_or_biosample_id: identifier.into(), 
                interpretation_status: interpretation_status.into(), 
                call: Some(Call::Gene(gdesc)) 
            }
    }

    pub fn genomic_interpretation_from_variant( 
        identifier: impl Into<String>, 
        interpretation_status: InterpretationStatus,
        variant_interpretation: VariantInterpretation) -> GenomicInterpretation {
        GenomicInterpretation { 
                subject_or_biosample_id: identifier.into(), 
                interpretation_status: interpretation_status.into(), 
                call: Some(Call::VariantInterpretation(variant_interpretation))
            }
    }

    
    pub fn variation_descriptor(identifier: impl Into<String>) -> VariationDescriptor {
        VariationDescriptor {
            id: identifier.into(),
            variation: None,
            label: String::default(),
            description: String::default(),
            gene_context: None,
            expressions: vec![],
            vcf_record: None,
            xrefs: vec![],
            alternate_labels: vec![],
            extensions: vec![],
            molecule_context: MoleculeContext::UnspecifiedMoleculeContext.into(),
            structural_type: None,
            vrs_ref_allele_seq: String::default(),
            allelic_state: None,
        }
    }

    pub fn variant_interpretation_acmg(
        acmg: AcmgPathogenicityClassification, 
        variant_descriptor: VariationDescriptor) -> VariantInterpretation {
        VariantInterpretation { 
            acmg_pathogenicity_classification: acmg.into(), 
            therapeutic_actionability: TherapeuticActionability::UnknownActionability.into(), 
            variation_descriptor: Some(variant_descriptor) 
        }
    }

    pub fn variant_interpretation_benign(variant_descriptor: VariationDescriptor) -> VariantInterpretation {
        Self::variant_interpretation_acmg(AcmgPathogenicityClassification::Benign, variant_descriptor)
    }

    pub fn variant_interpretation_likely_benign(variant_descriptor: VariationDescriptor) -> VariantInterpretation {
        Self::variant_interpretation_acmg(AcmgPathogenicityClassification::LikelyBenign, variant_descriptor)
    }

    pub fn variant_interpretation_vus(variant_descriptor: VariationDescriptor) -> VariantInterpretation {
        Self::variant_interpretation_acmg(AcmgPathogenicityClassification::UncertainSignificance, variant_descriptor)
    }

    pub fn variant_interpretation_likely_pathogenic(variant_descriptor: VariationDescriptor) -> VariantInterpretation {
        Self::variant_interpretation_acmg(AcmgPathogenicityClassification::LikelyPathogenic, variant_descriptor)
    }

    pub fn variant_interpretation_pathogenic(variant_descriptor: VariationDescriptor) -> VariantInterpretation {
        Self::variant_interpretation_acmg(AcmgPathogenicityClassification::Pathogenic, variant_descriptor)
    }

    pub fn phenopacket(identifier: impl Into<String>, meta_data: MetaData) -> Phenopacket {
        Phenopacket { 
            id: identifier.into(), 
            subject: None, 
            phenotypic_features: vec![], 
            measurements: vec![], 
            biosamples: vec![], 
            interpretations: vec![], 
            diseases: vec![],
            medical_actions: vec![], 
            files: vec![], 
            meta_data: Some(meta_data)
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::{builders::{resources::Resources, time_elements::{self, time_element_from_str}}, constants::disease_stage::DiseaseStage};
    use super::*;
    use phenopackets::ga4gh::{vrs::v1::SimpleInterval, vrsatile::v1::VcfRecord};
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
        let vcf = Builder::vcf_record(assembly, chr, position, ref_allele, alt_allele);
        assert_eq!(assembly, vcf.genome_assembly);
    }

    #[rstest]
    #[case("HP:0002162", "Low posterior hairline")]
    #[case("MONDO:0017309", "neonatal Marfan syndrome")]
    #[case("NCIT:C2926", "Lung Non-Small Cell Carcinoma")]
    fn test_valid_terms(
        #[case] term_id: &str, 
        #[case] term_label: &str, 
    ) {
        let term = Builder::ontology_class(term_id, term_label);
        assert!(term.is_ok());
        let term = term.unwrap();
        assert_eq!(term_id, term.id);
        assert_eq!(term_label, term.label);
    }



    #[rstest]
    #[case("HP0002162", "Low posterior hairline", "Invalid CURIE with no colon: 'HP0002162'")]
    #[case("MONDO:0017309 ", "neonatal Marfan syndrome", "Contains stray whitespace: 'MONDO:0017309 '")]
    fn test_invalid_terms(
        #[case] term_id: &str, 
        #[case] term_label: &str, 
        #[case] error_msg: &str, 
    ) {
        let term = Builder::ontology_class(term_id, term_label);
        assert!(term.is_err());
        assert!(matches!(&term, Err(Error::CurieError { .. })));
        assert_eq!(error_msg, term.unwrap_err().to_string());
    }


    #[rstest]
    fn test_simple_gene_descriptor() {
        let value_id = "HGNC:3477";
        let symbol = "ETF1";
        let g_d = Builder::gene_descriptor(value_id, symbol);
        assert_eq!(value_id, g_d.value_id);
        assert_eq!(symbol, g_d.symbol);
        assert_eq!(0, g_d.alternate_ids.len());
        assert_eq!(String::default(), g_d.description);
    }


    #[test]
    fn test_two_element_external_reference()  {
        let id = "PMID:30962759";
        let description = "Recurrent Erythema Nodosum in a Child with a SHOC2 Gene Mutation";
        let ext_ref = Builder::external_reference(id, description);
        assert_eq!(id, ext_ref.id);
        assert_eq!(description, ext_ref.description);
        assert_eq!("", ext_ref.reference);
    }

    fn approx_equal(a: f64, b: f64) -> bool
    {
        (a-b).abs() < std::f64::EPSILON
    }

    #[rstest]
    fn test_mosaicism() {
        let percentage = 42 as f64;
        let extension = Builder::mosaicism_extension(percentage);
        assert_eq!("mosaicism", extension.name);
        assert_eq!("42.0%", extension.value);
    }


    #[rstest]
    fn test_allele_frequency() {
        let percentage = 23 as f64;
        let extension = Builder::allele_frequency_extension(percentage);
        assert_eq!("allele-frequency", extension.name);
        assert_eq!("23.0%", extension.value);
    }

    #[rstest]
    fn cardiomyopathy() {
        let disease_id = "MONDO:0004994";
        let disease_label = "cardiomyopathy";
        let disease = Builder::ontology_class(disease_id, disease_label).unwrap();
        let disease = Builder::disease(disease);
        assert_eq!(disease_id, disease.term.unwrap().id);
    }

    #[rstest]
    fn cardiomyopathy_with_stage() {
        let disease_id = "MONDO:0004994";
        let disease_label = "cardiomyopathy";
        let disease_term = Builder::ontology_class(disease_id, disease_label).unwrap();
        let stage = "New York Heart Association Class IV";
        let mut disease = Builder::disease(disease_term);
        disease.disease_stage.push(DiseaseStage::nyha_class_iv());
        let dis_term = disease.term.unwrap();
        assert_eq!(disease_id, dis_term.id);
        assert_eq!(disease_label, dis_term.label);
        assert_eq!(stage, disease.disease_stage[0].label);
    }

    #[rstest]
    fn sca_with_onset() {
        let disease_id = "OMIM:164400";
        let disease_label = "Spinocerebellar ataxia 1";
        let onset = time_elements::age("P38Y7M").unwrap();
        let disease_term = Builder::ontology_class(disease_id, disease_label).unwrap();
        let mut disease = Builder::disease(disease_term);
        disease.onset = Some(onset.clone());
        let dis_term = disease.term.unwrap();
        assert_eq!(disease_id, dis_term.id);
        assert_eq!(disease_label, dis_term.label);
        assert!(disease.onset.is_some());
        let onset_age = disease.onset.unwrap();
        assert_eq!(onset_age, onset);
    }


    #[rstest]
    fn test_create_metadata() -> Result<()> {
        let created= "2019-07-21T00:25:54.662Z";
        let created_by = "Earnest B. Biocurator";
        let phenopacket_schema_version = "2.0.2";
        let hpo = Resources::hpo_version("v2025-03-03");
        let geno = Resources::geno_version("2023-10-08");
        let ext_res = Builder::external_reference("PMID:30808312","Bao M, et al. COL6A1 mutation leading to Bethlem myopathy with recurrent hematuria: a case report. BMC Neurol. 2019;19(1):32.");
        let mut mdata = Builder::meta_data_now(created_by);
        mdata.external_references.push(ext_res);
        mdata.resources.push(hpo);
        mdata.resources.push(geno);
        assert_eq!(created_by, mdata.created_by);
        assert_eq!(phenopacket_schema_version, mdata.phenopacket_schema_version);
        assert_eq!(2, mdata.resources.len());
        Ok(())
    }


}