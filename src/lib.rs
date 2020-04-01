pub mod indivisual;
pub mod simulator;

#[derive(Clone, Debug)]
pub struct Parameters {
    pub population_size: usize,
    pub gene_length: usize,
    pub mutation_rate: f64,
    pub iteration_count: usize,
}

#[derive(Clone, Debug)]
pub struct SimulationResult {
    pub fitness: f64,
    pub duration: u128,
    pub mutation_rate: f64,
    pub iteration_count: usize,
    pub population_size: usize,
}

impl SimulationResult {
    pub fn to_string(&self, is_verbose: bool) -> String {
        if is_verbose {
            format!("mutation rate: {}\niteration count: {}\npopulation size: {}\nfitness: {}, duration: {}ms",
                self.mutation_rate, self.iteration_count, self.population_size, self.fitness, self.duration)
        } else {
            format!(
                "{},{},{},{}",
                self.mutation_rate, self.iteration_count, self.population_size, self.fitness
            )
        }
    }
}
