use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{QuantityData, QuantityUnit};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "km quantity cups".to_string(),
            pattern: vec![regex("(បីកែវ)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    3.0,
                    QuantityUnit::Cup,
                )))
            }),
        },
        Rule {
            name: "km quantity bowl".to_string(),
            pattern: vec![regex("(១ចាន)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    1.0,
                    QuantityUnit::Cup,
                )))
            }),
        },
        Rule {
            name: "km quantity pint".to_string(),
            pattern: vec![regex("(ដប់ប្រាំថូ)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    15.0,
                    QuantityUnit::Cup,
                )))
            }),
        },
        Rule {
            name: "km quantity persons".to_string(),
            pattern: vec![regex("(មនុស្ស២នាក់|មនុស្សពីរនាក់)")],
            production: Box::new(|_| {
                let mut q = QuantityData::new(2.0, QuantityUnit::Cup);
                q.product = Some("មនុស្ស".to_string());
                Some(TokenData::Quantity(q))
            }),
        },
        Rule {
            name: "km quantity buildings".to_string(),
            pattern: vec![regex("(ផ្ទះ៨ខ្នង|ផ្ទះប្រាំបីខ្នង)")],
            production: Box::new(|_| {
                let mut q = QuantityData::new(8.0, QuantityUnit::Cup);
                q.product = Some("ផ្ទះ".to_string());
                Some(TokenData::Quantity(q))
            }),
        },
        Rule {
            name: "km quantity 1000 gram".to_string(),
            pattern: vec![regex("(មួយពាន់ក្រាម|មួយគីឡូក្រាម|មួយលានមីលីក្រាម)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    1000.0,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "km quantity 2-7 gram".to_string(),
            pattern: vec![regex(
                "(ចាប់ពី 2 ដល់ 7 ក្រាម|ចន្លោះពី ២ ដល់ ៧ក្រាម|ចន្លោះ ២ក្រាម និង ៧ក្រាម|ប្រហែល ២-៧ ក្រាម|~2-7ក្រាម)",
            )],
            production: Box::new(|_| {
                Some(TokenData::Quantity(
                    QuantityData::unit_only(QuantityUnit::Gram).with_interval(2.0, 7.0),
                ))
            }),
        },
        Rule {
            name: "km quantity max 4 tbsp".to_string(),
            pattern: vec![regex(
                "(តិចជាងបួនស្លាបព្រា|មិនលើស៤ស្លាបព្រា|ក្រោម៤ស្លាបព្រា|យ៉ាងច្រើន៤ស្លាបព្រា)",
            )],
            production: Box::new(|_| {
                Some(TokenData::Quantity(
                    QuantityData::unit_only(QuantityUnit::Tablespoon).with_max(4.0),
                ))
            }),
        },
        Rule {
            name: "km quantity min 10 bowl".to_string(),
            pattern: vec![regex("(ច្រើនជាងដប់ចាន|មិនតិចជាងដប់ចាន|លើសពីដប់ចាន|យ៉ាងតិចដប់ចាន)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(
                    QuantityData::unit_only(QuantityUnit::Cup).with_min(10.0),
                ))
            }),
        },
    ]
}
