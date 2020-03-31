use rand::Rng;
use std::mem;

type Gene = Vec<u8>;

#[derive(Clone, Debug)]
pub struct Indivisual {
    pub gene: Gene,
    pub evaluation: usize,
}

impl Indivisual {
    pub fn new(gene_length: usize) -> Self {
        let mut rng = rand::thread_rng();
        let gene: Gene = (0..gene_length).map(|_| rng.gen_range(0, 2)).collect();
        Self {
            gene,
            evaluation: 0,
        }
    }

    pub fn evaluate(&mut self) -> &mut Self {
        self.evaluation = self.gene.iter().sum::<u8>() as usize;
        self
    }

    pub fn build(&self) -> Self {
        Self {
            gene: self.gene.clone(),
            evaluation: self.evaluation,
        }
    }

    pub fn cross_over(parent1: &mut Indivisual, parent2: &mut Indivisual, gene_length: usize) {
        let mut rng = rand::thread_rng();
        let split_point1 = rng.gen_range(1, gene_length - 1);
        let split_point2 = rng.gen_range(split_point1 + 1, gene_length);

        for i in split_point1..=split_point2 {
            mem::swap(&mut parent1.gene[i], &mut parent2.gene[i]);
        }
        parent1.evaluate();
        parent2.evaluate();
    }

    pub fn mutate(&mut self, mutation_rate: f64) {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0_f64, 1_f64) < mutation_rate {
            let indice = rng.gen_range(0, self.gene.len());
            self.gene[indice] ^= 1;
        }
    }
}
