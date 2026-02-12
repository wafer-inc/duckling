use std::fmt;

use chrono::{DateTime, NaiveDateTime, Utc};
use crate::dimensions::amount_of_money::AmountOfMoneyData;
use crate::dimensions::credit_card_number::CreditCardNumberData;
use crate::dimensions::distance::DistanceData;
use crate::dimensions::duration::DurationData;
use crate::dimensions::email::EmailData;
use crate::dimensions::numeral::NumeralData;
use crate::dimensions::ordinal::OrdinalData;
use crate::dimensions::phone_number::PhoneNumberData;
use crate::dimensions::quantity::QuantityData;
use crate::dimensions::temperature::TemperatureData;
use crate::dimensions::time::TimeData;
use crate::dimensions::time_grain::Grain;
use crate::dimensions::url::UrlData;
use crate::dimensions::volume::VolumeData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DimensionKind {
    Numeral,
    Ordinal,
    Temperature,
    Distance,
    Volume,
    Quantity,
    AmountOfMoney,
    Email,
    PhoneNumber,
    Url,
    CreditCardNumber,
    TimeGrain,
    Duration,
    Time,
}

impl fmt::Display for DimensionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DimensionKind::Numeral => "number",
            DimensionKind::Ordinal => "ordinal",
            DimensionKind::Temperature => "temperature",
            DimensionKind::Distance => "distance",
            DimensionKind::Volume => "volume",
            DimensionKind::Quantity => "quantity",
            DimensionKind::AmountOfMoney => "amount-of-money",
            DimensionKind::Email => "email",
            DimensionKind::PhoneNumber => "phone-number",
            DimensionKind::Url => "url",
            DimensionKind::CreditCardNumber => "credit-card-number",
            DimensionKind::TimeGrain => "time-grain",
            DimensionKind::Duration => "duration",
            DimensionKind::Time => "time",
        };
        write!(f, "{}", s)
    }
}

/// A measurement with a numeric value and unit. Used by Temperature, Distance,
/// Volume, Quantity, and AmountOfMoney dimensions.
///
/// ```
/// use duckling::{parse_en, DimensionKind, DimensionValue, MeasurementValue};
///
/// let results = parse_en("$42.50", &[DimensionKind::AmountOfMoney]);
/// assert_eq!(results[0].value, DimensionValue::AmountOfMoney(MeasurementValue::Value {
///     value: 42.5, unit: "USD".into(),
/// }));
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum MeasurementValue {
    Value { value: f64, unit: String },
    Interval { from: Option<MeasurementPoint>, to: Option<MeasurementPoint> },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct MeasurementPoint {
    pub value: f64,
    pub unit: String,
}

/// A resolved time point — either an absolute UTC instant or a naive wall-clock time.
///
/// ```
/// use duckling::{parse, Locale, Lang, Context, Options, DimensionKind,
///                DimensionValue, TimeValue, TimePoint, Grain};
/// use chrono::{NaiveDate, TimeZone, Utc};
///
/// let locale = Locale::new(Lang::EN, None);
/// let context = Context {
///     reference_time: Utc.with_ymd_and_hms(2013, 2, 12, 4, 30, 0).unwrap(),
///     ..Context::default()
/// };
/// let options = Options::default();
///
/// // Wall-clock times are Naive (no timezone baked in)
/// let results = parse("tomorrow at 3pm", &locale, &[DimensionKind::Time], &context, &options);
/// assert_eq!(results[0].value, DimensionValue::Time(TimeValue::Single(TimePoint::Naive {
///     value: NaiveDate::from_ymd_opt(2013, 2, 13).unwrap().and_hms_opt(15, 0, 0).unwrap(),
///     grain: Grain::Hour,
/// })));
///
/// // Relative offsets from now are Instant (pinned to UTC)
/// let results = parse("in one hour", &locale, &[DimensionKind::Time], &context, &options);
/// assert_eq!(results[0].value, DimensionValue::Time(TimeValue::Single(TimePoint::Instant {
///     value: Utc.with_ymd_and_hms(2013, 2, 12, 5, 30, 0).unwrap(),
///     grain: Grain::Minute,
/// })));
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum TimePoint {
    Instant { value: DateTime<Utc>, grain: Grain },
    Naive { value: NaiveDateTime, grain: Grain },
}

impl TimePoint {
    pub fn grain(&self) -> Grain {
        match self {
            TimePoint::Instant { grain, .. } | TimePoint::Naive { grain, .. } => *grain,
        }
    }
}

/// A resolved time value — either a single point or an interval.
///
/// ```
/// use duckling::{parse, Locale, Lang, Context, Options, DimensionKind,
///                DimensionValue, TimeValue, TimePoint, Grain};
/// use chrono::{NaiveDate, TimeZone, Utc};
///
/// let locale = Locale::new(Lang::EN, None);
/// let context = Context {
///     reference_time: Utc.with_ymd_and_hms(2013, 2, 12, 4, 30, 0).unwrap(),
///     ..Context::default()
/// };
/// let options = Options::default();
///
/// // Single time point
/// let results = parse("tomorrow", &locale, &[DimensionKind::Time], &context, &options);
/// assert_eq!(results[0].value, DimensionValue::Time(TimeValue::Single(TimePoint::Naive {
///     value: NaiveDate::from_ymd_opt(2013, 2, 13).unwrap().and_hms_opt(0, 0, 0).unwrap(),
///     grain: Grain::Day,
/// })));
///
/// // Time interval
/// let results = parse("from 3pm to 5pm", &locale, &[DimensionKind::Time], &context, &options);
/// assert_eq!(results[0].value, DimensionValue::Time(TimeValue::Interval {
///     from: Some(TimePoint::Naive {
///         value: NaiveDate::from_ymd_opt(2013, 2, 12).unwrap().and_hms_opt(15, 0, 0).unwrap(),
///         grain: Grain::Hour,
///     }),
///     to: Some(TimePoint::Naive {
///         value: NaiveDate::from_ymd_opt(2013, 2, 12).unwrap().and_hms_opt(18, 0, 0).unwrap(),
///         grain: Grain::Hour,
///     }),
/// }));
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum TimeValue {
    Single(TimePoint),
    Interval { from: Option<TimePoint>, to: Option<TimePoint> },
}

/// The resolved value of a parsed entity.
///
/// ```
/// use duckling::{parse_en, DimensionKind, DimensionValue, Grain};
///
/// assert_eq!(parse_en("thirty three", &[DimensionKind::Numeral])[0].value,
///     DimensionValue::Numeral(33.0));
///
/// assert_eq!(parse_en("the 3rd", &[DimensionKind::Ordinal])[0].value,
///     DimensionValue::Ordinal(3));
///
/// assert_eq!(parse_en("3 days", &[DimensionKind::Duration])[0].value,
///     DimensionValue::Duration { value: 3, grain: Grain::Day, normalized_seconds: 259200 });
///
/// assert_eq!(parse_en("user@example.com", &[DimensionKind::Email])[0].value,
///     DimensionValue::Email("user@example.com".into()));
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum DimensionValue {
    Numeral(f64),
    Ordinal(i64),
    Temperature(MeasurementValue),
    Distance(MeasurementValue),
    Volume(MeasurementValue),
    Quantity { measurement: MeasurementValue, product: Option<String> },
    AmountOfMoney(MeasurementValue),
    Email(String),
    PhoneNumber(String),
    Url { value: String, domain: String },
    CreditCardNumber { value: String, issuer: String },
    TimeGrain(Grain),
    Duration { value: i64, grain: Grain, normalized_seconds: i64 },
    Time(TimeValue),
}

impl DimensionValue {
    pub fn dim_kind(&self) -> DimensionKind {
        match self {
            DimensionValue::Numeral(_) => DimensionKind::Numeral,
            DimensionValue::Ordinal(_) => DimensionKind::Ordinal,
            DimensionValue::Temperature(_) => DimensionKind::Temperature,
            DimensionValue::Distance(_) => DimensionKind::Distance,
            DimensionValue::Volume(_) => DimensionKind::Volume,
            DimensionValue::Quantity { .. } => DimensionKind::Quantity,
            DimensionValue::AmountOfMoney(_) => DimensionKind::AmountOfMoney,
            DimensionValue::Email(_) => DimensionKind::Email,
            DimensionValue::PhoneNumber(_) => DimensionKind::PhoneNumber,
            DimensionValue::Url { .. } => DimensionKind::Url,
            DimensionValue::CreditCardNumber { .. } => DimensionKind::CreditCardNumber,
            DimensionValue::TimeGrain(_) => DimensionKind::TimeGrain,
            DimensionValue::Duration { .. } => DimensionKind::Duration,
            DimensionValue::Time(_) => DimensionKind::Time,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenData {
    Numeral(NumeralData),
    Ordinal(OrdinalData),
    Temperature(TemperatureData),
    Distance(DistanceData),
    Volume(VolumeData),
    Quantity(QuantityData),
    AmountOfMoney(AmountOfMoneyData),
    Email(EmailData),
    PhoneNumber(PhoneNumberData),
    Url(UrlData),
    CreditCardNumber(CreditCardNumberData),
    TimeGrain(Grain),
    Duration(DurationData),
    Time(TimeData),
    RegexMatch(RegexMatchData),
}

impl TokenData {
    pub fn dimension_kind(&self) -> Option<DimensionKind> {
        match self {
            TokenData::Numeral(_) => Some(DimensionKind::Numeral),
            TokenData::Ordinal(_) => Some(DimensionKind::Ordinal),
            TokenData::Temperature(_) => Some(DimensionKind::Temperature),
            TokenData::Distance(_) => Some(DimensionKind::Distance),
            TokenData::Volume(_) => Some(DimensionKind::Volume),
            TokenData::Quantity(_) => Some(DimensionKind::Quantity),
            TokenData::AmountOfMoney(_) => Some(DimensionKind::AmountOfMoney),
            TokenData::Email(_) => Some(DimensionKind::Email),
            TokenData::PhoneNumber(_) => Some(DimensionKind::PhoneNumber),
            TokenData::Url(_) => Some(DimensionKind::Url),
            TokenData::CreditCardNumber(_) => Some(DimensionKind::CreditCardNumber),
            TokenData::TimeGrain(_) => Some(DimensionKind::TimeGrain),
            TokenData::Duration(_) => Some(DimensionKind::Duration),
            TokenData::Time(_) => Some(DimensionKind::Time),
            TokenData::RegexMatch(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegexMatchData {
    pub groups: Vec<Option<String>>,
}

impl RegexMatchData {
    pub fn group(&self, idx: usize) -> Option<&str> {
        self.groups.get(idx).and_then(|g| g.as_deref())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub range: Range,
    pub token_data: TokenData,
    pub children: Vec<Node>,
    pub rule_name: Option<String>,
}

impl Node {
    pub fn new(range: Range, token_data: TokenData) -> Self {
        Node {
            range,
            token_data,
            children: Vec::new(),
            rule_name: None,
        }
    }
}

pub type Predicate = Box<dyn Fn(&TokenData) -> bool + Send + Sync>;
pub type Production = Box<dyn Fn(&[&Node]) -> Option<TokenData> + Send + Sync>;

pub enum PatternItem {
    Regex(regex::Regex),
    Dimension(DimensionKind),
    Predicate(Predicate),
}

impl fmt::Debug for PatternItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternItem::Regex(r) => write!(f, "Regex({})", r.as_str()),
            PatternItem::Dimension(d) => write!(f, "Dimension({:?})", d),
            PatternItem::Predicate(_) => write!(f, "Predicate(...)"),
        }
    }
}

pub struct Rule {
    pub name: String,
    pub pattern: Vec<PatternItem>,
    pub production: Production,
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rule")
            .field("name", &self.name)
            .field("pattern", &self.pattern)
            .finish()
    }
}

/// A parsed entity extracted from text, with its position, matched text, and resolved value.
///
/// ```
/// use duckling::{parse_en, Entity, DimensionKind, DimensionValue};
///
/// assert_eq!(parse_en("I need 42 widgets", &[DimensionKind::Numeral]), vec![Entity {
///     body: "42".into(), start: 7, end: 9, latent: None,
///     value: DimensionValue::Numeral(42.0),
/// }]);
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Entity {
    pub body: String,
    pub start: usize,
    pub end: usize,
    pub value: DimensionValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latent: Option<bool>,
}
