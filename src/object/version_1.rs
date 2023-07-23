use crate::feature::reward::{RewardDto, RewardGroup};
use chrono::{DateTime, Utc};
use std::thread;

enum RewardCast<'a> {
    Unknown,
    Gem(&'a Gem),
    Item(&'a Item),
}
#[derive(Debug)]
struct RewardDisplay {
    unit_image: Box<str>,
    image: Box<str>,
    fmt_string: Box<str>,
}

trait Reward {
    fn downcast(&self) -> RewardCast {
        RewardCast::Unknown
    }
    fn try_display(&self) -> Option<RewardDisplay> {
        None
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
type RewardObj = Box<dyn Reward + Send>;
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

impl Reward for Unknown {}
impl Reward for Item {
    fn downcast(&self) -> RewardCast {
        RewardCast::Item(self)
    }
    fn try_display(&self) -> Option<RewardDisplay> {
        Some(self.display())
    }
}
impl Reward for Gem {
    fn downcast(&self) -> RewardCast {
        RewardCast::Gem(self)
    }
    fn try_display(&self) -> Option<RewardDisplay> {
        if self.is_hidden {
            None
        } else {
            Some(self.display())
        }
    }
}
trait DisplayableReward: Reward {
    fn display(&self) -> RewardDisplay;
}
impl DisplayableReward for Item {
    fn display(&self) -> RewardDisplay {
        RewardDisplay {
            unit_image: Box::from(""),
            image: Box::from(""),
            fmt_string: Box::from(format!("Item({})", self.name)),
        }
    }
}

impl DisplayableReward for Gem {
    fn display(&self) -> RewardDisplay {
        RewardDisplay {
            unit_image: Box::from("Gem Image"),
            image: Box::from("Gem Image"),
            fmt_string: Box::from(format!("{} Gems", self.delta)),
        }
    }
}

struct RewardCastError;
impl Item {
    fn try_downcast_from(reward: &RewardObj) -> Result<&Self, RewardCastError> {
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
    fn try_downcast_from(reward: &RewardObj) -> Result<&Self, RewardCastError> {
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
    let reward1 = gen_reward(dto);
    if let Some(display) = reward1.try_display() {
        println!("reward1: {:?}", display);
    }
    if let Ok(gem) = Gem::try_downcast_from(&reward1) {
        print!("it shines!: ");
        gem.shine();
    }

    print!("\n");
    let reward2 = gen_reward(dto2);
    if let Some(display) = reward2.try_display() {
        println!("reward2: {:?}", display);
    }
    if let Ok(Item {
        name, shelf_life, ..
    }) = Item::try_downcast_from(&reward2)
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
            let gem = Gem::try_downcast_from(&reward_err)?;
            print!("it shines!: ");
            gem.shine();
            Ok(())
        }),
        RewardGroup::ITEM => Box::new(move || {
            let Item { name, .. } = Item::try_downcast_from(&reward_err)?;
            println!("Item name: {name}");
            Ok(())
        }),
        _ => Box::new(|| Ok(())),
    };
    if let Err(_) = cb() {
        println!("Invalid Reward data: reward_err");
    }
}
