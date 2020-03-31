use std::fmt;

mod indivisual;
use indivisual::Indivisual;

fn main() {
    let mut simulator = Simulator::new(10, 20, 0_f64);
    simulator.run(10);
}

#[derive(Clone, Debug)]
struct Simulator {
    gene_length: usize,
    current_generation: usize,
    mutate_rate: f64,
    indivisuals: Vec<Indivisual>,
}

impl Simulator {
    fn new(num_indivisuals: usize, gene_length: usize, mutate_rate: f64) -> Self {
        let mut indivisuals: Vec<Indivisual> = (0..num_indivisuals)
            .map(|_| Indivisual::new(gene_length).evaluate().build())
            .collect();
        Self::sort_by_evaluation(&mut indivisuals);
        Self {
            gene_length,
            current_generation: 1,
            mutate_rate,
            indivisuals,
        }
    }

    fn run(&mut self, iteration_count: usize) {
        Self::sort_by_evaluation(&mut self.indivisuals);
        println!("{}", self);
        for _ in 0..iteration_count {
            self.proceed_generation();
            self.current_generation += 1;
            println!("{}", self);
        }
    }

    fn proceed_generation(&mut self) {
        let mut offspring: Vec<Indivisual> = Vec::with_capacity(self.indivisuals.len());
        for _ in 0..self.indivisuals.len() / 2 {
            let (mut parent1, mut parent2) = self.select();
            Indivisual::cross_over(&mut parent1, &mut parent2, self.gene_length);
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
