pub mod en;

use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct QuantityData {
    pub value: f64,
    pub unit: String,
    pub product: Option<String>,
}

impl QuantityData {
    pub fn new(value: f64, unit: &str) -> Self {
        QuantityData {
            value,
            unit: unit.to_string(),
            product: None,
        }
    }

    pub fn with_product(mut self, product: &str) -> Self {
        self.product = Some(product.to_string());
        self
    }
}

pub fn resolve(data: &QuantityData) -> ResolvedValue {
    let mut json = serde_json::json!({
        "value": data.value,
        "type": "value",
        "unit": data.unit,
    });
    if let Some(ref product) = data.product {
        json.as_object_mut()
            .unwrap()
            .insert("product".to_string(), serde_json::json!(product));
    }
    ResolvedValue {
        kind: "value".to_string(),
        value: json,
    }
}
