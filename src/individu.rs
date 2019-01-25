use crate::prob::Prob;

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

    pub fn mutation(&mut self) {
        if Prob::probability(10) {
            let index = rand::random::<usize>() % self.dna.len();
            self.dna.insert(index, if rand::random() { 1 } else { 0 });
        }

        if Prob::probability(10) {
            let index = rand::random::<usize>() % self.dna.len();
            self.dna.remove(index);
        }

        if Prob::probability(10) {
            let index = rand::random::<usize>() % self.dna.len();
            self.dna[index] = 1 - self.dna[index];
        }
    }
}