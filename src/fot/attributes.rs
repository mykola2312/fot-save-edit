use crate::fot::decoder::Decoder;

use super::esh::{ESHValue, ESH};
use super::ferror::FError as FE;
use super::stream::{ReadStream, WriteStream};
use super::tag::Tag;
use indexmap::IndexMap;

const MAX_STATS: usize = 7;
const STATS: [&str; 7] = [
    "strength",
    "perception",
    "endurance",
    "charisma",
    "intelligence",
    "agility",
    "luck",
];

const MAX_TRAITS: usize = 11;
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

const MAX_DERIVED: usize = 26;
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

const MAX_SKILLS: usize = 18;
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

const MAX_OPT_TRAITS: usize = 38;
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

const MAX_PERKS: usize = 111;
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

const MAX_ADDICTIONS: usize = 10;
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
    size1: u32,
    size2: u32,
    tag: Tag,
    enc_size: usize,
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
    pub fn from_binary(bin: &[u8]) -> Result<Self, FE> {
        let mut rd = ReadStream::new(bin, 0);

        let size1 = rd.read_u32()?;
        let esh: ESH = rd.read()?;
        if esh.props["Binary"] == ESHValue::Bool(false) {
            return Err(FE::AttributesNonBinary);
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

            let size2 = rd.read_u32()?;
            let tag: Tag = rd.read()?;

            for i in 0..MAX_STATS {
                stats.insert(STATS[i], rd.read_u32()?);
            }
            for i in 0..MAX_TRAITS {
                traits.insert(TRAITS[i], rd.read_u32()?);
            }
            for i in 0..MAX_DERIVED {
                derived.insert(DERIVED[i], rd.read_u32()?);
            }
            for i in 0..MAX_SKILLS {
                skills.insert(SKILLS[i], rd.read_u32()?);
            }
            for i in 0..MAX_SKILLS {
                skill_tags.insert(SKILLS[i], rd.read_bool()?);
            }
            for i in 0..MAX_OPT_TRAITS {
                opt_traits.insert(OPT_TRAITS[i], rd.read_bool()?);
            }
            for i in 0..MAX_PERKS {
                perks.insert(PERKS[i], rd.read_u32()?);
            }
            for i in 0..MAX_ADDICTIONS {
                addictions.insert(ADDICTIONS[i], rd.read_u32()?);
            }

            let enc_size = binary.len();
            Ok(Attributes {
                esh,
                size1,
                size2,
                tag,
                enc_size,
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
            return Err(FE::ValueNoESBIN);
        }
    }

    pub fn into_binary(mut self) -> Result<Vec<u8>, FE> {
        let esbin = {
            let mut wd = WriteStream::new(self.enc_size);

            wd.write_u32(self.size2)?;
            wd.write(&self.tag)?;

            for (_, value) in self.stats {
                wd.write_u32(value)?;
            }
            for (_, value) in self.traits {
                wd.write_u32(value)?;
            }
            for (_, value) in self.derived {
                wd.write_u32(value)?;
            }
            for (_, value) in self.skills {
                wd.write_u32(value)?;
            }
            for (_, value) in self.skill_tags {
                wd.write_bool(value)?;
            }
            for (_, value) in self.opt_traits {
                wd.write_bool(value)?;
            }
            for (_, value) in self.perks {
                wd.write_u32(value)?;
            }
            for (_, value) in self.addictions {
                wd.write_u32(value)?;
            }

            wd.into_vec()
        };
        self.esh.set("esbin", ESHValue::Binary(esbin));

        let mut wd = WriteStream::new(self.esh.get_enc_size());
        wd.write_u32(self.size1)?;
        wd.write(&self.esh)?;

        Ok(wd.into_vec())
    }
}
