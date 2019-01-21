use crate::individu::Individu;

#[derive(Debug)]
pub struct Monde {
    nombre_reproduction: i64,
    temps_meilleur: i64,
    nombre_mutations_meilleur: i64,
    individus: Vec<Individu>,
}

impl Monde {
    /*

    PUBLIC

    */

    pub fn new(nb_initial: i64, taille_initiale: i64) -> Monde {
        let mut monde = Monde {
            nombre_reproduction: 0,
            temps_meilleur: -1,
            nombre_mutations_meilleur: -1,
            individus: Vec::new(),
        };

        for _i in 0..nb_initial {
            monde.individus.push(Individu::new(taille_initiale));
        }

        return monde;
    }

    pub fn reproduction(&mut self) {
        self.nombre_reproduction += 1;
        let individu = self.selection_reproduction();
        self.individus.push(Individu::clone(&individu));
    }

    pub fn affichage(&self, verbose: bool) {
        let indice_meilleur = self.meilleur_individu();
        let meilleur = self.individus[indice_meilleur as usize].distance_to_optimal();
        println!("Meilleure distance à l'optimum : {}", meilleur);
        println!("Distance moyenne à l'optimum : {}", self.distance_moyenne());
        println!("{} individus : ", self.individus.len());

        if verbose {
            for individu in &self.individus {
                individu.affichage();
            }
        }
    }


    /*

    PRIVATE

    */

    fn selection_reproduction(&self) -> &Individu {
        &self.individus[self.meilleur_individu() as usize]
    }

    fn meilleur_individu(&self) -> i64 {
        if !self.individus.is_empty() {
            let mut distance = self.individus[0].distance_to_optimal();
            let mut individu = 0 as i64;

            for i in 0..self.individus.len() {
                let temp = self.individus[i].distance_to_optimal();
                if temp < distance {
                    distance = temp;
                    individu = i as i64;
                }
            }

            return individu;
        }

        return -1;
    }

    fn distance_moyenne(&self) -> f64 {
        if !self.individus.is_empty() {
            let mut distance = 0.0;
            let len = self.individus.len() as f64;


            for individu in &self.individus {
                distance += individu.distance_to_optimal() / len;
            }

            return distance;
        }

        return 0.0;
    }
}