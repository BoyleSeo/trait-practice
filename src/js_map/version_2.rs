use std::collections::hash_map::HashMap;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};
type JsMapIterator<'a, K, V> = Box<dyn Iterator<Item = (&'a K, &'a V)> + 'a>;
pub trait JsMap<K: JsKey, V: JsValue> {
    fn get_value(&self, key: &K) -> Option<&V>;

    fn entries(&self) -> JsMapIterator<K, V>;

    fn keys(&self) -> Keys<JsMapIterator<K, V>, K> {
        Keys {
            inner: self.entries(),
            mapper: |(k, _)| k,
        }
    }
}
pub trait JsKey: Sized + Eq + Hash + Ord + Clone + Debug + Display {}
impl JsKey for isize {}
impl JsKey for &str {}
impl JsKey for String {}
impl JsKey for Box<str> {}
impl JsKey for std::rc::Rc<str> {}
impl JsKey for std::sync::Arc<str> {}
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
impl<K: JsKey, V: JsValue> JsMap<K, V> for Vec<(K, V)> {
    fn get_value(&self, key: &K) -> Option<&V> {
        self.iter()
            .find(|&(k, _)| k == key)
            .and_then(|(_, e)| Some(e))
    }

    fn entries(&self) -> JsMapIterator<K, V> {
        Box::new(self.iter().map(|(k, v)| (k, v)))
    }
}

////////////////////////////////////////////////////////////////
impl<K: JsKey, V: JsValue> JsMap<K, V> for HashMap<K, V> {
    fn get_value(&self, key: &K) -> Option<&V> {
        self.get(&key)
    }

    fn entries(&self) -> JsMapIterator<K, V> {
        Box::new(self.iter())
    }
}

////////////////////////////////////////////////////////////////
// #[test]
pub fn test() {
    let vec_i_f = vec![
        (122, 9.343), //
        (121, 2.6),
        (120, 5.5),
    ];
    let hm_i_f = HashMap::from([
        (3, 24.36), //
        (4, 2.436),
        (5, 243.6),
        (6, 2.436),
    ]);
    let vec_str_str = vec![
        ("3ho", "!!!!!!!!!"), //
        ("2ya", "~!~!~"),
        ("1mu", "~~!"),
    ];
    let hm_str_str = HashMap::from_iter([
        ("a", "mu"), //
        ("b", "ya"),
        ("c", "ho"),
    ]);

    let test_i_f = test_js_map;
    let test_str_str = test_js_map;

    println!("test begin");
    println!("\n~~~~Vec<(i32, f64)>~~~~");
    test_i_f(&vec_i_f);
    println!("\n~~~~HashMap<i32, f64>~~~~");
    test_i_f(&hm_i_f);
    println!("\n~~~~Vec<(&str, &str)>~~~~");
    test_str_str(&vec_str_str);
    println!("\n~~~~HashMap<&str, &str>~~~~");
    test_str_str(&hm_str_str);
}
pub fn test_js_map<K: JsKey, V: JsValue>(map: &dyn JsMap<K, V>) {
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
