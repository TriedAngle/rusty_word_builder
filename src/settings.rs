/// Define which syllables should be generated
/// C also represents cluster
/// V also represents diphthongs and glides
/// All means CV, VC and CVC syllables
#[derive(PartialEq, Debug)]
pub enum SyllableStructure {
    All,
    CV,
    VC,
    CVC,
}

/// Define which diphthongs should be generated
/// All: create all possible combinations
/// NoLong: All without long vowels
/// OnlyGiven: only supplied diphthongs,
/// DisableGiven: All without supplied ones
///
/// If only some U/W and I/Y/J glides should be generated, add them here
#[derive(PartialEq, Debug)]
pub enum Diphthongs {
    All,
    NoLong,
    OnlyGiven,
    DisableGiven,
}

/// Define which glides should be generated
/// All: U/W and I/Y/J glides for all vowels and diphthongs
/// AllOnlyU: U/W glides for all vowels and diphthongs
/// AllOnlyY: I/Y/J glides for all vowels and diphthongs
#[derive(PartialEq, Debug)]
pub enum Glides {
    All,
    AllOnlyU,
    AllOnlyY,
}

/// Max cluster length
#[derive(PartialEq, Debug)]
pub enum ConsonantClusterLength {
    C,
    CC,
    CCC,
    CCCC,
    None,
}

/// Clusters represent single consonants and consonant clusters
/// single consonants are automatically added, do not supply them manually
/// All: create all possible cluster
/// NoDouble: create all possible cluster but no same in a row (ex: no bb)
/// OnlyGiven: only supplied clusters
/// DisableGiven: All possible consonant clusters but exclude the given ones.
#[derive(PartialEq, Debug)]
pub enum ConsonantCluster {
    All,
    NoDouble,
    OnlyGiven,
    DisableGiven,
}

pub enum WriteOption {
    CSV(String),
    PSQL,
}

pub enum SyllableFormat {
    IPA,
    Romanization,
}
