use std::collections::HashMap;

pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut out = HashMap::new();

    for path in paths.iter(){
        *out.entry(path[0].clone()).or_insert(0) +=1;
        out.entry(path[1].clone()).or_insert(0);
    }

    out.iter().find(|(_, &v)| v == 0).unwrap().0.clone()
}