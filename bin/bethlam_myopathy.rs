

use clap::builder::Str;
use phenopackets::schema::v2::{core::{
        Diagnosis, Evidence, ExternalReference, GenomicInterpretation, Individual, Interpretation, MetaData, OntologyClass, PhenotypicFeature, Sex, TimeElement
    }, Phenopacket};
use phenopackets::schema::v2::core::time_element::Element;
use std::{collections::HashMap, env::var};

use phenopacket_tools::{builders::{builder::Builder, expressions::Expressions, resources::{self, Resources}, time_elements}, constants::{self, allelic_state::AllelicState}};
use phenopacket_tools::error::{self, Error, Result};


pub fn bethlem_myopathy_phenopacket() -> Result<Phenopacket> {
    let bethlem_myopathy = Builder::ontology_class("OMIM:158810", "Bethlem myopathy 1")?;
    let external_reference = Builder::external_reference("PMID:30808312","COL6A1 mutation leading to Bethlem myopathy with recurrent hematuria: a case report");
    let evidence = Evidence {
        evidence_code: Some(OntologyClass {
            id: "ECO:0000033".to_string(),
            label: "author statement supported by traceable reference".to_string(),
        }),
        reference: Some(external_reference.clone()),
        ..Default::default()
    };

    let mut individual = Builder::individual("proband A");
    individual.set_sex(Sex::Male);
    individual.time_at_last_encounter = Some(time_elements::age("P6Y3M")?);
    let geno = resources::Resources::geno_version("2020-03-08");
    let eco = Resources::eco_version("2022-08-05");
    let omim = Resources::omim_version("2022-11-23");
    let hpo_r = Resources::hpo_version("2021-08-02");
    

    let created_ts = time_elements::timestamp_from_str("2021-05-14T10:35:00Z")?;
    let mut metadata = Builder::meta_data(created_ts, "anonymous biocurator");
    metadata.resources.push(hpo_r);
    metadata.resources.push(geno);
    metadata.resources.push(eco);
    metadata.resources.push(omim);
    metadata.external_references.push(external_reference);
   

    let gene_descriptor = Builder::gene_descriptor("HGNC:2211", "COL6A1");
    let vcf_record = Builder::vcf_record("GRCh38", "chr21", 45989626, "G", "A");
    let hgvs_c = Expressions::hgvs_cdna("NM_001848.2:c.877G>A");
    let mut var_descriptor = Builder::variation_descriptor("variant id");
    var_descriptor.expressions.push(hgvs_c);
    var_descriptor.vcf_record = Some(vcf_record);
    var_descriptor.gene_context = Some(gene_descriptor);
    var_descriptor.allelic_state = Some(AllelicState::heterozygous());
      

    let variant_interpretation = Builder::variant_interpretation_acmg(phenopackets::schema::v2::core::AcmgPathogenicityClassification::Pathogenic, var_descriptor);
    let genomic_i = Builder::genomic_interpretation_from_variant("id",
            phenopackets::schema::v2::core::genomic_interpretation::InterpretationStatus::Causative,
            variant_interpretation);
   
    let diagnosis = Diagnosis{
        disease: Some( bethlem_myopathy),
        genomic_interpretations: vec![genomic_i],
    };
    

    let interpretation = Builder::solved_interpretation("arbitrary interpretation id", diagnosis);
 

    // Create PhenotypicFeatures
    let phenotypic_features = vec![
        ("HP:0001629", "Ventricular septal defect", Some("congenital")),
        ("HP:0000280", "Coarse facial features", None),
        ("HP:0008689", "Bilateral cryptorchidism", Some("congenital")),
        ("HP:0001561", "Polyhydramnios", Some("fetal")),
        ("HP:0000054", "Micropenis", Some("congenital")),
        ("HP:0001798", "Anonychia", Some("congenital")),
        ("HP:0001320", "Cerebellar vermis hypoplasia", None),
        ("HP:0000518", "Cataract", Some("infantile")),
        ("HP:0002198", "Dilated fourth ventricle", None),
        ("HP:0100333", "Unilateral cleft lip", Some("congenital")),
    ]
    .into_iter()
    .map(|(id, label, onset)| {
        let onset_time = match onset {
            Some("congenital") => Some(TimeElement {
                element: Some(Element::OntologyClass(constants::onset::CONGENITAL_ONSET.clone()))
            }),
            Some("fetal") => Some(TimeElement {
                element: Some(Element::OntologyClass(constants::onset::FETAL_ONSET.clone()))
            }),
            Some("infantile") => Some(TimeElement {
                element: Some(Element::OntologyClass(constants::onset::INFANTILE_ONSET.clone()))
            }),
            _ => None,
        };
        PhenotypicFeature {
            r#type: Some(OntologyClass {
                id: id.to_string(),
                label: label.to_string(),
            }),
            onset: onset_time,
            evidence: vec![evidence.clone()],
            ..Default::default()
        }
    })
    .collect();

    // Create Phenopacket
    let phenopacket = Phenopacket {
        id: "arbitrary proband id".to_string(),
        subject: Some(individual),
        phenotypic_features,
        interpretations: vec![interpretation],
        meta_data: Some(metadata),
        ..Default::default()
    };
    Ok(phenopacket)

    // phenopacket can now be serialized or used as needed
}
