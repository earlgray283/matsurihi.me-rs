use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::idol::Idol;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i32,
    pub name: String,
    pub sort_id: i32,
    pub idol_id: Idol,
    pub idol_type: IdolType,
    pub resource_id: String,
    pub rarity: Rarity,
    pub event_id: Option<i32>,
    pub category: Category,
    pub extra_type: ExtraType,
    pub costume: Option<Costume>,
    pub bonus_costume: Option<Costume>,
    pub rank5_costume: Option<Costume>,
    pub flavor_text: Option<String>,
    pub flavor_text_awakened: Option<String>,
    pub level_max: i32,
    pub level_max_awakened: i32,
    pub vocal_min: i32,
    pub vocal_max: i32,
    pub vocal_min_awakened: i32,
    pub vocal_max_awakened: i32,
    pub vocal_master_bonus: i32,
    pub dance_min: i32,
    pub dance_max: i32,
    pub dance_min_awakened: i32,
    pub dance_max_awakened: i32,
    pub dance_master_bonus: i32,
    pub visual_min: i32,
    pub visual_max: i32,
    pub visual_min_awakened: i32,
    pub visual_max_awakened: i32,
    pub visual_master_bonus: i32,
    pub life: i32,
    pub master_rank_max: i32,
    pub center_effect: Option<CenterEffect>,
    pub center_effect_name: Option<String>,
    pub skill: Option<Vec<Skill>>,
    pub add_date: Option<chrono::DateTime<chrono::Local>>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum IdolType {
    Princess = 1,
    Fairy = 2,
    Angel = 3,
    All = 4,
    Ex = 5,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum Rarity {
    N = 1,
    R = 2,
    SR = 3,
    SSR = 4,
}

impl std::fmt::Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Rarity::N => write!(f, "n"),
            Rarity::R => write!(f, "r"),
            Rarity::SR => write!(f, "sr"),
            Rarity::SSR => write!(f, "ssr"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Category {
    Normal1, // 初期ノーマル
    Gasha0,  // 恒常
    Gasha1,  // 限定
    Gasha2,  // フェス
    Gasha4,  // プレミアムピックアップ SR
    Gasha5,  // セカンドヘアスタイル
    Event0,  // ミリコレ
    Event1,  // シアター
    Event2,  // ツアー
    Event3,  // 周年イベ
    Event4,  // 投票イベント追加 SR
    Event5,  // ミリコレ(R)
    Other,   // その他
}

impl std::str::FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal1" => Ok(Category::Normal1),
            "gasha0" => Ok(Category::Gasha0),
            "gasha1" => Ok(Category::Gasha1),
            "gasha2" => Ok(Category::Gasha2),
            "gasha4" => Ok(Category::Gasha4),
            "gasha5" => Ok(Category::Gasha5),
            "event0" => Ok(Category::Event0),
            "event1" => Ok(Category::Event1),
            "event2" => Ok(Category::Event2),
            "event3" => Ok(Category::Event3),
            "event4" => Ok(Category::Event4),
            "event5" => Ok(Category::Event5),
            "other" => Ok(Category::Other),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        stringify!(self).to_lowercase().fmt(f)
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(i32)]
#[serde(rename_all = "camelCase")]
pub enum ExtraType {
    None = 0,
    PSTRankingReward = 2,
    PSTPointReward = 3,
    Fes = 4,
    FirstAnivReward = 5,
    ExtraCard = 6,
    SecondAnivReward = 7,
    ExtraPSTRankingReward = 8,
    ExtraPSTPointReward = 9,
    ThirdAnivReward = 10,
    ExtraPSTRankingReward2 = 11,
    ExtraPSTPointReward2 = 12,
    ForthAnivReward = 13,
    SecondHairStyle = 14,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Costume {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub resource_id: String,
    pub model_id: String,
    pub sort_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CenterEffect {
    pub id: i32,
    pub description: String,
    pub idol_type: IdolType,
    pub specific_idol_type: Option<IdolType>,
    pub attribute: Attribute,
    pub value: i32,
    pub song_type: Option<IdolType>,
    pub attribute2: Option<Attribute>,
    pub value2: Option<i32>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(i32)]
#[serde(rename_all = "camelCase")]
pub enum Attribute {
    Vocal = 1,
    Dance = 2,
    Visual = 3,
    AllAppeal = 4,
    Life = 5,
    SkillProbablity = 6,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub id: i32,
    pub description: String,
    pub effect_id: EffectID,
    pub evaluation: Evaluation,
    pub evaluation2: Evaluation,
    pub duration: i32,
    pub interval: i32,
    pub probability: i32,
    pub value: Vec<i32>,
    pub skill_name: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(i32)]
#[serde(rename_all = "camelCase")]
pub enum EffectID {
    ScoreUp = 1,
    ComboBonus = 2,
    LifeRecovery = 3,
    DamageGuard = 4,
    ComboContinuation = 5,
    EnhancedEvalution = 6,
    DoubleBoost = 7,
    MultiUp = 8,
    OverClock = 10,
    OverRondo = 11,
    DoubleEffect = 12,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(i32)]
#[serde(rename_all = "camelCase")]
pub enum Evaluation {
    All = 0,
    Perfect = 1,
    PerfectGreat = 2,
    Great = 3,
    GreatGoodFastSlow = 4,
    PerfectGreatGood = 5,
    PerfectGreatGoodFastSlow = 6,
    GreatGood = 7,
}
