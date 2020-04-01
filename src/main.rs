#[macro_use]
extern crate clap;
extern crate onemax;

use onemax::simulator::Simulator;
use onemax::{Parameters, SimulationResult};

fn main() {
    let matches = clap_app!(onemax =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg population_size: -n --("Pupulation-size") +takes_value "Number of indivisuals (20)")
        (@arg gene_length: -l --("gene-length") +takes_value "Length of gene (10)")
        (@arg mutation_rate: -m --("mutation-rate") +takes_value "Probability that mutation occurs (0.3)")
        (@arg iterations: -i --iteration +takes_value "Number of generations (20)")
        (@arg is_verbose: -v --verbose "Specify if output is verbose(false)")
        (@group analyze =>
            (@arg analyze_mutation: --am ... #{0,3} +takes_value "Specify a range of mutation rate(0, 1, 0.1)")
            (@arg analyze_iteration: --ai ... #{0,3} +takes_value "Specify a range of iteration(10, 100, 10)")
            (@arg analyze_population: --ap ... #{0,3} +takes_value "Specify a range of population size(10, 100, 10)")
        )
    )
    .get_matches();

    let mut params = Parameters {
        population_size: value_t!(matches, "population_size", usize).unwrap_or(20),
        gene_length: value_t!(matches, "gene_length", usize).unwrap_or(10),
        mutation_rate: value_t!(matches, "mutation_rate", f64).unwrap_or(0.3),
        iteration_count: value_t!(matches, "iterations", usize).unwrap_or(20),
    };
    let is_verbose = matches.is_present("is_verbose");

    if matches.is_present("analyze_mutation") {
        let range = values_t!(matches, "analyze_mutation", f64).unwrap_or(vec![0f64, 1f64, 0.1]);
        analyze_mutaion(&mut params, range[0], range[1], range[2], is_verbose);
        return;
    }
    if matches.is_present("analyze_iteration") {
        let range = values_t!(matches, "analyze_iteration", usize).unwrap_or(vec![10, 100, 10]);
        analyze_iteration(&mut params, range[0], range[1], range[2], is_verbose);
        return;
    }
    if matches.is_present("analyze_population") {
        let range = values_t!(matches, "analyze_population", usize).unwrap_or(vec![10, 100, 10]);
        analyze_population(&mut params, range[0], range[1], range[2], is_verbose);
        return;
    }

    let result = simulate(&params);
    println!("{}", result.to_string(is_verbose));
}

fn simulate(params: &Parameters) -> SimulationResult {
    let mut simulator = Simulator::new(params);
    simulator.run()
}

fn analyze_mutaion(params: &mut Parameters, start: f64, end: f64, offset: f64, is_verbose: bool) {
    if start > end {
        panic!("Minimum mutation rate must be smaller than maximum mutation rate.");
    }
    let mut current_mutation_rate = start;
    while current_mutation_rate <= end {
        params.mutation_rate = current_mutation_rate;
        let result = simulate(params);
        println!("{}", result.to_string(is_verbose));
        current_mutation_rate += offset;
    }
}

fn analyze_iteration(
    params: &mut Parameters,
    start: usize,
    end: usize,
    offset: usize,
    is_verbose: bool,
) {
    if start > end {
        panic!("Minimum iteration count must be smaller than maximum iteration count.");
    }
    let mut current_iteration_count = start;
    while current_iteration_count <= end {
        params.iteration_count = current_iteration_count;
        let result = simulate(params);
        println!("{}", result.to_string(is_verbose));
        current_iteration_count += offset;
    }
}

fn analyze_population(
    params: &mut Parameters,
    start: usize,
    end: usize,
    offset: usize,
    is_verbose: bool,
) {
    if start > end {
        panic!("Minimum populaton size must be smaller than maximum population size.");
    }
    let mut current_population_size = start;
    while current_population_size <= end {
        params.population_size = current_population_size;
        let result = simulate(params);
        println!("{}", result.to_string(is_verbose));
        current_population_size += offset;
    }
}
