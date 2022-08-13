// Thanks: https://stackoverflow.com/a/53894298/665869
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

type Map = HashMap<String, String>;

fn handle_n_times(count: i32, arc_map: Arc<Mutex<Map>>) {
    for i in 0..count {
        let clone_arc = arc_map.clone();
        thread::spawn(move || {
            let mut map = clone_arc.lock().unwrap();
            map.insert(format!("key-{}", i), format!("value-{}", i));
        });
    }
}

fn print_map_items(arc_map: Arc<Mutex<Map>>) {
    let clone_arc = arc_map.clone();
    let map = clone_arc.lock().unwrap();
    for (k, v) in map.iter() {
        println!("k: {} v: {}", k, v);
    }
}

fn main() {
    let arc_map = Arc::new(Mutex::new(Map::new()));
    handle_n_times(5, arc_map.clone());

    // todo: join the threads above, wait them till finished

    print_map_items(arc_map.clone());
}
