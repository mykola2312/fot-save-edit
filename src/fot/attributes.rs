use super::stream::{ReadStream, WriteStream};
use super::esh::{ESH, ESHValue};
use super::tag::Tag;
use indexmap::IndexMap;
use anyhow::{Result, anyhow};

const STATS: [&str; 7] = [
    "strength",
    "perception",
    "endurance",
    "charisma",
    "intelligence",
    "agility",
    "luck"
];

const TRAITS: [&str; 11] = [
    "experience",
    "skillPoints",
    "tagsAvailable",
    "statsAvailable",
    "perksToTake",
    "rank",
    "reputation",
    "age",
    "bonusAC",
    "sex",
    "race"
];

const DERIVED: [&str; 26] = [
    "maxHitPoints",
    "maxCarryWeight",
    "maxActionPoints",
    "radiationResist",
    "poisonResist",
    "armorClass",
    "criticalChance",
    "fallover",
    "normalThresh",
    "energyThresh",
    "fireThresh",
    "gasThresh",
    "explodeThresh",
    "electricalThresh",
    "normalResist",
    "energyResist",
    "fireResist",
    "gasResist",
    "explodeResist",
    "electricalResist",
    "camoflage",
    "healRate",
    "meleeDamage",
    "bonusDamage",
    "skillPerLevel",
    "levelsPerPerk"
];

const SKILLS: [&str; 18] = [
    "smallGuns",
    "bigGuns",
    "energyWeapons",
    "unarmed",
    "meleeWeapons",
    "throwing",
    "firstAid",
    "doctor",
    "sneak",
    "lockpick",
    "steal",
    "traps",
    "science",
    "repair",
    "pilot",
    "barter",
    "gambling",
    "outdoorsman"
];

const OPT_TRAITS: [&str; 38] = [
    "fastMetabolism",
    "bruiser",
    "smallFrame",
    "oneHander",
    "finesse",
    "kamikaze",
    "heavyHanded",
    "fastShot",
    "bloodyMess",
    "jinxed",
    "goodNatured",
    "chemReliant",
    "chemResistant",
    "nightPerson",
    "skilled",
    "gifted",
    "glowingOne",
    "techWizard",
    "fearTheReaper",
    "vatSkin",
    "hamFisted",
    "domesticated",
    "rabid",
    "tightNuts",
    "targetingComputer",
    "betaSoftware",
    "empShielding",
    "Human",
    "Ghoul",
    "Mutant",
    "RobotHumanoid",
    "Deathclaw",
    "Dog",
    "doAdrenalineRush",
    "doDieHard",
    "doHthEvade",
    "doDrunkenMaster",
    "doNightPerson"
];

pub struct Attributes  {
    esh: ESH,
    pub stats: IndexMap<&'static str, u32>,
    pub traits: IndexMap<&'static str, u32>,
    pub derived: IndexMap<&'static str, u32>,
    pub skills: IndexMap<&'static str, u32>,
    pub skill_tags: IndexMap<&'static str, bool>,
    pub opt_traits: IndexMap<&'static str, bool>,
    pub perks: IndexMap<&'static str, u32>,
    pub addictions: IndexMap<&'static str, u32>
}

impl Attributes {
    fn from_binary(bin: &[u8]) -> Result<Self> {        
        let mut rd = ReadStream::new(bin, 0);
        
        let _ = rd.read_u32()?;
        let esh: ESH = rd.read()?;
        if esh.props["Binary"] == ESHValue::Bool(false) {
            return Err(anyhow!("Attributes Binary == false"));
        }

        let mut stats: IndexMap<&'static str, u32>;
        let mut traits: IndexMap<&'static str, u32>;
        let mut derived: IndexMap<&'static str, u32>;
        let mut skills: IndexMap<&'static str, u32>;
        let mut skill_tags: IndexMap<&'static str, bool>;
        let mut opt_traits: IndexMap<&'static str, bool>;
        let mut perks: IndexMap<&'static str, u32>;
        let mut addictions: IndexMap<&'static str, u32>;

        if let ESHValue::Binary(binary) = &esh.props["esbin"] {
            let mut rd = ReadStream::new(&binary, 0);
            
            let _ = rd.read_u32()?;
            let _: Tag = rd.read()?;
        } else {
            return Err(anyhow!("Attributes has no esbin"));
        }

        todo!()
    }
}
