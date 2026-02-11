// Ported from Duckling/Quantity/EN/Corpus.hs
use duckling::{parse_en, DimensionKind};

fn check_quantity(text: &str, expected_val: f64, expected_unit: &str) {
    check_quantity_impl(text, expected_val, expected_unit, None);
}

fn check_quantity_with_product(text: &str, expected_val: f64, expected_unit: &str, expected_product: &str) {
    check_quantity_impl(text, expected_val, expected_unit, Some(expected_product));
}

fn check_quantity_impl(text: &str, expected_val: f64, expected_unit: &str, expected_product: Option<&str>) {
    let entities = parse_en(text, &[DimensionKind::Quantity]);
    let found = entities.iter().any(|e| {
        if e.dim != "quantity" {
            return false;
        }
        let v = &e.value.value;

        // Helper to check a single value object (top-level, from, or to)
        let check_obj = |obj: &serde_json::Value| -> bool {
            let val_ok = obj
                .get("value")
                .and_then(|v| v.as_f64())
                .map(|val| (val - expected_val).abs() < 0.01)
                .unwrap_or(false);
            let unit_ok = obj.get("unit").and_then(|u| u.as_str()) == Some(expected_unit);
            let product_ok = match expected_product {
                Some(p) => obj.get("product").and_then(|v| v.as_str()) == Some(p),
                None => true,
            };
            val_ok && unit_ok && product_ok
        };

        // Check simple value (top-level has value + unit)
        if check_obj(v) {
            return true;
        }

        // Check interval from value
        if let Some(from) = v.get("from") {
            if check_obj(from) {
                return true;
            }
        }

        // Check interval to value
        if let Some(to) = v.get("to") {
            if check_obj(to) {
                return true;
            }
        }

        false
    });
    assert!(
        found,
        "Expected quantity {} {} {} for '{}', got: {:?}",
        expected_val,
        expected_unit,
        expected_product.unwrap_or(""),
        text,
        entities
            .iter()
            .map(|e| format!("{}={:?}", e.dim, e.value))
            .collect::<Vec<_>>()
    );
}

// simple Pound 2 (Just "meat")
#[test]
fn test_quantity_2_pounds_meat() {
    check_quantity_with_product("two pounds of meat", 2.0, "pound", "meat");
}

// simple Gram 2 Nothing
#[test]
fn test_quantity_2_grams() {
    check_quantity("2 grams", 2.0, "gram");
    check_quantity("0.002 kg", 2.0, "gram");
    check_quantity("2 g.", 2.0, "gram");
    check_quantity("2 gs", 2.0, "gram");
    check_quantity("2/1000 kilograms", 2.0, "gram");
    check_quantity("2000 milligrams", 2.0, "gram");
    check_quantity("2,000 milligrams", 2.0, "gram");
}

// simple Gram 1000 Nothing
#[test]
fn test_quantity_1_kilogram() {
    check_quantity("a kilogram", 1000.0, "gram");
    check_quantity("a kg", 1000.0, "gram");
    check_quantity("1 k.g.", 1000.0, "gram");
    check_quantity("1 k.gs", 1000.0, "gram");
    check_quantity("1000 gs", 1000.0, "gram");
}

// simple Pound 1 Nothing
#[test]
fn test_quantity_1_pound() {
    check_quantity("a Pound", 1.0, "pound");
    check_quantity("1 lb", 1.0, "pound");
    check_quantity("a lb", 1.0, "pound");
}

// simple Ounce 2 Nothing
#[test]
fn test_quantity_2_ounces() {
    check_quantity("2 ounces", 2.0, "ounce");
    check_quantity("2oz", 2.0, "ounce");
}

// simple Cup 3 (Just "sugar")
#[test]
fn test_quantity_3_cups_sugar() {
    check_quantity_with_product("3 Cups of sugar", 3.0, "cup", "sugar");
    check_quantity_with_product("3 Cups of SugAr", 3.0, "cup", "sugar");
}

// simple Cup 0.75 Nothing
#[test]
fn test_quantity_three_quarter_cup() {
    check_quantity("3/4 cup", 0.75, "cup");
    check_quantity("0.75 cup", 0.75, "cup");
    check_quantity(".75 cups", 0.75, "cup");
}

// simple Gram 500 (Just "strawberries")
#[test]
fn test_quantity_500g_strawberries() {
    check_quantity_with_product("500 grams of strawberries", 500.0, "gram", "strawberries");
    check_quantity_with_product("500g of strawberries", 500.0, "gram", "strawberries");
    check_quantity_with_product("0.5 kilograms of strawberries", 500.0, "gram", "strawberries");
    check_quantity_with_product("0.5 kg of strawberries", 500.0, "gram", "strawberries");
    check_quantity_with_product("500000mg of strawberries", 500.0, "gram", "strawberries");
}

// between Gram (100,1000) (Just "strawberries")
#[test]
fn test_quantity_100_1000g_strawberries() {
    check_quantity_with_product("100-1000 gram of strawberries", 100.0, "gram", "strawberries");
    check_quantity_with_product("between 100 and 1000 grams of strawberries", 100.0, "gram", "strawberries");
    check_quantity_with_product("from 100 to 1000 g of strawberries", 100.0, "gram", "strawberries");
    check_quantity_with_product("100 - 1000 g of strawberries", 100.0, "gram", "strawberries");
}

// between Gram (2,7) Nothing
#[test]
fn test_quantity_2_7g() {
    check_quantity("around 2 -7 g", 2.0, "gram");
    check_quantity("~2-7 grams", 2.0, "gram");
    check_quantity("from 2 to 7 g", 2.0, "gram");
    check_quantity("between 2.0 g and about 7.0 g", 2.0, "gram");
    check_quantity("between 0.002 kg and about 0.007 kg", 2.0, "gram");
    check_quantity("2 - ~7 grams", 2.0, "gram");
}

// under Pound 6 (Just "meat")
#[test]
fn test_quantity_under_6_pounds_meat() {
    check_quantity_with_product("less than six pounds of meat", 6.0, "pound", "meat");
    check_quantity_with_product("no more than 6 lbs of meat", 6.0, "pound", "meat");
    check_quantity_with_product("below 6.0 pounds of meat", 6.0, "pound", "meat");
    check_quantity_with_product("at most six pounds of meat", 6.0, "pound", "meat");
}

// above Cup 2 Nothing
#[test]
fn test_quantity_above_2_cups() {
    check_quantity("exceeding 2 Cups", 2.0, "cup");
    check_quantity("at least two Cups", 2.0, "cup");
    check_quantity("over 2 Cups", 2.0, "cup");
    check_quantity("more than 2 Cups", 2.0, "cup");
}

// above Ounce 4 (Just "chocolate")
#[test]
fn test_quantity_above_4oz_chocolate() {
    check_quantity_with_product("exceeding 4 oz of chocolate", 4.0, "ounce", "chocolate");
    check_quantity_with_product("at least 4.0 oz of chocolate", 4.0, "ounce", "chocolate");
    check_quantity_with_product("over four ounces of chocolate", 4.0, "ounce", "chocolate");
    check_quantity_with_product("more than four ounces of chocolate", 4.0, "ounce", "chocolate");
}
