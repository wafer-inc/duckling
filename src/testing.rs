use crate::engine;
use crate::resolve::{Context, Options};
use crate::types::{DimensionKind, Entity, Rule};

/// A corpus example: input text and expected outputs.
pub struct Example {
    pub text: String,
    pub check: Box<dyn Fn(&Entity) -> bool>,
}

/// A corpus is a collection of examples for testing.
pub struct Corpus {
    pub context: Context,
    pub examples: Vec<(Vec<String>, Box<dyn Fn(&Entity) -> bool>)>,
}

impl Corpus {
    pub fn new(context: Context) -> Self {
        Corpus {
            context,
            examples: Vec::new(),
        }
    }

    /// Add a set of example texts that should all produce the same result.
    pub fn add<F>(&mut self, texts: Vec<&str>, check: F)
    where
        F: Fn(&Entity) -> bool + 'static,
    {
        let texts: Vec<String> = texts.into_iter().map(String::from).collect();
        self.examples.push((texts, Box::new(check)));
    }
}

/// Check a corpus against a set of rules and return failures.
pub fn check_corpus(corpus: &Corpus, rules: &[Rule], dims: &[DimensionKind]) -> Vec<String> {
    let options = Options {
        with_latent: false,
    };
    let mut failures = Vec::new();

    for (texts, check) in &corpus.examples {
        for text in texts {
            let entities = engine::parse_and_resolve(text, rules, &corpus.context, &options, dims);
            let any_match = entities.iter().any(|e| check(e));
            if !any_match {
                let dim_str = dims
                    .iter()
                    .map(|d| format!("{:?}", d))
                    .collect::<Vec<_>>()
                    .join(", ");
                failures.push(format!(
                    "FAIL: \"{}\" - expected match for [{}], got {} entities: {:?}",
                    text,
                    dim_str,
                    entities.len(),
                    entities
                        .iter()
                        .map(|e| format!("{}({:?})", e.value.dim_kind(), e.value))
                        .collect::<Vec<_>>()
                ));
            }
        }
    }

    failures
}
