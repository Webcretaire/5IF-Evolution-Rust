mod monde;
mod individu;
mod prob;

use getopts::Options;
use monde::Monde;
use std::env;
use getopts::Matches;

fn get_i64_opt(matches: &Matches, arg: &str, default: i64) -> i64 {
    let val = matches.opt_str(arg);
    return if val.is_none() { default } else { val.unwrap().parse::<i64>().unwrap() };
}

fn main() {
    let args: Vec<String> = env::args()
        .map(|x| x.to_string())
        .collect();

    let mut opts = Options::new();
    opts.optopt("n", "", "", "VALUE");
    opts.optopt("t", "", "", "VALUE");
    opts.optopt("i", "", "", "VALUE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => { panic!(f.to_string()) }
    };

    let taille_initiale = get_i64_opt(&matches, "t", 0);
    let nombre_initial = get_i64_opt(&matches, "n", 0);
    let iterations = get_i64_opt(&matches, "i", 0);

    let mut monde = Monde::new(nombre_initial, taille_initiale);
//    println!("{:#?}", monde);
    monde.affichage(true);

    for i in 0..iterations {
        println!();
        println!("=== > Itération {}", i);
        println!();

        monde.reproduction();
        monde.mort();
        monde.affichage(false);
    }

    println!();
    println!("=== > Monde final : ");
    println!();

    monde.affichage(true);
}
