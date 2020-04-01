use rand::seq::IteratorRandom;
use std::fmt;
use std::time::Instant;

use crate::indivisual::Indivisual;
use crate::{Parameters, SimulationResult};


#[derive(Clone, Debug)]
pub struct Simulator {
    population: Vec<Indivisual>,
    current_generation: usize,
    gene_length: usize,
    mutation_rate: f64,
    iteration_count: usize,
}

impl Simulator {
    pub fn new(params: &Parameters) -> Self {
        let mut population: Vec<Indivisual> = (0..params.population_size)
            .map(|_| Indivisual::new(params.gene_length).evaluate().build())
            .collect();
        Self::sort_by_fitness(&mut population);
        Self {
            population,
            current_generation: 1,
            gene_length: params.gene_length,
            mutation_rate: params.mutation_rate,
            iteration_count: params.iteration_count,
        }
    }

    pub fn run(&mut self) -> SimulationResult {
        Self::sort_by_fitness(&mut self.population);
        let start_time = Instant::now();
        for _ in 0..self.iteration_count {
            self.proceed_generation();
            self.current_generation += 1;
        }
        let duration = start_time.elapsed().as_millis();
        SimulationResult {
            fitness: self.population[0].fitness,
            duration,
            mutation_rate: self.mutation_rate,
            iteration_count: self.iteration_count,
            population_size: self.population.len(),
        }
    }

    fn proceed_generation(&mut self) {
        let mut offspring: Vec<Indivisual> = Vec::with_capacity(self.population.len());
        for _ in 0..self.gene_length / 2 {
            let mut parent_x = self.select();
            let mut parent_y = self.select();
            Indivisual::cross_over(&mut parent_x, &mut parent_y, self.gene_length);
            parent_x.mutate(self.mutation_rate);
            parent_y.mutate(self.mutation_rate);
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
