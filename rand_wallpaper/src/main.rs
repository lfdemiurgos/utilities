use rand::Rng;
use std::env;
use std::fs;

fn main() {
    let path = format!(
        "{}/Images/Wallpapers",
        env::var("HOME").expect("Set $HOMe environment variable")
    );
    let wallpapers = fs::read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|wall| wall.path())
        .collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    println!(
        "{}",
        wallpapers[rng.gen_range(0..wallpapers.len())].display()
    )
}
