#[derive(Debug)]
pub struct Individu {
    dna: Vec<i8>,
}

impl Individu {
    pub fn new(taille: i64) -> Individu {
        let mut individu = Individu {
            dna: Vec::new(),
        };

        for _i in 0..taille {
            individu.dna.push(if rand::random() { 1 } else { 0 });
        }

        return individu;
    }

    pub fn clone(individu: &Individu) -> Individu {
        Individu {
            dna: individu.dna.clone(),
        }
    }

    pub fn affichage(&self) {
        for char in &self.dna {
            print!("{}", char);
        }
        println!();
    }

    pub fn distance_to_optimal(&self) -> f64 {
        let mut distance = 0;
        let half_dna = self.dna.len() / 2.0 as usize;

        for i in 0..half_dna {
            if self.dna[i] != self.dna[self.dna.len() - i - 1] {
                distance += 1;
            }
        }

        return distance as f64 / half_dna as f64; // Devrait être entre 0 et 1
    }
}