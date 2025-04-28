use phenopackets::ga4gh::vrsatile::v1::Extension;

/// Constants for extension names
pub const MOSAICISM: &str = "mosaicism";
pub const ALLELE_FREQUENCY: &str = "allele-frequency";

pub struct Extensions;

impl Extensions {
    /// Create a mosaicism Extension with the given percentage (e.g., 12.5%).
    pub fn mosaicism(percentage: f64) -> Extension {
        let percentage_string = format!("{:.1}%", percentage);
        Extension {
            name: MOSAICISM.to_string(),
            value: percentage_string,
            ..Default::default()
        }
    }

    /// Create an allele frequency Extension with the given frequency (in percent).
    pub fn allele_frequency(frequency: f64) -> Extension {
        let percentage_string = format!("{:.1}%", frequency);
        Extension {
            name: ALLELE_FREQUENCY.to_string(),
            value: percentage_string,
            ..Default::default()
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::builder::builders::{ontology_class_builder, time_elements::time_element_from_str};

    use super::*;
    use phenopackets::ga4gh::{vrs::v1::abundance, vrsatile::v1::Extension};
    use rstest::rstest;
    use serde_json::de;

    fn approx_equal(a: f64, b: f64) -> bool
    {
        (a-b).abs() < std::f64::EPSILON
    }

    #[rstest]
    fn test_mosaicism() {
        let percentage = 42 as f64;
        let extension = Extensions::mosaicism(percentage);
        assert_eq!(MOSAICISM, extension.name);
        assert_eq!("42.0%", extension.value);
    }


    #[rstest]
    fn test_allele_frequency() {
        let percentage = 23 as f64;
        let extension = Extensions::allele_frequency(percentage);
        assert_eq!(ALLELE_FREQUENCY, extension.name);
        assert_eq!("23.0%", extension.value);
    }

}