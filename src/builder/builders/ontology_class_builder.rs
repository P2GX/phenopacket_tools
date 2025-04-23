use phenopackets::schema::v2::core::OntologyClass;

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
            if !suffix.chars().all(char::is_numeric) {
                return Err(Error::CurieError {
                    msg: format!("Invalid CURIE with non-digit characters in suffix: '{}'", s),
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



pub fn ontology_class(id: impl Into<String>, label: impl Into<String>) -> Result<OntologyClass> {
    let id = id.into();
    check_valid_curie(&id)?;
    Ok(OntologyClass {
        id: id.into(),
        label: label.into(),
    })
}


#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};


    #[rstest]
    #[case("HP:0002162", "Low posterior hairline")]
    #[case("MONDO:0017309", "neonatal Marfan syndrome")]
    fn test_valid_terms(
        #[case] term_id: &str, 
        #[case] term_label: &str, 
    ) {
        let term = ontology_class(term_id, term_label);
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
        let term = ontology_class(term_id, term_label);
        assert!(term.is_err());
        assert!(matches!(&term, Err(Error::CurieError { .. })));
        assert_eq!(error_msg, term.unwrap_err().to_string());
    }

}