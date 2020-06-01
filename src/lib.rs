use core::iter;
use csv::Writer;
use std::error::Error;

pub mod settings;
pub mod syllable;

#[cfg(test)]
mod tests {
    #[test]
    fn test_syllable_builder() {
        use crate::settings::*;
        use crate::syllable::{SyllableBuilder, SyllableHandler};

        let vowels: Vec<String> = vec!["i", "ɨ", "u", "e", "o", "ɛ", "ʌ", "a"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        let diphthongs: Vec<String> = vec!["ai", "ao", "oa", "oi"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        let consonants: Vec<String> = vec![
            "p", "b", "t", "d", "k", "g", "m", "n", "ŋ", "ks", "tʃ", "ɸ", "s", "z", "ʃ", "h", "ɾ",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        let consonant_cluster: Vec<String> =
            vec!["ɸn", "sɾ", "ʃm", "ʃɾ", "ɾb", "ɾm", "ɾt", "ɾg", "kɾ", "bs"]
                .iter()
                .map(|x| x.to_string())
                .collect();

        let no_init_consonants: Vec<String> = vec!["bs", "ɾb", "ɾm", "ɾt", "ɾg"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        let no_coda_consonants: Vec<String> = vec![
            "p", "t", "k", "kɾ", "h", "ks", "z", "ʃ", "tʃ", "ʃɾ", "ʃm", "ɸn", "ɸ", "ɸn",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();

        let syllables = SyllableBuilder::new()
            .with_syllable_structure_settings(SyllableStructure::CVC)
            .with_diphthong_settings(Diphthongs::OnlyGiven)
            .with_glide_settings(Glides::All)
            .with_consonant_cluster_settings(
                ConsonantCluster::OnlyGiven,
                ConsonantClusterLength::None,
            )
            .with_vowel_data(vowels)
            .with_diphthong_data(diphthongs)
            .with_disabled_init_cluster(no_init_consonants)
            .with_disabled_coda_cluster(no_coda_consonants)
            .with_consonant_data(consonants)
            .with_consonant_cluster_data(consonant_cluster)
            .create_glides()
            .create_nucleus()
            .create_init_cluster()
            .create_coda_cluster()
            .build();

        syllables.write(
            WriteOption::CSV("syllables.csv".to_owned()),
            SyllableFormat::IPA,
        );
    }
}
