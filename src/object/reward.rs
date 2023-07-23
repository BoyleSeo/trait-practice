use chrono::{DateTime, Utc};
#[derive(Debug, Clone)]
pub(crate) enum RewardGroup {
    ASSET,
    AVATAR,
    ITEM,
    COUPON,
    QUEST,
}

#[derive(Clone)]
pub struct RewardDto {
    pub(crate) group: RewardGroup, // makes item visible within the current crate
    pub _type: String,
    pub name: Option<String>,
    pub code: Option<String>,
    pub delta: u32,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub url: Option<String>,
    pub is_hidden: bool,
    pub shelf_life: Option<DateTime<Utc>>,
}
