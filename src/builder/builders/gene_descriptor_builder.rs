use phenopackets::ga4gh::vrsatile::v1::GeneDescriptor;

pub struct GeneDescriptorBuilder {
    builder: GeneDescriptor,
}

impl GeneDescriptorBuilder {
    pub fn of(value_id: impl Into<String>, symbol: impl Into<String>) -> GeneDescriptor {
        GeneDescriptor {
            value_id: value_id.into(),
            symbol: symbol.into(),
            ..Default::default()
        }
    }

    pub fn builder(value_id: impl Into<String>, symbol: impl Into<String>) -> Self {
        Self {
            builder: GeneDescriptor {
                value_id: value_id.into(),
                symbol: symbol.into(),
                ..Default::default()
            },
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.builder.description = desc.into();
        self
    }

    pub fn add_alternate_id(mut self, alt_id: impl Into<String>) -> Self {
        self.builder.alternate_ids.push(alt_id.into());
        self
    }

    pub fn add_all_alternate_ids(mut self, alt_ids: impl IntoIterator<Item = String>) -> Self {
        self.builder.alternate_ids.extend(alt_ids);
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

    pub fn add_alternate_symbol(mut self, alt_symbol: impl Into<String>) -> Self {
        self.builder.alternate_symbols.push(alt_symbol.into());
        self
    }

    pub fn add_all_alternate_symbols(mut self, alt_symbols: impl IntoIterator<Item = String>) -> Self {
        self.builder.alternate_symbols.extend(alt_symbols);
        self
    }

    pub fn build(self) -> GeneDescriptor {
        self.builder
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    fn test_simple_gene_descriptor() {
        let value_id = "HGNC:3477";
        let symbol = "ETF1";
        let g_d = GeneDescriptorBuilder::of(value_id, symbol);
        assert_eq!(value_id, g_d.value_id);
        assert_eq!(symbol, g_d.symbol);
        assert_eq!(0, g_d.alternate_ids.len());
        assert_eq!(String::default(), g_d.description);
    }

    #[rstest]
    fn test_complex_gene_descriptor() {
        let value_id = "HGNC:3477";
        let symbol = "ETF1";
        let alternate_ids = vec![
                "ensembl:ENSG00000120705".to_string(), "ncbigene:2107".to_string(), 
                "ucsc:uc003ldc.6".to_string(), "OMIM:600285".to_string()
        ];
        let alternate_symbols = vec![
                "SUP45L1".to_string(), "ERF1".to_string(), "ERF".to_string(), 
                "eRF1".to_string(), "TB3-1".to_string(), "RF1".to_string()
        ];
        let xrefs = vec![ "VGNC:97422".to_string(),  "MGI:2385071".to_string(),  
            "RGD:1305712".to_string(), "ensembl:ENSRNOG00000019450".to_string(), 
            "ncbigene:307503".to_string() ];
        let g_d = GeneDescriptorBuilder::builder(value_id, symbol)
                .add_all_alternate_ids(alternate_ids)
                .add_all_alternate_symbols(alternate_symbols)
                .add_all_xrefs(xrefs)
                .build();
        assert_eq!(value_id, g_d.value_id);
        assert_eq!(symbol, g_d.symbol);
        assert_eq!(4, g_d.alternate_ids.len());
        assert_eq!(6, g_d.alternate_symbols.len());
        assert_eq!(5, g_d.xrefs.len());

    }

}