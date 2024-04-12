use std::collections::hash_map::{self, HashMap};
use std::iter;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};
pub trait JsMap<'a> {
    type Key: JsKey + 'a;
    type Value: JsValue + 'a;
    type EntryIter: Iterator<Item = (&'a Self::Key, &'a Self::Value)>;

    fn get_value(&self, key: &Self::Key) -> Option<&Self::Value>;

    fn entries(&'a self) -> Self::EntryIter;

    fn keys(&'a self) -> Keys<Self::EntryIter, Self::Key> {
        Keys {
            inner: self.entries(),
            mapper: |(k, _)| k,
        }
    }
}
pub trait JsKey: Eq + Hash + Ord + Debug + Display {}
impl JsKey for str {}
impl JsKey for String {}
impl JsKey for usize {}
impl JsKey for u8 {}
impl JsKey for isize {}
impl JsKey for i32 {}
impl<T: JsKey> JsKey for &T {}
// impl<T> JsKey for T where T: Sized + Deref<Target = str> {}
impl<T: ?Sized + JsKey> JsKey for Box<T> {}
impl<T: ?Sized + JsKey> JsKey for std::rc::Rc<T> {}
impl<T: ?Sized + JsKey> JsKey for std::sync::Arc<T> {}

pub trait JsValue: Sized + Debug + Display {}
impl<T> JsValue for T where T: Sized + Debug + Display {}

pub struct Keys<'a, JsMapIterator: Iterator, Key> {
    inner: JsMapIterator,
    mapper: fn(JsMapIterator::Item) -> &'a Key,
}
impl<'a, MapIter: Iterator, Key> Iterator for Keys<'a, MapIter, Key> {
    type Item = &'a Key;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(self.mapper)
    }
}

////////////////////////////////////////////////////////////////
impl<'a, K: JsKey + 'a, V: JsValue + 'a> JsMap<'a> for Vec<(K, V)> {
    type Key = K;
    type Value = V;
    type EntryIter = iter::Map<std::slice::Iter<'a, (K, V)>, fn(&'a (K, V)) -> (&'a K, &'a V)>;

    fn get_value(&self, key: &K) -> Option<&V> {
        self.iter()
            .find(|&(k, _)| k == key)
            .and_then(|(_, e)| Some(e))
    }

    fn entries(&'a self) -> Self::EntryIter {
        self.iter().map(|(k, v)| (k, v))
    }
}

////////////////////////////////////////////////////////////////
impl<'a, K: JsKey + 'a, V: JsValue + 'a> JsMap<'a> for HashMap<K, V> {
    type Key = K;
    type Value = V;
    type EntryIter = hash_map::Iter<'a, K, V>;

    fn get_value(&self, key: &K) -> Option<&V> {
        self.get(&key)
    }

    fn entries(&'a self) -> Self::EntryIter {
        self.iter()
    }
}

////////////////////////////////////////////////////////////////
// #[test]
pub fn test() {
    let vec_boxstr_str: Vec<(Box<str>, _)> = vec![
        (Box::from("3ho"), "!!!!!!!!!"), //
        (Box::from("2ya"), "~!~!~"),
        (Box::from("1mu"), "~~!"),
    ];
    let hm_isize_str: HashMap<isize, _> = HashMap::from_iter([
        (1, "mu"), //
        (2, "ya"),
        (3, "ho"),
    ]);

    println!("test begin");
    println!("\n~~~~Vec<(Box<str>, &str)>~~~~");
    test_js_map(&vec_boxstr_str);
    println!("\n~~~~HashMap<isize, &str>~~~~");
    test_js_map(&hm_isize_str);
}
pub fn test_js_map<'a>(map: &'a impl JsMap<'a>) {
    print!("\nentries: \n\t");
    for (k, v) in map.entries() {
        print!("({k}, {v}), ");
    }
    print!("\nkeys: \n\t");
    for key in map.keys() {
        print!("{key}, ");
    }
    print!("\n\nsorted_by_key: \n");
    let mut new_vec: Vec<_> = map.entries().collect();
    new_vec.sort_by_key(|&(k, _)| k);
    for (k, v) in new_vec {
        println!("\t{k}: {v}");
    }
}
