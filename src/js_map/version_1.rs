use std::collections::hash_map::{self, HashMap};
use std::iter;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};
pub trait JsMap<'a, K: JsKey + 'a, V: JsValue + 'a> {
    type EntryIter: Iterator<Item = (&'a K, &'a V)>;

    fn get_value(&self, key: &K) -> Option<&V>;

    fn entries(&'a self) -> Self::EntryIter;

    fn keys(&'a self) -> Keys<Self::EntryIter, K> {
        Keys {
            inner: self.entries(),
            mapper: |(k, _)| k,
        }
    }
}
pub trait JsKey: Sized + Eq + Hash + Ord + Clone + Debug + Display {}
// impl JsKey for isize {}
// impl JsKey for &str {}
// impl JsKey for String {}
// impl JsKey for Box<str> {}
// impl JsKey for std::rc::Rc<str> {}
// impl JsKey for std::sync::Arc<str> {}
impl<T> JsKey for T
where
    T: std::ops::Deref<Target = str>,
    T: Sized + Eq + Hash + Ord + Clone + Debug + Display,
{
}
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
impl<'a, K: JsKey + 'a, V: JsValue + 'a> JsMap<'a, K, V> for Vec<(K, V)> {
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
impl<'a, K: JsKey + 'a, V: JsValue + 'a> JsMap<'a, K, V> for HashMap<K, V> {
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
    let vec_boxstr_str = vec![
        (Box::from("3ho"), "!!!!!!!!!"), //
        (Box::from("2ya"), "~!~!~"),
        (Box::from("1mu"), "~~!"),
    ];
    let hm_boxstr_str = HashMap::from_iter([
        (Box::from("a"), "mu"), //
        (Box::from("b"), "ya"),
        (Box::from("c"), "ho"),
    ]);

    println!("test begin");
    println!("\n~~~~Vec<(Box<str>, &str)>~~~~");
    test_js_map(&vec_boxstr_str);
    println!("\n~~~~HashMap<Box<str>, &str>~~~~");
    test_js_map(&hm_boxstr_str);
}
pub fn test_js_map<'a, K, V, E>(map: &'a impl JsMap<'a, K, V, EntryIter = E>)
where
    K: JsKey + 'a,
    V: JsValue + 'a,
    E: Iterator<Item = (&'a K, &'a V)>,
{
    print!("\nentries: \n\t");
    for (k, v) in map.entries() {
        print!("({k}, {v}), ");
    }
    print!("\nkeys: \n\t");
    for key in map.keys() {
        print!("{key}, ");
    }
    print!("\n\nsorted_by_key: \n");
    let mut new_vec: Vec<(&K, &V)> = map.entries().collect();
    new_vec.sort_by_key(|&(k, _)| k);
    for (k, v) in new_vec {
        println!("\t{k}: {v}");
    }
}
