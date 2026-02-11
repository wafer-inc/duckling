// Ported from Duckling/Volume/EN/Corpus.hs
use duckling::{parse_en, DimensionKind};

fn check_volume(text: &str, expected_val: f64, expected_unit: &str) {
    let entities = parse_en(text, &[DimensionKind::Volume]);
    let found = entities.iter().any(|e| {
        if e.dim != "volume" {
            return false;
        }
        let v = &e.value.value;

        // Check simple value (type=value)
        if v.get("value")
            .and_then(|v| v.as_f64())
            .map(|val| (val - expected_val).abs() < 0.01)
            .unwrap_or(false)
            && v.get("unit").and_then(|u| u.as_str()) == Some(expected_unit)
        {
            return true;
        }

        // Check interval from value
        if let Some(from) = v.get("from") {
            if from
                .get("value")
                .and_then(|v| v.as_f64())
                .map(|val| (val - expected_val).abs() < 0.01)
                .unwrap_or(false)
                && from.get("unit").and_then(|u| u.as_str()) == Some(expected_unit)
            {
                return true;
            }
        }

        // Check interval to value
        if let Some(to) = v.get("to") {
            if to
                .get("value")
                .and_then(|v| v.as_f64())
                .map(|val| (val - expected_val).abs() < 0.01)
                .unwrap_or(false)
                && to.get("unit").and_then(|u| u.as_str()) == Some(expected_unit)
            {
                return true;
            }
        }

        false
    });
    assert!(
        found,
        "Expected volume {} {} for '{}', got: {:?}",
        expected_val,
        expected_unit,
        text,
        entities
            .iter()
            .map(|e| format!("{}={:?}", e.dim, e.value))
            .collect::<Vec<_>>()
    );
}

// simple Litre 1
#[test]
fn test_volume_1_litre() {
    check_volume("1 liter", 1.0, "litre");
    check_volume("1 litre", 1.0, "litre");
    check_volume("one liter", 1.0, "litre");
    check_volume("a liter", 1.0, "litre");
}

// simple Litre 2
#[test]
fn test_volume_2_litres() {
    check_volume("2 liters", 2.0, "litre");
    check_volume("2l", 2.0, "litre");
}

// simple Litre 1000
#[test]
fn test_volume_1000_litres() {
    check_volume("1000 liters", 1000.0, "litre");
    check_volume("thousand liters", 1000.0, "litre");
}

// simple Litre 0.5
#[test]
fn test_volume_half_litre() {
    check_volume("half liter", 0.5, "litre");
    check_volume("half-litre", 0.5, "litre");
    check_volume("half a liter", 0.5, "litre");
}

// simple Litre 0.25
#[test]
fn test_volume_quarter_litre() {
    check_volume("quarter-litre", 0.25, "litre");
    check_volume("fourth of liter", 0.25, "litre");
}

// simple Millilitre 1
#[test]
fn test_volume_1_ml() {
    check_volume("one milliliter", 1.0, "millilitre");
    check_volume("an ml", 1.0, "millilitre");
    check_volume("a millilitre", 1.0, "millilitre");
}

// simple Millilitre 250
#[test]
fn test_volume_250_ml() {
    check_volume("250 milliliters", 250.0, "millilitre");
    check_volume("250 millilitres", 250.0, "millilitre");
    check_volume("250ml", 250.0, "millilitre");
    check_volume("250mls", 250.0, "millilitre");
    check_volume("250 ml", 250.0, "millilitre");
}

// simple Gallon 3
#[test]
fn test_volume_3_gallons() {
    check_volume("3 gallons", 3.0, "gallon");
    check_volume("3 gal", 3.0, "gallon");
    check_volume("3gal", 3.0, "gallon");
    check_volume("around three gallons", 3.0, "gallon");
}

// simple Gallon 0.5
#[test]
fn test_volume_half_gallon() {
    check_volume("0.5 gals", 0.5, "gallon");
    check_volume("1/2 gallon", 0.5, "gallon");
    check_volume("half a gallon", 0.5, "gallon");
}

// simple Gallon 0.1
#[test]
fn test_volume_tenth_gallon() {
    check_volume("0.1 gallons", 0.1, "gallon");
    check_volume("tenth of a gallon", 0.1, "gallon");
}

// simple Hectolitre 3
#[test]
fn test_volume_3_hectolitres() {
    check_volume("3 hectoliters", 3.0, "hectolitre");
}

// between Litre (100, 1000)
#[test]
fn test_volume_between_100_1000_litres() {
    check_volume("between 100 and 1000 liters", 100.0, "litre");
    check_volume("100-1000 liters", 100.0, "litre");
    check_volume("from 100 to 1000 l", 100.0, "litre");
    check_volume("100 - 1000 l", 100.0, "litre");
}

// between Litre (2, 7)
#[test]
fn test_volume_between_2_7_litres() {
    check_volume("around 2 -7 l", 2.0, "litre");
    check_volume("~2-7 liters", 2.0, "litre");
    check_volume("from 2 to 7 l", 2.0, "litre");
    check_volume("between 2.0 l and about 7.0 l", 2.0, "litre");
    check_volume("between 2l and about 7l", 2.0, "litre");
    check_volume("2 - ~7 litres", 2.0, "litre");
}

// under Gallon 6
#[test]
fn test_volume_under_6_gallons() {
    check_volume("less than six gallons", 6.0, "gallon");
    check_volume("under six gallon", 6.0, "gallon");
    check_volume("no more than 6 gals", 6.0, "gallon");
    check_volume("below 6.0gal", 6.0, "gallon");
    check_volume("at most six gallons", 6.0, "gallon");
}

// above Hectolitre 2
#[test]
fn test_volume_above_2_hectolitres() {
    check_volume("exceeding 2 hectoliters", 2.0, "hectolitre");
    check_volume("at least two hectolitres", 2.0, "hectolitre");
    check_volume("over 2 hectolitre", 2.0, "hectolitre");
    check_volume("more than 2 hectoliter", 2.0, "hectolitre");
}

// above Millilitre 4
#[test]
fn test_volume_above_4_ml() {
    check_volume("exceeding 4 ml", 4.0, "millilitre");
    check_volume("at least 4.0 ml", 4.0, "millilitre");
    check_volume("over four milliliters", 4.0, "millilitre");
    check_volume("more than four mls", 4.0, "millilitre");
}
