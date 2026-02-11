// Ported from Duckling/Numeral/EN/Corpus.hs
use duckling::{parse_en, DimensionKind};

fn check_numeral(text: &str, expected: f64) {
    let entities = parse_en(text, &[DimensionKind::Numeral]);
    let found = entities.iter().any(|e| {
        e.dim == "number"
            && e.value
                .value
                .get("value")
                .and_then(|v| v.as_f64())
                .map(|v| (v - expected).abs() < 0.01)
                .unwrap_or(false)
    });
    assert!(
        found,
        "Expected numeral {} for '{}', got: {:?}",
        expected,
        text,
        entities
            .iter()
            .map(|e| format!("{}={:?}", e.dim, e.value))
            .collect::<Vec<_>>()
    );
}

// NumeralValue 0
#[test]
fn test_numeral_zero() {
    check_numeral("0", 0.0);
    check_numeral("naught", 0.0);
    check_numeral("nought", 0.0);
    check_numeral("zero", 0.0);
    check_numeral("nil", 0.0);
}

// NumeralValue 1
#[test]
fn test_numeral_one() {
    check_numeral("1", 1.0);
    check_numeral("one", 1.0);
    check_numeral("single", 1.0);
}

// NumeralValue 2
#[test]
fn test_numeral_two() {
    check_numeral("2", 2.0);
    check_numeral("two", 2.0);
    check_numeral("a pair", 2.0);
    check_numeral("a couple", 2.0);
    check_numeral("a couple of", 2.0);
}

// NumeralValue 3
#[test]
fn test_numeral_three() {
    check_numeral("3", 3.0);
    check_numeral("three", 3.0);
    check_numeral("a few", 3.0);
    check_numeral("few", 3.0);
}

// NumeralValue 10
#[test]
fn test_numeral_ten() {
    check_numeral("10", 10.0);
    check_numeral("ten", 10.0);
}

// NumeralValue 12
#[test]
fn test_numeral_twelve() {
    check_numeral("12", 12.0);
    check_numeral("twelve", 12.0);
    check_numeral("a dozen", 12.0);
    check_numeral("a dozen of", 12.0);
}

// NumeralValue 14
#[test]
fn test_numeral_fourteen() {
    check_numeral("14", 14.0);
    check_numeral("fourteen", 14.0);
}

// NumeralValue 16
#[test]
fn test_numeral_sixteen() {
    check_numeral("16", 16.0);
    check_numeral("sixteen", 16.0);
}

// NumeralValue 17
#[test]
fn test_numeral_seventeen() {
    check_numeral("17", 17.0);
    check_numeral("seventeen", 17.0);
}

// NumeralValue 18
#[test]
fn test_numeral_eighteen() {
    check_numeral("18", 18.0);
    check_numeral("eighteen", 18.0);
}

// NumeralValue 33
#[test]
fn test_numeral_thirty_three() {
    check_numeral("33", 33.0);
    check_numeral("thirty three", 33.0);
    check_numeral("0033", 33.0);
}

// NumeralValue 24
#[test]
fn test_numeral_twenty_four() {
    check_numeral("24", 24.0);
    check_numeral("2 dozens", 24.0);
    check_numeral("two dozen", 24.0);
    check_numeral("Two dozen", 24.0);
}

// NumeralValue 1.1
#[test]
fn test_numeral_one_point_one() {
    check_numeral("1.1", 1.1);
    check_numeral("1.10", 1.1);
    check_numeral("01.10", 1.1);
    check_numeral("1 point 1", 1.1);
}

// NumeralValue 0.77
#[test]
fn test_numeral_point_seven_seven() {
    check_numeral(".77", 0.77);
    check_numeral("0.77", 0.77);
    check_numeral("point 77", 0.77);
}

// NumeralValue 100000
#[test]
fn test_numeral_hundred_thousand() {
    check_numeral("100,000", 100000.0);
    check_numeral("100,000.0", 100000.0);
    check_numeral("100000", 100000.0);
    check_numeral("100K", 100000.0);
    check_numeral("100k", 100000.0);
    check_numeral("one hundred thousand", 100000.0);
}

// NumeralValue 0.2
#[test]
fn test_numeral_fractions() {
    check_numeral("1/5", 0.2);
    check_numeral("2/10", 0.2);
    check_numeral("3/15", 0.2);
    check_numeral("20/100", 0.2);
}

// NumeralValue 3000000
#[test]
fn test_numeral_three_million() {
    check_numeral("3M", 3000000.0);
    check_numeral("3000K", 3000000.0);
    check_numeral("3000000", 3000000.0);
    check_numeral("3,000,000", 3000000.0);
    check_numeral("3 million", 3000000.0);
    check_numeral("30 lakh", 3000000.0);
    check_numeral("30 lkh", 3000000.0);
    check_numeral("30 l", 3000000.0);
}

// NumeralValue 1200000
#[test]
fn test_numeral_one_point_two_million() {
    check_numeral("1,200,000", 1200000.0);
    check_numeral("1200000", 1200000.0);
    check_numeral("1.2M", 1200000.0);
    check_numeral("1200k", 1200000.0);
    check_numeral(".0012G", 1200000.0);
    check_numeral("12 lakhs", 1200000.0);
    check_numeral("12 lkhs", 1200000.0);
}

// NumeralValue 5000
#[test]
fn test_numeral_five_thousand() {
    check_numeral("5 thousand", 5000.0);
    check_numeral("five thousand", 5000.0);
}

// NumeralValue -504
#[test]
fn test_numeral_negative_504() {
    check_numeral("-504", -504.0);
    check_numeral("negative five hundred and four", -504.0);
}

// NumeralValue -1200000
#[test]
fn test_numeral_negative_1_2_million() {
    check_numeral("- 1,200,000", -1200000.0);
    check_numeral("-1200000", -1200000.0);
    check_numeral("minus 1,200,000", -1200000.0);
    check_numeral("negative 1200000", -1200000.0);
    check_numeral("-1.2M", -1200000.0);
    check_numeral("-1200K", -1200000.0);
    check_numeral("-.0012G", -1200000.0);
}

// NumeralValue -3200000
#[test]
fn test_numeral_negative_3_2_million() {
    check_numeral("-3,200,000", -3200000.0);
    check_numeral("-3200000", -3200000.0);
    check_numeral("minus three million two hundred thousand", -3200000.0);
}

// NumeralValue 122
#[test]
fn test_numeral_one_twenty_two() {
    check_numeral("one twenty two", 122.0);
    check_numeral("ONE TwentY tWO", 122.0);
}

// NumeralValue 200000
#[test]
fn test_numeral_two_hundred_thousand() {
    check_numeral("two Hundred thousand", 200000.0);
}

// NumeralValue 21011
#[test]
fn test_numeral_twenty_one_thousand_eleven() {
    check_numeral("twenty-one thousand Eleven", 21011.0);
}

// NumeralValue 721012
#[test]
fn test_numeral_seven_hundred_twenty_one_thousand_twelve() {
    check_numeral("seven hundred twenty-one thousand twelve", 721012.0);
    check_numeral("seven hundred twenty-one thousand and twelve", 721012.0);
}

// NumeralValue 31256721
#[test]
fn test_numeral_thirty_one_million() {
    check_numeral(
        "thirty-one million two hundred fifty-six thousand seven hundred twenty-one",
        31256721.0,
    );
    check_numeral(
        "three crore twelve lakh fifty-six thousand seven hundred twenty-one",
        31256721.0,
    );
    check_numeral(
        "three cr twelve lac fifty-six thousand seven hundred twenty-one",
        31256721.0,
    );
}

// NumeralValue 2400
#[test]
fn test_numeral_two_hundred_dozens() {
    check_numeral("two hundred dozens", 2400.0);
    check_numeral("200 dozens", 2400.0);
}

// NumeralValue 2200000
#[test]
fn test_numeral_two_point_two_million() {
    check_numeral("two point two million", 2200000.0);
}

// NumeralValue 3000000000
#[test]
fn test_numeral_three_billion() {
    check_numeral("three billions", 3000000000.0);
    check_numeral("three thousand millions", 3000000000.0);
    check_numeral("three hundred crores", 3000000000.0);
    check_numeral("three hundred Cr", 3000000000.0);
    check_numeral("three hundred koti", 3000000000.0);
    check_numeral("three hundred krores", 3000000000.0);
    check_numeral("three hundred Kr", 3000000000.0);
}

// NumeralValue 45
#[test]
fn test_numeral_forty_five() {
    check_numeral("forty-five (45)", 45.0);
    check_numeral("45 (forty five)", 45.0);
}
