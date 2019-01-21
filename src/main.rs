mod monde;
mod individu;

use monde::Monde;

fn main() {
    let mut f = Monde::new(2, 4);
//    println!("{:#?}", f);
    f.affichage(true);
    f.reproduction();
    f.affichage(true);
//    println!("{:#?}", f);
}
