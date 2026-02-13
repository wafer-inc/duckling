use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{DistanceData, DistanceUnit};

fn distance_data(td: &TokenData) -> Option<&DistanceData> { if let TokenData::Distance(d)=td{Some(d)}else{None} }

pub fn rules()->Vec<Rule>{vec![
    Rule{name:"<latent dist> km".to_string(),pattern:vec![dim(DimensionKind::Distance),regex("k(ilo)?m?(eter)?")],production:Box::new(|n|{let d=distance_data(&n[0].token_data)?;Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Kilometre)))})},
    Rule{name:"<dist> meter".to_string(),pattern:vec![dim(DimensionKind::Distance),regex("m(eter)?")],production:Box::new(|n|{let d=distance_data(&n[0].token_data)?;Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Metre)))})},
    Rule{name:"<dist> centimeters".to_string(),pattern:vec![dim(DimensionKind::Distance),regex("cm|centimeter")],production:Box::new(|n|{let d=distance_data(&n[0].token_data)?;Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Centimetre)))})},
    Rule{name:"<dist> mils".to_string(),pattern:vec![dim(DimensionKind::Distance),regex("mils?")],production:Box::new(|n|{let d=distance_data(&n[0].token_data)?;let x=d.value?;Some(TokenData::Distance(DistanceData::new(x*10.0, DistanceUnit::Kilometre)))})},
]}
