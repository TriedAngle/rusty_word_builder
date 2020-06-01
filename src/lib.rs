use std::error::Error;
use csv::Writer;
use core::iter;

#[cfg(test)]
mod tests {
    use crate::{create_all_syllables, write_to_csv, SyllableBuilder, SyllableHandler, SyllableStructure, ClusterSettings, ClusterLength, WriteOption, SyllableFormat};

    #[test]
    fn test_create_syllables() {
        let syllables = create_all_syllables();
        for syllable in &syllables {
            println!("{}", syllable)
        }
        assert_ne!(syllables.len(), 0)
    }

    #[test]
    fn test_write_to_file() {
        let syllables = create_all_syllables();
        for syllable in &syllables {
            println!("{}", syllable)
        }
        let result = write_to_csv(&syllables,"syllables.csv");
    }

    #[test]
    fn test_syllable_builder() {
        let vowels =
            vec!["i", "ɨ", "u", "e", "o", "ɛ", "ʌ", "a"]
                .iter().map(|x| x.to_string()).collect();

        let diphthongs = vec!["ai", "ao", "oa", "oi"]
            .iter().map(|x| x.to_string()).collect();

        let consonants =
            vec!["p", "b", "t", "d", "k", "g", "m", "n", "ŋ",
                 "ks", "tʃ", "ɸ", "s", "z", "ʃ", "h", "ɾ"]
                .iter().map(|x| x.to_string()).collect();

        let consonant_cluster =
            vec!["ɸn", "sɾ", "ʃm", "ʃɾ", "ɾb", "ɾm", "ɾt", "ɾg", "kɾ", "bs"]
                .iter().map(|x| x.to_string()).collect();

        let no_init_consonants =
            vec!["bs", "ɾb", "ɾm", "ɾt", "ɾg"]
                .iter().map(|x| x.to_string()).collect();

        let no_coda_consonants =
            vec!["p", "t", "k", "kɾ", "h", "ks", "z", "ʃ", "tʃ", "ʃɾ", "ʃm", "ɸn", "ɸ", "ɸn"]
                .iter().map(|x| x.to_string()).collect();


        let syllables =
            SyllableBuilder::new(SyllableStructure::CVC)
                .set_vowels(vowels)
                .set_diphthongs_custom(diphthongs)
                .set_y_u_diphthongs()
                .set_consonants(consonants)
                .set_settings_cluster(ClusterSettings::OnlyGiven)
                .set_consonant_cluster_custom(consonant_cluster)
                .no_init_consonants(no_init_consonants)
                .no_coda_consonants(no_coda_consonants)
                .build();

        syllables.write(WriteOption::CSV("syllables.csv".to_owned()), SyllableFormat::IPA);
    }
}

pub enum SyllableStructure {
    CV,
    VC,
    CVC,
}

pub enum ClusterSettings {
    All,
    NoDouble,
    OnlyGiven,
    DisableGiven,
}

pub enum ClusterLength {
    C,
    CC,
    CCC,
    CCCC,
}

pub enum WriteOption {
    CSV(String),
    PSQL,
}

pub enum SyllableFormat {
    IPA,
    Romanization,
}

pub struct SyllableHandler {
    syllables: Vec<String>,
}

impl SyllableHandler {
    pub fn new(syllables: Vec<String>) -> Self {
        Self {
            syllables,
        }
    }

    pub fn all(&self) -> Vec<String> {
        self.syllables.clone()
    }

    pub fn write(&self, write_option: WriteOption, syllable_format: SyllableFormat) -> Result<(), Box<dyn Error>> {
        match syllable_format {
            SyllableFormat::IPA => { println!("TODO") },
            SyllableFormat::Romanization => { println!("TODO") }
        }

        match write_option {
            WriteOption::CSV(path) => {
                let mut writer = Writer::from_path(path)?;
                for syllable in &self.syllables {
                    writer.write_record(iter::once(syllable))?;
                }
                writer.flush()?;
            },
            _ => { println!("TODO")},
        }

        Ok(())
    }
}

pub struct SyllableBuilder {
    structure: SyllableStructure,
    vowels: Vec<String>,
    diphthongs: Vec<String>,
    y_diphthongs: Vec<String>,
    u_diphthongs: Vec<String>,
    consonants: Vec<String>,
    consonant_cluster: Vec<String>,
    cluster_settings: ClusterSettings,
    cluster_length: ClusterLength,
    no_init_consonants: Vec<String>,
    no_coda_consonants: Vec<String>,
    disabled_combinations: Vec<String>,
}

impl SyllableBuilder {
    pub fn new(structure: SyllableStructure) -> Self {
        Self {
            structure,
            vowels: Vec::new(),
            diphthongs: Vec::new(),
            y_diphthongs: Vec::new(),
            u_diphthongs: Vec::new(),
            consonants: Vec::new(),
            consonant_cluster: Vec::new(),
            cluster_settings: ClusterSettings::All,
            cluster_length: ClusterLength::CC,
            no_init_consonants: Vec::new(),
            no_coda_consonants: Vec::new(),
            disabled_combinations: Vec::new(),
        }
    }

    pub fn set_vowels(&mut self, vowels: Vec<String>) -> &mut SyllableBuilder {
        self.vowels.extend(vowels.iter().cloned());
        self
    }

    pub fn set_diphthongs(&mut self, no_double: bool) -> &mut SyllableBuilder {
        for vowel1 in &self.vowels {
            for vowel2 in &self.vowels {
                if no_double && vowel1 == vowel2 {
                    continue
                }
                self.diphthongs.push(format!("{}{}", vowel1, vowel2))
            }
        }
        self
    }

    pub fn set_diphthongs_custom(&mut self, diphthongs: Vec<String>) -> &mut SyllableBuilder {
        self.diphthongs.extend(diphthongs.iter().cloned());
        self
    }

    pub fn set_y_u_diphthongs(&mut self) -> &mut SyllableBuilder {
        for vowel in &self.vowels {
            self.y_diphthongs.push(format!("y{}", vowel));
            self.u_diphthongs.push(format!("w{}", vowel));
        }
        for diphthong in &self.diphthongs {
            self.y_diphthongs.push(format!("y{}", diphthong));
            self.u_diphthongs.push(format!("w{}", diphthong));
        }
        self
    }

    pub fn set_consonants(&mut self, consonants: Vec<String>) -> &mut SyllableBuilder {
        self.consonants.extend(consonants.iter().cloned());
        self
    }

    pub fn set_settings_cluster(&mut self, settings: ClusterSettings) -> &mut SyllableBuilder {
        self.cluster_settings = settings;
        self
    }

    // not needed when using OnlyGiven in custom
    pub fn set_cluster_length(&mut self, length: ClusterLength) -> &mut SyllableBuilder {
        self.cluster_length = length;
        self
    }

    pub fn set_consonant_cluster(&mut self) -> &mut SyllableBuilder {
        match self.cluster_settings {
            ClusterSettings::All => {
                match self.cluster_length {
                    ClusterLength::C => {
                        for consonant1 in &self.consonants {
                            self.consonant_cluster.push(format!("{}", consonant1));
                        }
                    }
                    ClusterLength::CC => {
                        for consonant1 in &self.consonants {
                            self.consonant_cluster.push(format!("{}", consonant1));
                            for consonant2 in &self.consonants {
                                self.consonant_cluster.push(format!("{}{}", consonant1, consonant2))
                            }
                        }
                    }
                    ClusterLength::CCC => {
                        for consonant1 in &self.consonants {
                            self.consonant_cluster.push(format!("{}", consonant1));
                            for consonant2 in &self.consonants {
                                self.consonant_cluster.push(format!("{}{}", consonant1, consonant2));
                                for consonant3 in &self.consonants {
                                    self.consonant_cluster.push(format!("{}{}{}", consonant1, consonant2, consonant3))
                                }
                            }
                        }
                    }
                    ClusterLength::CCCC => {
                        for consonant1 in &self.consonants {
                            self.consonant_cluster.push(format!("{}", consonant1));
                            for consonant2 in &self.consonants {
                                self.consonant_cluster.push(format!("{}{}", consonant1, consonant2));
                                for consonant3 in &self.consonants {
                                    self.consonant_cluster.push(format!("{}{}{}", consonant1, consonant2, consonant3));
                                    for consonant4 in &self.consonants {
                                        self.consonant_cluster.push(format!("{}{}{}{}", consonant1, consonant2, consonant3, consonant4))
                                    }
                                }
                            }
                        }
                    }
                }
            }
            ClusterSettings::NoDouble => {
                println!("TODO");
            },
            _ => { println!("USE CUSTOM")},
        }
        self
    }

    pub fn set_consonant_cluster_custom(&mut self, cluster: Vec<String>) -> &mut SyllableBuilder {
        match self.cluster_settings {
            ClusterSettings::OnlyGiven => {
                self.consonant_cluster.extend(self.consonants.iter().cloned());
                self.consonant_cluster.extend(cluster.iter().cloned());
            },
            ClusterSettings::DisableGiven => {
                println!("TODO");
            },
            _ => { println!("USE NORMAL")}
        }
        self
    }

    // also cluster
    pub fn no_init_consonants(&mut self, disabled_consonants: Vec<String>) -> &mut SyllableBuilder {
        self.no_init_consonants.extend(disabled_consonants.iter().cloned());
        self
    }

    // also cluster
    pub fn no_coda_consonants(&mut self, disabled_consonants: Vec<String>) -> &mut SyllableBuilder {
        self.no_coda_consonants.extend(disabled_consonants.iter().cloned());
        self
    }

    pub fn build(&self) -> SyllableHandler {
        let mut syllables: Vec<String> = Vec::new();

        let mut all_nucleus: Vec<String> = Vec::new();
        for vowel in &self.vowels {
            all_nucleus.push(vowel.clone());
        }
        for diph in &self.diphthongs {
            all_nucleus.push(diph.clone());
        }
        for y_diph in &self.y_diphthongs {
            all_nucleus.push(y_diph.clone());
        }
        for u_diph in &self.u_diphthongs {
            all_nucleus.push(u_diph.clone());
        }

        // CV
        for nucleus in &all_nucleus {
            for consonants in &self.consonant_cluster {
                if self.no_init_consonants.contains(consonants) {
                    continue
                };
                syllables.push(format!("{}{}", consonants, nucleus))
            }
        }

        // VC
        for nucleus in &all_nucleus {
            for consonants in &self.consonant_cluster {
                if self.no_coda_consonants.contains(consonants) {
                    continue
                };
                syllables.push(format!("{}{}", nucleus, consonants))
            }
        }

        // CVC
        for nucleus in &all_nucleus{
            for consonants_init in &self.consonant_cluster {
                if self.no_init_consonants.contains(consonants_init) {
                  continue
                };
                for consonants_coda in &self.consonant_cluster {
                    if self.no_coda_consonants.contains(consonants_coda) {
                        continue
                    };
                    syllables.push(format!("{}{}{}", consonants_init, nucleus, consonants_coda))
                }
            }
        }

        SyllableHandler::new(syllables)
    }
}

// old
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