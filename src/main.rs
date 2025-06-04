#[cfg(feature = "js_map")]
#[path = "js_map/mod.rs"]
mod feature;

#[cfg(not(any(feature = "js_map", feature = "object")))]
#[path = "other.rs"]
mod feature;
#[cfg(feature = "object")]
#[path = "object/mod.rs"]
mod feature;

use feature::*;
fn main() {
    println!("\n[VERSION 1]");
    version_1::test();
    println!("\n[VERSION 2]");
    version_2::test();
    dbg!(Enum::try_from("crane"));
}
#[derive(Debug)]
enum Enum {
    Asimo,
    Balaclava,
    Crane,
    DdongJengE,
}
//이런건 매크로로 만드는 실습 하면 좋을 듯. derive 가능하게
impl TryFrom<&str> for Enum {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "asimo" => Ok(Enum::Asimo),
            "balaclava" => Ok(Enum::Balaclava),
            "ddongJengE" => Ok(Enum::DdongJengE),
            "crane" => Ok(Enum::Crane),
            _ => Err(()),
        }
    }
}

trait Traitor {
    type Foo;
    type Bar;

    fn is_valid(&self) -> bool;
    fn foo_foo(&self, n1: &Self::Foo, n2: &Self::Foo) -> bool;
    fn foo_bar(&self, n: &Self::Foo) -> Vec<Self::Bar>;
}

fn funk(traitor: impl Traitor) -> bool {
    traitor.is_valid()
}

fn bunk<T: Traitor<Foo = u8>>(traitor: T) -> bool {
    traitor.foo_bar(&234);
    traitor.is_valid()
}

fn grunge<T: Traitor>(traitor: &T, f1: &T::Foo, f2: &T::Foo) -> u32 {
    let is_foo = traitor.foo_foo(f1, f2);
    let bar = traitor.foo_bar(f2);
    0
}
