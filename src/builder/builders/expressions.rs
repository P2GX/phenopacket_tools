use phenopackets::ga4gh::vrsatile::v1::Expression;

pub struct Expressions;

impl Expressions {
    /// An HGVS cDNA expression (e.g., "NM_004006.1:c.3>T")
    pub fn hgvs_cdna(hgvs_expression: impl Into<String>) -> Expression {
        Expression {
            syntax: "hgvs.c".to_string(),
            value: hgvs_expression.into(),
            version: String::default(),
        }
    }

    /// An HGVS genomic expression (e.g., "NC_000001.11:g.27549219del")
    pub fn hgvs_genomic(hgvs_expression: impl Into<String>) -> Expression {
        Expression {
            syntax: "hgvs.g".to_string(),
            value: hgvs_expression.into(),
            version: String::default(),
        }
    }

    /// A transcript reference (e.g., "NM_000321.2")
    pub fn transcript_reference(value: impl Into<String>) -> Expression {
        Expression {
            syntax: "transcript_reference".to_string(),
            value: value.into(),
            version: String::default(),
        }
    }
}
