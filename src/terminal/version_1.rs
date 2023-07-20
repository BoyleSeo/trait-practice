use std::thread;

use chrono::{DateTime, Utc};

use crate::feature::reward::{RewardDto, RewardGroup};

enum RewardCast {
    Unknown,
    Gem(Gem),
    Item(Item),
}
trait RewardDisplay {
    fn is_hidden(&self) -> bool;
    fn unit_image(&self) -> &str;
    fn image(&self) -> &str;
    fn fmt_string(&self) -> String;
    fn downcast(self: Box<Self>) -> RewardCast {
        RewardCast::Unknown
    }
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
type RewardObj = Box<dyn RewardDisplay + Send>;
trait RewardFactory {
    fn try_gen(dto: RewardDto) -> thread::Result<RewardObj> {
        //not recommended. panic catch하는 용법 자체를 피하는게 좋다고 합니다.
        std::panic::catch_unwind(|| {
            match dto.group {
                RewardGroup::ASSET => Box::new(Gem::from(dto)),
                RewardGroup::ITEM => {
                    //From & Into trait 은 서로를 자동구현시킴
                    let item: Item = dto.into();
                    Box::new(item) as RewardObj
                }
                _ => Box::new(Unknown),
            }
        })
    }
    fn gen(dto: RewardDto) -> RewardObj;
}
//stateless 서비스 다형성 예시
struct RewardFactoryUnsafe;
impl RewardFactory for RewardFactoryUnsafe {
    fn gen(dto: RewardDto) -> RewardObj {
        Self::try_gen(dto).unwrap()
    }
}
struct RewardFactorySafe;
impl RewardFactory for RewardFactorySafe {
    fn gen(dto: RewardDto) -> RewardObj {
        Self::try_gen(dto).unwrap_or_else(|err| {
            if let Ok(err_str) = err.downcast::<&str>() {
                println!("ERROR REPORT: {:?} (RewardFactory::new)", err_str);
            }
            Box::new(Unknown)
        })
    }
}
fn gen_reward<T: RewardFactory>(dto: RewardDto) -> RewardObj {
    T::gen(dto)
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

    fn downcast(self: Box<Self>) -> RewardCast {
        RewardCast::Item(*self)
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
    fn downcast(self: Box<Self>) -> RewardCast {
        RewardCast::Gem(*self)
    }
}

struct RewardCastError;
impl Item {
    fn try_downcast_from(reward: RewardObj) -> Result<Self, RewardCastError> {
        if let RewardCast::Item(item) = reward.downcast() {
            Ok(item)
        } else {
            Err(RewardCastError)
        }
    }
}
impl Gem {
    fn shine(&self) {
        println!("bling bling");
    }
    fn try_downcast_from(reward: RewardObj) -> Result<Self, RewardCastError> {
        if let RewardCast::Gem(gem) = reward.downcast() {
            Ok(gem)
        } else {
            Err(RewardCastError)
        }
    }
}
pub fn test() {
    assert_eq!(
        DateTime::<Utc>::default(), //1970.1.1
        Utc::now()
            .checked_sub_days(chrono::Days::new(18446744073709551615))
            .unwrap_or_default()
    );
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
        shelf_life: Utc::now()
            .checked_sub_days(chrono::Days::new(1))
            // .checked_sub_signed(chrono::Duration::days(1)) //dst로인한 버그가능
            .or(Some(DateTime::default())),
    };
    let dto_invalid = dto2.clone();
    let gen_reward = gen_reward::<RewardFactoryUnsafe>;
    let reward = gen_reward(dto);
    println!("reward1: {}", reward.fmt_string());
    println!("reward1: {}", reward.image());
    println!("reward1: {}", reward.unit_image());
    //ownership 이동
    if let Ok(gem) = Gem::try_downcast_from(reward) {
        print!("it shines!: ");
        gem.shine();
    }

    print!("\n");
    let reward2 = gen_reward(dto2);
    println!("reward2: {}", reward2.fmt_string());
    if let Ok(Item {
        name, shelf_life, ..
    }) = Item::try_downcast_from(reward2)
    {
        print!("Item {name}");
        if let Some(expr) = shelf_life.expired_at() {
            print!(" (EXPIRED AT {})", expr.label());
        }
        println!();
    }
    print!("\n");
    factory_glitched(dto_invalid);
}
trait ShelfLife {
    fn expired_at(&self) -> Option<&DateTime<Utc>>;
}
impl ShelfLife for Option<DateTime<Utc>> {
    fn expired_at(&self) -> Option<&DateTime<Utc>> {
        self.as_ref().filter(|&d| d < &Utc::now())
    }
}
impl ShelfLife for Item {
    fn expired_at(&self) -> Option<&DateTime<Utc>> {
        self.shelf_life.expired_at()
    }
}
trait DateTimeExt {
    fn label(&self) -> String;
}
impl DateTimeExt for DateTime<Utc> {
    fn label(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
fn factory_glitched(dto: RewardDto) {
    let dto = RewardDto {
        _type: String::from("*@#&^$*@&#^$"),
        name: None,
        url: None,
        code: None,
        min: None,
        max: None,
        shelf_life: None,
        ..dto
    };
    let reward_err = gen_reward::<RewardFactorySafe>(dto.clone());
    let cb: Box<dyn FnOnce() -> Result<(), RewardCastError>> = match dto.group {
        RewardGroup::ASSET => Box::new(move || {
            let gem = Gem::try_downcast_from(reward_err)?;
            print!("it shines!: ");
            gem.shine();
            Ok(())
        }),
        RewardGroup::ITEM => Box::new(move || {
            let Item { name, .. } = Item::try_downcast_from(reward_err)?;
            println!("Item name: {name}");
            Ok(())
        }),
        _ => Box::new(|| Ok(())),
    };
    if let Err(_) = cb() {
        println!("Invalid Reward data: reward_err");
    }
}
