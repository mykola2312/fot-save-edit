use super::esh::{ESHValue, ESH};
use super::stream::{ReadStream, WriteStream};
use super::tag::Tag;
use anyhow::{anyhow, Result};
use indexmap::IndexMap;

const STATS: [&str; 7] = [
    "strength",
    "perception",
    "endurance",
    "charisma",
    "intelligence",
    "agility",
    "luck",
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
    "race",
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
    "levelsPerPerk",
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
    "outdoorsman",
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
    "doNightPerson",
];

const PERKS: [&str; 111] = [
    "awareness",
    "bonusHtHAttacks",
    "bonusHtHDamage",
    "bonusMove",
    "bonusRangedDamage",
    "bonusRateofFire",
    "fasterHealing",
    "moreCriticals",
    "nightVision",
    "radResistance",
    "toughness",
    "strongBack",
    "sharpshooter",
    "silentRunning",
    "survivalist",
    "masterTrader",
    "educated",
    "healer",
    "fortuneFinder",
    "betterCriticals",
    "slayer",
    "sniper",
    "silentDeath",
    "actionBoy",
    "lifegiver",
    "dodger",
    "snakeater",
    "mrFixit",
    "medic",
    "masterThief",
    "heaveHo",
    "pickpocket",
    "ghost",
    "explorer",
    "flowerChild",
    "pathfinder",
    "scout",
    "mysteriousStranger",
    "ranger",
    "quickPockets",
    "swiftLearner",
    "tag",
    "mutate",
    "adrenalineRush",
    "cautiousNature",
    "comprehension",
    "demolitionExpert",
    "gambler",
    "gainStrenght",
    "gainPerception",
    "gainEndurance",
    "gainCharisma",
    "gainIntelligence",
    "gainAgility",
    "gainLuck",
    "harmless",
    "hereandNow",
    "hthEvade",
    "lightStep",
    "livingAnatomy",
    "negotiator",
    "packRat",
    "pyromaniac",
    "quickRecovery",
    "salesman",
    "stonewall",
    "thief",
    "weaponHandling",
    "stuntMan",
    "crazyBomber",
    "roadWarrior",
    "gunner",
    "leadFoot",
    "tunnelRat",
    "bracing",
    "flexible",
    "bendTheRules",
    "breakTheRules",
    "loner",
    "teamPlayer",
    "leader",
    "hitTheDeck",
    "boneHead",
    "brownNoser",
    "dieHard",
    "drunkenMaster",
    "stat",
    "radChild",
    "cancerousGrowth",
    "bonsai",
    "steadyArm",
    "psychotic",
    "toughHige",
    "deathSense",
    "brutishHulk",
    "talonOfFear",
    "hideOfScars",
    "wayOfTheFruit",
    "twitchGamer",
    "bluffMaster",
    "divineFavour",
    "unk1",
    "unk2",
    "unk3",
    "unk4",
    "unk5",
    "unk6",
    "unk7",
    "unk8",
    "unk9",
    "unk10",
];

const ADDICTIONS: [&str; 10] = [
    "buffoutAddiction",
    "afterburnerAddiction",
    "mentatsAddiction",
    "psychoAddiction",
    "radAwayAddiction",
    "voodooAddiction",
    "nukaColaAddiction",
    "boozeAddiction",
    "withdrawal",
    "drunk",
];

#[derive(Debug)]
pub struct Attributes {
    esh: ESH,
    pub stats: IndexMap<&'static str, u32>,
    pub traits: IndexMap<&'static str, u32>,
    pub derived: IndexMap<&'static str, u32>,
    pub skills: IndexMap<&'static str, u32>,
    pub skill_tags: IndexMap<&'static str, bool>,
    pub opt_traits: IndexMap<&'static str, bool>,
    pub perks: IndexMap<&'static str, u32>,
    pub addictions: IndexMap<&'static str, u32>,
}

impl Attributes {
    pub fn from_binary(bin: &[u8]) -> Result<Self> {
        let mut rd = ReadStream::new(bin, 0);

        let _ = rd.read_u32()?;
        let esh: ESH = rd.read()?;
        if esh.props["Binary"] == ESHValue::Bool(false) {
            return Err(anyhow!("Attributes Binary == false"));
        }

        let mut stats: IndexMap<&'static str, u32> = IndexMap::with_capacity(7);
        let mut traits: IndexMap<&'static str, u32> = IndexMap::with_capacity(11);
        let mut derived: IndexMap<&'static str, u32> = IndexMap::with_capacity(26);
        let mut skills: IndexMap<&'static str, u32> = IndexMap::with_capacity(18);
        let mut skill_tags: IndexMap<&'static str, bool> = IndexMap::with_capacity(18);
        let mut opt_traits: IndexMap<&'static str, bool> = IndexMap::with_capacity(38);
        let mut perks: IndexMap<&'static str, u32> = IndexMap::with_capacity(111);
        let mut addictions: IndexMap<&'static str, u32> = IndexMap::with_capacity(10);

        if let ESHValue::Binary(binary) = &esh.props["esbin"] {
            let mut rd = ReadStream::new(&binary, 0);

            let _ = rd.read_u32()?;
            let _: Tag = rd.read()?;

            for i in 0..7 {
                stats.insert(STATS[i], rd.read_u32()?);
            }
            for i in 0..11 {
                traits.insert(TRAITS[i], rd.read_u32()?);
            }
            for i in 0..26 {
                derived.insert(DERIVED[i], rd.read_u32()?);
            }
            for i in 0..18 {
                skills.insert(SKILLS[i], rd.read_u32()?);
            }
            for i in 0..18 {
                skill_tags.insert(SKILLS[i], rd.read_bool()?);
            }
            for i in 0..38 {
                opt_traits.insert(OPT_TRAITS[i], rd.read_bool()?);
            }
            for i in 0..111 {
                perks.insert(PERKS[i], rd.read_u32()?);
            }
            for i in 0..10 {
                addictions.insert(ADDICTIONS[i], rd.read_u32()?);
            }

            Ok(Attributes {
                esh,
                stats,
                traits,
                derived,
                skills,
                skill_tags,
                opt_traits,
                perks,
                addictions,
            })
        } else {
            return Err(anyhow!("Attributes has no esbin"));
        }
    }
}
