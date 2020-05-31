use std::error::Error;
use csv::Writer;
use core::iter;

#[cfg(test)]
mod tests {
    use crate::{create_all_syllables, write_to_csv};

    #[test]
    fn test_create_syllables() {
        let syllables = create_all_syllables();
        for syllable in &syllables {
            println!("{}", syllable)
        }
        write_to_csv(&syllables,"syllables.csv");
        assert_ne!(syllables.len(), 0)
    }
}

pub fn create_all_syllables() -> Vec<String> {
    //ng initial a "helper consonant" to display syllables that do not start with a consonant
    let init_consonants =
        vec!["p", "b", "t", "d", "k", "g", "m", "n", "s", "z",
             "ch", "sh", "f", "h", "l", "x", "kl", "shm", "fn", "ng"];

    let vowels =
        vec!["i", "eu", "u", "e", "o", "ae", "eo", "a", "ai", "ao", "oa", "oi",
             "yi", "yeu", "yu", "ye", "yo", "yae", "yeo", "ya", "yai", "yao", "yoa", "yoi",
             "wi", "weu", "wu", "we", "wo", "wae", "weo", "wa", "wai", "wao", "woa", "woi",
        ];
    let coda_consonants =
        vec!["ng", "m", "n", "s", "b", "g", "d",
             "bs", "l", "lb", "lg", "ld", "lm"];

    let mut all_syllables: Vec<String> = Vec::new();

    let mut i = 0;

    // ALL C V
    for &ic in &init_consonants {
        for &v in &vowels {
            i += 1;
            all_syllables.push(format!("{}{}", ic, v));
        }
    }


    // ALL V C
    for &v in &vowels {
        for &cc in &coda_consonants {
            i += 1;
            all_syllables.push(format!("{}{}", v, cc));
        }
    }

    for &ic in &init_consonants {
        for &v in &vowels {
            for &cc in &coda_consonants {
                i += 1;
                all_syllables.push(format!("{}{}{}", ic, v, cc));
            }
        }
    }
    all_syllables
}

pub fn write_to_csv(data: &Vec<String>, file: &str) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(file)?;
    for syllable in data {
        writer.write_record(iter::once(syllable))?;
    }
    writer.flush()?;
    Ok(())
}