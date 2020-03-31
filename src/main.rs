use std::fmt;
use rand::Rng;

mod indivisual;
use indivisual::Indivisual;

fn main() {
    let mut rng = rand::thread_rng();
    let mut simulator = Simulator::new(10, 100, 0.3, &mut rng);
    simulator.run(20, &mut rng);
}

#[derive(Clone, Debug)]
struct Simulator {
    gene_length: usize,
    current_generation: usize,
    mutate_rate: f64,
    indivisuals: Vec<Indivisual>,
}

impl Simulator {
    fn new<R: Rng>(num_indivisuals: usize, gene_length: usize, mutate_rate: f64, rng: &mut R) -> Self {
        let mut indivisuals: Vec<Indivisual> = (0..num_indivisuals)
            .map(|_| Indivisual::new(gene_length, &mut rng).evaluate().build())
            .collect();
        Self::sort_by_evaluation(&mut indivisuals);
        Self {
            gene_length,
            current_generation: 1,
            mutate_rate,
            indivisuals,
        }
    }

    fn run<R: Rng>(&mut self, iteration_count: usize, rng: &mut R) {
        Self::sort_by_evaluation(&mut self.indivisuals);
        println!("{}", self);
        for _ in 0..iteration_count {
            self.proceed_generation(&mut rng);
            self.current_generation += 1;
            println!("{}", self);
        }
    }

    fn proceed_generation<R: Rng>(&mut self, rng: &mut R) {
        let mut offspring: Vec<Indivisual> = Vec::with_capacity(self.indivisuals.len());
        for _ in 0..self.indivisuals.len() / 2 {
            let (mut parent1, mut parent2) = self.select();
            Indivisual::cross_over(&mut parent1, &mut parent2, self.gene_length, &mut rng);
            parent1.mutate(self.mutate_rate, &mut rng);
            parent2.mutate(self.mutate_rate, &mut rng);
            offspring.push(parent1);
            offspring.push(parent2);
        }
        self.indivisuals = offspring;
        Self::sort_by_evaluation(&mut self.indivisuals);
    }

    fn select(&mut self) -> (Indivisual, Indivisual) {
        (self.indivisuals[0].clone(), self.indivisuals[1].clone())
    }

    fn sort_by_evaluation(indivisuals: &mut Vec<Indivisual>) {
        indivisuals.sort_by(|x, y| (y.evaluation).cmp(&x.evaluation));
    }
}

impl fmt::Display for Simulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {f, "--------------------Generation {}--------------------\ngene_length: {}\nmutate_rate: {}\nmost elite indivisuals: {:?}"
        , self.current_generation, self.gene_length, self.mutate_rate, self.indivisuals[0]}
    }
}
