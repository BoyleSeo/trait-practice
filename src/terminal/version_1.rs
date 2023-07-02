use chrono::{DateTime, Utc};

use crate::feature::reward::{RewardDto, RewardGroup};

trait RewardDisplay {
    fn is_hidden(&self) -> bool;
    fn unit_image(&self) -> &str;
    fn image(&self) -> &str;
    fn fmt_string(&self) -> String;
}

trait TimeLimited {
    fn has_expired(&self) -> bool;
    fn fmt_expire_time(&self) -> String;
}

trait PlayioReward<T> {
    fn is_hidden(&self) -> bool;
}

struct Unknown;

struct Gem {
    delta: u32,
    min: Option<u32>,
    max: Option<u32>,
    is_hidden: bool,
}

struct Item {
    url: String,
    name: String,
    shelf_life: Option<DateTime<Utc>>,
    delta: u32,
    is_hidden: bool,
}

impl PlayioReward<Item> for Item {
    fn is_hidden(&self) -> bool {
        self.is_hidden
    }
}

struct RewardFactory;
impl RewardFactory {
    fn new(dto: RewardDto) -> Box<dyn RewardDisplay> {
        match dto.group {
            RewardGroup::ASSET => Box::new(Gem::from(dto)),
            RewardGroup::ITEM => Box::new(Item::from(dto)),
            _ => Box::new(Unknown),
        }
    }
}

impl From<RewardDto> for Gem {
    fn from(value: RewardDto) -> Self {
        Gem {
            delta: value.delta,
            min: value.min,
            max: value.max,
            is_hidden: value.is_hidden,
        }
    }
}

impl From<RewardDto> for Item {
    fn from(value: RewardDto) -> Self {
        Self {
            url: value.url.unwrap(),
            name: value.name.unwrap(),
            delta: value.delta,
            shelf_life: value.shelf_life,
            is_hidden: value.is_hidden,
        }
    }
}

impl RewardDisplay for Unknown {
    fn is_hidden(&self) -> bool {
        false
    }

    fn unit_image(&self) -> &str {
        ""
    }

    fn image(&self) -> &str {
        ""
    }

    fn fmt_string(&self) -> String {
        "".to_owned()
    }
}

impl RewardDisplay for Item {
    fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    fn unit_image(&self) -> &str {
        &self.url
    }

    fn image(&self) -> &str {
        &self.url
    }

    fn fmt_string(&self) -> String {
        format!("{} Item", self.name)
    }
}

impl RewardDisplay for Gem {
    fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    fn unit_image(&self) -> &str {
        "Gem Image"
    }

    fn image(&self) -> &str {
        "Gem Image"
    }

    fn fmt_string(&self) -> String {
        format!("{} Gems", self.delta)
    }
}

impl TimeLimited for Item {
    fn has_expired(&self) -> bool {
        match self.shelf_life {
            Some(d) => d < Utc::now(),
            None => false,
        }
    }

    fn fmt_expire_time(&self) -> String {
        match self.shelf_life {
            Some(d) => d.format("%Y-%m-%d %H:%M:%S").to_string(),
            None => String::from(""),
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

    let reward: Box<dyn RewardDisplay> = RewardFactory::new(dto);
    println!("v1 reward1: {}", reward.fmt_string());
    println!("v1 reward1: {}", reward.image());
    println!("v1 reward1: {}", reward.unit_image());

    // downcast 왜 안됨...
    let reward2 = RewardFactory::new(dto2);
}
