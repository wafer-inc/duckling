// Port of exe/Duckling/Ranking/Generate.hs
// Trains Naive Bayes classifiers from corpus examples and writes JSON files.

use duckling::corpus::time_en;
use duckling::{train_classifiers, DimensionKind, Lang, Locale};
use std::collections::BTreeMap;
use std::path::Path;

fn main() {
    let locales = vec![(Lang::EN, "en_xx")];

    for (lang, filename) in locales {
        let locale = Locale::new(lang, None);
        let dims = [DimensionKind::Time];
        let corpus = time_en::corpus();

        println!("Training classifiers for {}...", filename);
        let classifiers = train_classifiers(&locale, &corpus, &dims);
        println!(
            "  Trained {} classifiers from {} examples",
            classifiers.len(),
            corpus.examples.len()
        );

        // Sort by rule name for deterministic output
        let sorted: BTreeMap<_, _> = classifiers.into_iter().collect();
        let json = serde_json::to_string_pretty(&sorted).expect("Failed to serialize classifiers");

        let out_path = Path::new("src/ranking_classifiers").join(format!("{}.json", filename));
        std::fs::write(&out_path, &json).expect("Failed to write classifier file");
        println!("  Wrote {}", out_path.display());
    }

    println!("Done!");
}
