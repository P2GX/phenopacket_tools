use phenopacket_tools::{builder::builders::{diagnosis_builder::DiagnosisBuilder, expressions::Expressions, external_reference_builder::ExternalReferenceBuilder, gene_descriptor_builder::GeneDescriptorBuilder, genomic_interpretation_builder::GenomicInterpretationBuilder, individual_builder::IndividualBuilder, interpretation_builder::InterpretationBuilder, meta_data_builder::MetaDataBuilder, ontology_class_builder, resources, time_elements, variant_interpretation_builder::VariantInterpretationBuilder, variation_descriptor_builder::VariationDescriptorBuilder, vcf_record_builder::VcfRecordBuilder}, constants};

use phenopackets::schema::v2::{core::{
        Diagnosis, Evidence, ExternalReference, GenomicInterpretation, Individual, Interpretation, MetaData, OntologyClass, PhenotypicFeature, Sex, TimeElement
    }, Phenopacket};
use phenopackets::schema::v2::core::time_element::Element;
use std::collections::HashMap;

use phenopacket_tools::builder::builders::resources::Resources;
use phenopacket_tools::error::{self, Error, Result};


pub fn bethlem_myopathy_phenopacket() -> Result<Phenopacket> {
    let bethlem_myopathy = ontology_class_builder::ontology_class("OMIM:158810", "Bethlem myopathy 1")?;


    let external_reference = ExternalReferenceBuilder::of("PMID:30808312",
    "COL6A1 mutation leading to Bethlem myopathy with recurrent hematuria: a case report");
        

    let evidence = Evidence {
        evidence_code: Some(OntologyClass {
            id: "ECO:0000033".to_string(),
            label: "author statement supported by traceable reference".to_string(),
        }),
        reference: Some(external_reference.clone()),
        ..Default::default()
    };

    let individual = IndividualBuilder::new("proband A")
        .male()
        .age_at_last_encounter(time_elements::age("P6Y3M")?)
        .build()?;
  

    let geno = resources::Resources::geno_version("2020-03-08");
    let eco = Resources::eco_version("2022-08-05");
    let omim = Resources::omim_version("2022-11-23");
    let hpo_r = Resources::hpo_version("2021-08-02");
    

    let created_ts = time_elements::timestamp_from_str("2021-05-14T10:35:00Z")?;
    let metadata = MetaDataBuilder::from_created(created_ts, "anonymous biocurator")
        .add_resource(hpo_r)
        .add_resource(geno)
        .add_resource(eco)
        .add_resource(omim)
        .add_external_reference(external_reference)
        .build();

    let gene_descriptor = GeneDescriptorBuilder::builder("HGNC:2211", "COL6A1").build();
        
    let vcf_record = VcfRecordBuilder::builder("GRCh38", "chr21", 45989626, "G", "A").build();
    let hgvs_c = Expressions::hgvs_cdna("NM_001848.2:c.877G>A");
    let var_descriptor = VariationDescriptorBuilder::builder("variant id")
        .add_expression(hgvs_c)
        .vcf_record(vcf_record)
        .gene_context(gene_descriptor)
        .heterozygous()
        .build();

    let variant_interpretation = VariantInterpretationBuilder::new(var_descriptor)
        .pathogenic()
        .build();
    let genomic_interpretation = GenomicInterpretationBuilder::builder("arbitrary interpretation id")
        .causative()
        .variant_interpretation(variant_interpretation)?
        .build()?;
    let diagnosis = DiagnosisBuilder::new(bethlem_myopathy)
        .add_genomic_interpretation(genomic_interpretation)
        .build();

    let interpretation = InterpretationBuilder::new("arbitrary interpretation id")
        .solved(diagnosis)
        .build();


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
