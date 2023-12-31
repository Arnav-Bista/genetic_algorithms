use rand::thread_rng;
use rand::prelude::*;

#[derive(Clone)]
pub struct Candidate {
    pub chromozones: Vec<usize>,
    fitness: f32
}

impl Candidate {

    pub fn new(cities: &Vec<(u32,u32)>) -> Self {
        let mut chromozones: Vec<usize> = (0..cities.len()).collect();
        chromozones.shuffle(&mut thread_rng());
        let mut candidate = Self {
            chromozones,
            fitness: 0.0
        };
        candidate.calcualte_fitness(cities);
        candidate
    }

    pub fn empty() -> Self {
        Self {
            chromozones: Vec::new(),
            fitness: 0.0
        }
    }

    pub fn fitness(&self) -> f32 {
        self.fitness
    }

    pub fn calcualte_fitness(&mut self, cities: &Vec<(u32,u32)>) -> f32 {
        self.fitness = 0.0;
        let mut prev: (u32,u32) = (0,0);
        let mut prev_updated = false;
        for index in &self.chromozones {
            if !prev_updated {
                prev = cities[*index];
                prev_updated = true;
            }
            else {
                self.fitness += f32::sqrt(
                    f32::powi(cities[*index].0 as f32 - prev.0 as f32, 2) + 
                    f32::powi(cities[*index].1 as f32 - prev.1 as f32, 2));
                prev = cities[*index];
            }
        }

        self.fitness += f32::sqrt(
            f32::powi(cities[self.chromozones[0]].0 as f32 - cities[self.chromozones[self.chromozones.len() - 1]].0 as f32, 2) + 
            f32::powi(cities[self.chromozones[0]].1 as f32 - cities[self.chromozones[self.chromozones.len() - 1]].1 as f32, 2));
        self.fitness = 1.0 / self.fitness * 1000.0;
        self.fitness
    }
}
