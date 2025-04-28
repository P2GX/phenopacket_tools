use phenopackets::schema::v2::core::Resource;

const DEFAULT_UCUM_VERSION: &str = "2.1";
const DEFAULT_CHEBI_VERSION: &str = "241";

pub struct Resources;

impl Resources {
    fn base_resource(id: &str, name: &str, prefix: &str, iri_prefix: &str, url: &str) -> Resource {
        Resource {
            id: id.to_string(),
            name: name.to_string(),
            namespace_prefix: prefix.to_string(),
            iri_prefix: iri_prefix.to_string(),
            url: url.to_string(),
            ..Default::default()
        }
    }

    pub fn hpo_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "hp",
            "human phenotype ontology",
            "HP",
            "http://purl.obolibrary.org/obo/HP_",
            "http://purl.obolibrary.org/obo/hp.owl",
        );
        res.version = version.to_string();
        res
    }

    pub fn geno_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "geno",
            "Genotype Ontology",
            "GENO",
            "http://purl.obolibrary.org/obo/GENO_",
            "http://purl.obolibrary.org/obo/geno.owl",
        );
        res.version = version.to_string();
        res
    }

    pub fn pato_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "pato",
            "PhenotypicFeature And Trait Ontology",
            "PATO",
            "http://purl.obolibrary.org/obo/PATO_",
            "http://purl.obolibrary.org/obo/pato.owl",
        );
        res.version = version.to_string();
        res
    }

    pub fn efo_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "efo",
            "Experimental Factor Ontology",
            "EFO",
            "http://purl.obolibrary.org/obo/EFO_",
            "http://www.ebi.ac.uk/efo/efo.owl",
        );
        res.version = version.to_string();
        res
    }

    pub fn eco_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "eco",
            "Evidence & Conclusion Ontology (ECO)",
            "ECO",
            "http://purl.obolibrary.org/obo/ECO_",
            "http://purl.obolibrary.org/obo/eco.owl",
        );
        res.version = version.to_string();
        res
    }

    pub fn cl_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "cl",
            "Cell Ontology",
            "CL",
            "http://purl.obolibrary.org/obo/CL_",
            "http://purl.obolibrary.org/obo/cl.owl",
        );
        res.version = version.to_string();
        res
    }

    pub fn ncit_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "ncit",
            "NCI Thesaurus",
            "NCIT",
            "http://purl.obolibrary.org/obo/NCIT_",
            "http://purl.obolibrary.org/obo/ncit.owl",
        );
        res.version = version.to_string();
        res
    }

    pub fn mondo_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "mondo",
            "Mondo Disease Ontology",
            "MONDO",
            "http://purl.obolibrary.org/obo/MONDO_",
            "http://purl.obolibrary.org/obo/mondo.obo",
        );
        res.version = version.to_string();
        res
    }

    pub fn uberon_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "uberon",
            "Uber-anatomy ontology",
            "UBERON",
            "http://purl.obolibrary.org/obo/UBERON_",
            "http://purl.obolibrary.org/obo/uberon.owl",
        );
        res.version = version.to_string();
        res
    }

    pub fn ncbi_taxon_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "ncbitaxon",
            "NCBI organismal classification",
            "NCBITaxon",
            "http://purl.obolibrary.org/obo/NCBITaxon_",
            "http://purl.obolibrary.org/obo/ncbitaxon.owl",
        );
        res.version = version.to_string();
        res
    }

    pub fn so_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "so",
            "Sequence types and features ontology",
            "SO",
            "http://purl.obolibrary.org/obo/SO_",
            "http://purl.obolibrary.org/obo/so.owl",
        );
        res.version = version.to_string();
        res
    }

    pub fn uo_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "uo",
            "Units of measurement ontology",
            "UO",
            "http://purl.obolibrary.org/obo/UO_",
            "http://purl.obolibrary.org/obo/uo.owl",
        );
        res.version = version.to_string();
        res
    }

    pub fn ucum_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "ucum",
            "Unified Code for Units of Measure",
            "UCUM",
            "https://units-of-measurement.org/",
            "https://ucum.org",
        );
        res.version = version.to_string();
        res
    }

    pub fn ucum() -> Resource {
        Self::ucum_version(DEFAULT_UCUM_VERSION)
    }

    pub fn loinc_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "loinc",
            "Logical Observation Identifiers Names and Codes",
            "LOINC",
            "https://loinc.org/",
            "https://loinc.org",
        );
        res.version = version.to_string();
        res
    }

    pub fn drug_central_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "drugcentral",
            "Drug Central",
            "DrugCentral",
            "https://drugcentral.org/drugcard/",
            "https://drugcentral.org/",
        );
        res.version = version.to_string();
        res
    }

    pub fn omim_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "omim",
            "An Online Catalog of Human Genes and Genetic Disorders",
            "OMIM",
            "https://www.omim.org/entry/",
            "https://www.omim.org",
        );
        res.version = version.to_string();
        res
    }

    pub fn chebi_version(version: &str) -> Resource {
        let mut res = Self::base_resource(
            "chebi",
            "Chemical Entities of Biological Interest",
            "CHEBI",
            "https://purl.obolibrary.org/obo/CHEBI_",
            "https://www.ebi.ac.uk/chebi",
        );
        res.version = version.to_string();
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use phenopackets::schema::v2::core::time_element;
    use rstest::rstest;

    #[rstest]
    #[case("chebi", "Chemical Entities of Biological Interest", "CHEBI", "https://purl.obolibrary.org/obo/CHEBI_", "https://www.ebi.ac.uk/chebi", DEFAULT_CHEBI_VERSION)]
    #[case("hp","human phenotype ontology","HP", "http://purl.obolibrary.org/obo/HP_","http://purl.obolibrary.org/obo/hp.owl","v2025-03-03")]
    #[case("ucum", "Unified Code for Units of Measure", "UCUM", "https://units-of-measurement.org/", "https://ucum.org", DEFAULT_UCUM_VERSION)]
    fn test_ctor(
        #[case] id: &str, 
        #[case] name: &str, 
        #[case] namespace_prefix: &str, 
        #[case] iri_prefix: &str, 
        #[case] url: &str, 
        #[case] version: &str, 
    ) {
        let resource = match id {
             "chebi" =>  Resources::chebi_version(version),
             "hp" =>  Resources::hpo_version(version),
             "ucum" => Resources::ucum(),
             _ => panic!("Did not recognize resource id {}", id)
        };
       
        assert_eq!(id, resource.id);
        assert_eq!(name, resource.name);
        assert_eq!(namespace_prefix, resource.namespace_prefix);
        assert_eq!(iri_prefix, resource.iri_prefix);
        assert_eq!(version, resource.version);
   
    }

}