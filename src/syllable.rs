use crate::settings::*;
use core::iter;
use csv::Writer;
use std::error::Error;

/// All vowels are moved into diphthongs
/// All consonants are moved into consonant_cluster
/// this is for easier creation
/// to avoid "undefined behaviour", call the functions in the impl order
pub struct SyllableBuilder {
    syllable_structure: SyllableStructure,
    diphthong_settings: Diphthongs,
    glide_settings: Glides,
    consonant_cluster_settings: ConsonantCluster,
    consonant_cluster_length: ConsonantClusterLength,
    disabled_combinations: Vec<String>,

    vowels: Vec<String>,
    diphthongs: Vec<String>,
    disabled_diphthongs: Vec<String>,

    consonants: Vec<String>,
    consonant_cluster: Vec<String>,

    disabled_init_cluster: Vec<String>,
    disabled_coda_cluster: Vec<String>,

    init_cluster: Vec<String>,
    nucleus: Vec<String>,
    coda_cluster: Vec<String>,
}

impl SyllableBuilder {
    /// init a new SyllableBuilder with default settings and not content
    pub fn new() -> SyllableBuilder {
        SyllableBuilder {
            syllable_structure: SyllableStructure::All,
            diphthong_settings: Diphthongs::All,
            glide_settings: Glides::All,
            consonant_cluster_settings: ConsonantCluster::All,
            consonant_cluster_length: ConsonantClusterLength::CC,
            disabled_combinations: Vec::new(),

            vowels: Vec::new(),
            diphthongs: Vec::new(),
            disabled_diphthongs: Vec::new(),

            consonants: Vec::new(),
            consonant_cluster: Vec::new(),
            disabled_init_cluster: Vec::new(),
            disabled_coda_cluster: Vec::new(),

            init_cluster: Vec::new(),
            nucleus: Vec::new(),
            coda_cluster: Vec::new(),
        }
    }

    // START SETTINGS
    pub fn with_syllable_structure_settings(
        &mut self,
        syllable_structure: SyllableStructure,
    ) -> &mut SyllableBuilder {
        self.syllable_structure = syllable_structure;
        self
    }

    pub fn with_diphthong_settings(
        &mut self,
        diphthong_settings: Diphthongs,
    ) -> &mut SyllableBuilder {
        self.diphthong_settings = diphthong_settings;
        self
    }

    pub fn with_glide_settings(&mut self, glide_settings: Glides) -> &mut SyllableBuilder {
        self.glide_settings = glide_settings;
        self
    }

    pub fn with_consonant_cluster_settings(
        &mut self,
        consonant_cluster_settings: ConsonantCluster,
        consonant_cluster_length: ConsonantClusterLength,
    ) -> &mut SyllableBuilder {
        self.consonant_cluster_settings = consonant_cluster_settings;
        self.consonant_cluster_length = consonant_cluster_length;
        self
    }

    pub fn with_disabled_diphthongs(
        &mut self,
        disabled_diphthongs: Vec<String>,
    ) -> &mut SyllableBuilder {
        assert_eq!(self.diphthong_settings, Diphthongs::DisableGiven);

        self.disabled_diphthongs = disabled_diphthongs;
        self
    }

    pub fn with_disabled_init_cluster(
        &mut self,
        disabled_init_cluster: Vec<String>,
    ) -> &mut SyllableBuilder {
        self.disabled_init_cluster = disabled_init_cluster;
        self
    }

    pub fn with_disabled_coda_cluster(
        &mut self,
        disabled_coda_cluster: Vec<String>,
    ) -> &mut SyllableBuilder {
        self.disabled_coda_cluster = disabled_coda_cluster;
        self
    }

    pub fn with_disabled_combinations(
        &mut self,
        disabled_combinations: Vec<String>,
    ) -> &mut SyllableBuilder {
        self.disabled_combinations = disabled_combinations;
        self
    }
    // END SETTINGS

    // START DATA
    pub fn with_vowel_data(&mut self, vowels: Vec<String>) -> &mut SyllableBuilder {
        self.vowels = vowels;
        self
    }

    pub fn with_diphthong_data(&mut self, diphthongs: Vec<String>) -> &mut SyllableBuilder {
        assert_eq!(self.diphthong_settings, Diphthongs::OnlyGiven);
        self.diphthongs = diphthongs;
        self
    }

    pub fn with_consonant_data(&mut self, consonants: Vec<String>) -> &mut SyllableBuilder {
        self.consonants = consonants;
        self
    }

    pub fn with_consonant_cluster_data(
        &mut self,
        consonant_cluster: Vec<String>,
    ) -> &mut SyllableBuilder {
        assert_eq!(self.consonant_cluster_settings, ConsonantCluster::OnlyGiven);
        self.consonant_cluster = consonant_cluster;
        self
    }
    // END DATA

    // START CREATION
    pub fn create_diphthongs(&mut self) -> &mut SyllableBuilder {
        match self.diphthong_settings {
            Diphthongs::All => {
                for vowel1 in &self.vowels {
                    for vowel2 in &self.vowels {
                        self.diphthongs.push(format!("{}{}", vowel1, vowel2))
                    }
                }
            }
            Diphthongs::DisableGiven => {
                for vowel1 in &self.vowels {
                    for vowel2 in &self.vowels {
                        let diphthong = format!("{}{}", vowel1, vowel2);
                        if self.disabled_diphthongs.contains(&diphthong) {
                            continue;
                        }
                        self.diphthongs.push(diphthong);
                    }
                }
            }
            Diphthongs::NoLong => {
                for vowel1 in &self.vowels {
                    for vowel2 in &self.vowels {
                        if vowel1 == vowel2 {
                            continue;
                        }
                        self.diphthongs.push(format!("{}{}", vowel1, vowel2))
                    }
                }
            }
            _ => {}
        }
        self
    }

    pub fn create_glides(&mut self) -> &mut SyllableBuilder {
        let mut glides = Vec::new();
        match self.glide_settings {
            Glides::All => {
                for vowel in &self.vowels {
                    glides.push(format!("y{}", vowel));
                    glides.push(format!("w{}", vowel));
                }
                for diphthong in &self.diphthongs {
                    glides.push(format!("y{}", diphthong));
                    glides.push(format!("w{}", diphthong));
                }
            }
            Glides::AllOnlyU => {
                for vowel in &self.vowels {
                    glides.push(format!("w{}", vowel));
                }
                for diphthong in &self.diphthongs {
                    glides.push(format!("w{}", diphthong));
                }
            }
            Glides::AllOnlyY => {
                for vowel in &self.vowels {
                    glides.push(format!("y{}", vowel));
                }
                for diphthong in &self.diphthongs {
                    glides.push(format!("y{}", diphthong));
                }
            }
        }
        self.diphthongs.extend(glides);
        self
    }

    pub fn create_consonant_cluster(&mut self) -> &mut SyllableBuilder {
        assert_ne!(self.consonant_cluster_length, ConsonantClusterLength::C);
        assert_ne!(self.consonant_cluster_settings, ConsonantCluster::OnlyGiven);
        match self.consonant_cluster_length {
            ConsonantClusterLength::CC => {
                for consonant1 in &self.consonants {
                    for consonant2 in &self.consonants {
                        self.consonant_cluster
                            .push(format!("{}{}", consonant1, consonant2))
                    }
                }
            }
            ConsonantClusterLength::CCC => {
                for consonant1 in &self.consonants {
                    for consonant2 in &self.consonants {
                        self.consonant_cluster
                            .push(format!("{}{}", consonant1, consonant2));
                        for consonant3 in &self.consonants {
                            self.consonant_cluster
                                .push(format!("{}{}{}", consonant1, consonant2, consonant3))
                        }
                    }
                }
            }
            ConsonantClusterLength::CCCC => {
                for consonant1 in &self.consonants {
                    for consonant2 in &self.consonants {
                        self.consonant_cluster
                            .push(format!("{}{}", consonant1, consonant2));
                        for consonant3 in &self.consonants {
                            self.consonant_cluster
                                .push(format!("{}{}{}", consonant1, consonant2, consonant3));
                            for consonant4 in &self.consonants {
                                self.consonant_cluster.push(format!(
                                    "{}{}{}{}",
                                    consonant1, consonant2, consonant3, consonant4
                                ))
                            }
                        }
                    }
                }
            }
            ConsonantCluster::None => panic!("Do not use this option with custom data"),
            _ => {}
        }
        self
    }

    pub fn create_nucleus(&mut self) -> &mut SyllableBuilder {
        for vowel in &self.vowels {
            self.nucleus.push(vowel.clone());
        }

        for diphthong in &self.diphthongs {
            self.nucleus.push(diphthong.clone())
        }

        self
    }

    pub fn create_init_cluster(&mut self) -> &mut SyllableBuilder {
        for consonant in &self.consonants {
            if self.disabled_init_cluster.contains(consonant) {
                continue;
            }
            self.init_cluster.push(consonant.clone());
        }

        for cluster in &self.consonant_cluster {
            if self.disabled_init_cluster.contains(cluster) {
                continue;
            }
            self.init_cluster.push(cluster.clone());
        }

        self
    }

    pub fn create_coda_cluster(&mut self) -> &mut SyllableBuilder {
        for consonant in &self.consonants {
            if self.disabled_coda_cluster.contains(consonant) {
                continue;
            }
            self.coda_cluster.push(consonant.clone());
        }

        for cluster in &self.consonant_cluster {
            if self.disabled_coda_cluster.contains(cluster) {
                continue;
            }
            self.coda_cluster.push(cluster.clone());
        }

        self
    }

    pub fn build(&self) -> SyllableHandler {
        let mut syllables = Vec::new();
        for nucleus in &self.nucleus {
            syllables.push(format!("{}", nucleus));
            for init in &self.init_cluster {
                let combination = format!("{}{}", init, nucleus);
                if self.disabled_combinations.contains(&combination) {
                    continue;
                };
                syllables.push(combination);
            }
            for coda in &self.coda_cluster {
                let combination = format!("{}{}", nucleus, coda);
                if self.disabled_combinations.contains(&combination) {
                    continue;
                };
                syllables.push(combination);
            }
            for init in &self.init_cluster {
                for coda in &self.coda_cluster {
                    let combination = format!("{}{}{}", init, nucleus, coda);
                    if self.disabled_combinations.contains(&combination) {
                        continue;
                    };
                    syllables.push(format!("{}{}{}", init, nucleus, coda));
                }
            }
        }
        SyllableHandler::new(syllables)
    }
    // END CREATION
}

pub struct SyllableHandler {
    syllables: Vec<String>,
}

impl SyllableHandler {
    pub fn new(syllables: Vec<String>) -> Self {
        Self { syllables }
    }

    pub fn all(&self) -> Vec<String> {
        self.syllables.clone()
    }

    pub fn write(
        &self,
        write_option: WriteOption,
        syllable_format: SyllableFormat,
    ) -> Result<(), Box<dyn Error>> {
        match syllable_format {
            SyllableFormat::IPA => println!("FORMAT TODO"),
            SyllableFormat::Romanization => println!("FORMAT TODO"),
        }

        match write_option {
            WriteOption::CSV(path) => {
                let mut writer = Writer::from_path(path)?;
                for syllable in &self.syllables {
                    writer.write_record(iter::once(syllable))?;
                }
                writer.flush()?;
            }
            _ => println!("WRITE OPTION TODO"),
        }

        Ok(())
    }
}
