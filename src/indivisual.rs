use rand::Rng;
use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use std::mem;

type Gene = Vec<f64>;

#[derive(Clone, Debug, Default)]
pub struct Indivisual {
    pub gene: Gene,
    pub fitness: f64,
}

impl Indivisual {
    pub fn new(gene_length: usize) -> Self {
        let mut rng = rand::thread_rng();
        let gene: Gene = (0..gene_length)
            .map(|_| rng.gen())
            .collect();
        Self {
            gene,
            fitness: 0f64,
        }
    }

    pub fn evaluate(&mut self) -> &mut Self {
        self.fitness = (self.gene.iter().sum::<f64>() / self.gene.len() as f64).into();
        self
    }

    pub fn build(&self) -> Self {
        Self {
            gene: self.gene.clone(),
            fitness: self.fitness,
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
            self.gene[indice] = rng.gen_range(self.gene[indice], 1f64);
        }
    }
}

impl Eq for Indivisual {}

impl Ord for Indivisual {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ordering) = self.fitness.partial_cmp(&other.fitness) {
            return ordering;
        }
        unimplemented!();
    }
}

impl PartialOrd<Indivisual> for Indivisual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.fitness.partial_cmp(&self.fitness)
    }
}

impl PartialEq<Indivisual> for Indivisual {
    fn eq(&self, other: &Self) -> bool {
        if let Some(ordering) = self.fitness.partial_cmp(&other.fitness) {
            return ordering == Ordering::Equal;
        }
        unimplemented!();
    }
}
