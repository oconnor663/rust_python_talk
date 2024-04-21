use std::collections::BTreeMap;
use std::ops::DerefMut;
use std::sync::Mutex;

type Map = BTreeMap<String, i32>;

static MY_MAP: Mutex<Map> = Mutex::new(Map::new());

fn foo(maps_list: &mut Vec<&mut Map>) {
    // We can get a pointer to the map.
    let mut guard = MY_MAP.lock().unwrap();
    let map: &mut Map = guard.deref_mut();

    // We can use that pointer to edit the map.
    map.insert("foo".into(), 42);

    // But we can't save it.
    maps_list.push(map);
}
