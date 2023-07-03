use chrono::{DateTime, Utc};

use crate::feature::reward::{RewardDto, RewardGroup};

// enumc으로 Reward 정의하기?
enum Reward {
    Item {
        url: String,
        name: String,
        shelf_life: Option<DateTime<Utc>>,
        delta: u32,
        is_hidden: bool,
    },
    Gem {
        delta: u32,
        min: Option<u32>,
        max: Option<u32>,
        is_hidden: bool,
    },
    Unknown,
}

impl From<RewardDto> for Reward {
    fn from(value: RewardDto) -> Self {
        match value.group {
            RewardGroup::ITEM => Reward::Item {
                url: value.url.unwrap(),
                name: value.name.unwrap(),
                shelf_life: value.shelf_life,
                delta: value.delta,
                is_hidden: value.is_hidden,
            },
            RewardGroup::ASSET => match value._type == "XP" {
                true => Reward::Gem {
                    delta: value.delta,
                    min: value.min,
                    max: value.max,
                    is_hidden: value.is_hidden,
                },
                false => Reward::Unknown,
            },
            _ => Reward::Unknown,
        }
    }
}

trait TimeLimited {
    fn has_expired(&self) -> bool;
    fn fmt_expire_time(&self) -> String;
}

trait RewardDisplay {
    fn is_hidden(&self) -> bool;
    fn unit_image(&self) -> &str;
    fn image(&self) -> &str;
    fn fmt_string(&self) -> String;
    fn get_shelf_life(&self) -> Option<DateTime<Utc>>;
    fn has_expired(&self) -> bool {
        match self.get_shelf_life() {
            Some(d) => d < Utc::now(),
            None => false,
        }
    }
    fn fmt_expire_time(&self) -> String {
        match self.get_shelf_life() {
            Some(d) => d.format("%Y-%m-%d %H:%M:%S").to_string(),
            None => String::from(""),
        }
    }
}

impl RewardDisplay for Reward {
    fn is_hidden(&self) -> bool {
        match self {
            Reward::Gem { is_hidden, .. } => *is_hidden,
            Reward::Item { is_hidden, .. } => *is_hidden,
            _ => false,
        }
    }

    fn unit_image(&self) -> &str {
        match self {
            Reward::Gem { .. } => "Gem Image",
            Reward::Item { .. } => "Item Image",
            _ => "",
        }
    }

    fn image(&self) -> &str {
        match self {
            Reward::Gem { .. } => "Gem Image",
            Reward::Item { url, .. } => url,
            _ => "",
        }
    }

    fn fmt_string(&self) -> String {
        match self {
            Reward::Gem { delta, .. } => format!("{} Gems", delta),
            Reward::Item { name, .. } => format!("{} Item", name),
            _ => String::from(""),
        }
    }

    fn get_shelf_life(&self) -> Option<DateTime<Utc>> {
        match self {
            Reward::Item { shelf_life, .. } => *shelf_life,
            _ => None,
        }
    }
}

pub fn test() {
    let dto = RewardDto {
        group: RewardGroup::ASSET,
        delta: 3000,
        _type: String::from("XP"),
        name: None,
        code: None,
        min: None,
        max: None,
        url: None,
        is_hidden: false,
        shelf_life: None,
    };

    let dto2 = RewardDto {
        group: RewardGroup::ITEM,
        delta: 1,
        _type: String::from("1000XP"),
        name: Some(String::from("1,000-Gem Pouch")),
        url: Some(String::from(
            "https://static.playio.club/__asset/item_1000_gem_pack.png",
        )),
        code: None,
        min: None,
        max: None,
        is_hidden: false,
        shelf_life: Some(Utc::now()),
    };

    let reward = Reward::from(dto);
    let reward2 = Reward::from(dto2);

    println!("v2 reward1: {}", reward.fmt_string());
    println!("v2 reward1: {}", reward.image());
    println!("v2 reward1: {}", reward.unit_image());

    println!("v2 reward2: {}", reward2.fmt_string());
    println!("v2 reward2: {}", reward2.image());
    println!("v2 reward2: {}", reward2.unit_image());
    println!("v2 reward2: {}", reward2.fmt_expire_time());
}
