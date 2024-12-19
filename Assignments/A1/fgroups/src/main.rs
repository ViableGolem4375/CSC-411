use std::collections::HashMap;
use std::io;

fn main() {
    let mut fingerprints: HashMap<_ , _> = HashMap::new();
    for line in io::stdin().lines() {
        let l = line.unwrap();
        let items: Vec<String> = l.split(spacer).map(String::from).collect();
        if items.len() < 2 {
            eprintln!("Error: input was invalid.");
            continue;
        }
        let fingerprint = &items[0];
        let name = &l[fingerprint.len() + 1..];
        fingerprints.entry(fingerprint.to_string()).or_insert(Vec::new()).push(name.to_string());
    }
    let mut fgroups = Vec::new();
    for (_, names) in fingerprints {
        if names.len() > 1 {
            fgroups.push(names);
        }
    }
    let mut counter = fgroups.len() as u32;
    let base = 1;
    //println!("{}", counter);
    for fgroup in fgroups {
        for name in fgroup {
            println!("{}", name.trim_start());
        }
        if counter > base {
            counter = counter - 1;
            println!();
        }
    }
}

fn spacer(space: char) -> bool {
	space == ' ' || space == '\t'
}
