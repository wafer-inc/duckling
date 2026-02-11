pub mod en;

use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct QuantityData {
    pub value: Option<f64>,
    pub unit: Option<QuantityUnit>,
    pub product: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantityUnit {
    Cup,
    Gram,
    Ounce,
    Pound,
}

impl QuantityUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            QuantityUnit::Cup => "cup",
            QuantityUnit::Gram => "gram",
            QuantityUnit::Ounce => "ounce",
            QuantityUnit::Pound => "pound",
        }
    }
}

impl QuantityData {
    pub fn new(value: f64, unit: QuantityUnit) -> Self {
        QuantityData {
            value: Some(value),
            unit: Some(unit),
            product: None,
            min_value: None,
            max_value: None,
        }
    }

    pub fn unit_only(unit: QuantityUnit) -> Self {
        QuantityData {
            value: None,
            unit: Some(unit),
            product: None,
            min_value: None,
            max_value: None,
        }
    }

    pub fn with_product(mut self, product: &str) -> Self {
        self.product = Some(product.to_string());
        self
    }

    pub fn with_interval(mut self, from: f64, to: f64) -> Self {
        self.value = None;
        self.min_value = Some(from);
        self.max_value = Some(to);
        self
    }

    pub fn with_min(mut self, v: f64) -> Self {
        self.value = None;
        self.min_value = Some(v);
        self.max_value = None;
        self
    }

    pub fn with_max(mut self, v: f64) -> Self {
        self.value = None;
        self.min_value = None;
        self.max_value = Some(v);
        self
    }
}

fn single_json(unit_str: &str, value: f64, product: &Option<String>) -> serde_json::Value {
    let mut obj = serde_json::json!({"value": value, "unit": unit_str});
    if let Some(ref p) = product {
        obj.as_object_mut()
            .unwrap()
            .insert("product".to_string(), serde_json::json!(p));
    }
    obj
}

pub fn resolve(data: &QuantityData) -> Option<ResolvedValue> {
    let unit = data.unit.as_ref()?;
    let unit_str = unit.as_str();

    match (data.value, data.min_value, data.max_value) {
        (Some(v), _, _) => {
            let mut json = serde_json::json!({
                "value": v,
                "type": "value",
                "unit": unit_str,
            });
            if let Some(ref p) = data.product {
                json.as_object_mut()
                    .unwrap()
                    .insert("product".to_string(), serde_json::json!(p));
            }
            Some(ResolvedValue {
                kind: "value".to_string(),
                value: json,
            })
        }
        (None, Some(from), Some(to)) => Some(ResolvedValue {
            kind: "value".to_string(),
            value: serde_json::json!({
                "type": "interval",
                "from": single_json(unit_str, from, &data.product),
                "to": single_json(unit_str, to, &data.product),
            }),
        }),
        (None, Some(from), None) => Some(ResolvedValue {
            kind: "value".to_string(),
            value: serde_json::json!({
                "type": "interval",
                "from": single_json(unit_str, from, &data.product),
            }),
        }),
        (None, None, Some(to)) => Some(ResolvedValue {
            kind: "value".to_string(),
            value: serde_json::json!({
                "type": "interval",
                "to": single_json(unit_str, to, &data.product),
            }),
        }),
        _ => None,
    }
}
