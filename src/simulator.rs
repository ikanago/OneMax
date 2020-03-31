use rand::seq::IteratorRandom;
use std::fmt;

use crate::indivisual::Indivisual;

#[derive(Clone, Debug)]
pub struct Simulator {
    gene_length: usize,
    current_generation: usize,
    mutate_rate: f64,
    population: Vec<Indivisual>,
}

impl Simulator {
    pub fn new(num_indivisuals: usize, gene_length: usize, mutate_rate: f64) -> Self {
        let mut population: Vec<Indivisual> = (0..num_indivisuals)
            .map(|_| Indivisual::new(gene_length).evaluate().build())
            .collect();
        Self::sort_by_fitness(&mut population);
        Self {
            gene_length,
            current_generation: 1,
            mutate_rate,
            population,
        }
    }

    pub fn run(&mut self, iteration_count: usize, is_verbose: bool) {
        Self::sort_by_fitness(&mut self.population);
        println!("{}", self);
        for _ in 0..iteration_count {
            self.proceed_generation();
            self.current_generation += 1;
            if is_verbose {
                println!("{}", self);
            }
        }
        println!("---------------------Result---------------------------");
        println!("Best fitness: {}, Duration: {}", self.population[0].fitness, 0);
    }

    fn proceed_generation(&mut self) {
        let mut offspring: Vec<Indivisual> = Vec::with_capacity(self.population.len());
        for _ in 0..self.gene_length / 2 {
            let mut parent_x = self.select();
            let mut parent_y = self.select();
            Indivisual::cross_over(&mut parent_x, &mut parent_y, self.gene_length);
            parent_x.mutate(self.mutate_rate);
            parent_y.mutate(self.mutate_rate);
            offspring.push(parent_x.clone());
            offspring.push(parent_y.clone());
        }
        self.population = offspring;
        Self::sort_by_fitness(&mut self.population);
    }

    fn select(&mut self) -> Indivisual {
        let candidate_size: usize = 4;
        let mut rng = rand::thread_rng();
        let candidate: Vec<Indivisual> = (0..candidate_size)
            .map(|_| self.population.iter().choose(&mut rng).unwrap().clone())
            .collect();
        candidate
            .iter()
            .fold(Default::default(), |maximum, indivisual| {
                std::cmp::max(maximum, indivisual.clone())
            })
    }

    fn sort_by_fitness(indivisuals: &mut Vec<Indivisual>) {
        indivisuals.sort_by(|x, y| x.cmp(&y));
    }
}

impl fmt::Display for Simulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! {f, "--------------------Generation {}--------------------\nBest fitness: {}"
        , self.current_generation, self.population[0].fitness}
    }
}
