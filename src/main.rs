use rand::prelude::*;


const LENGTH: usize = 4;
const SIZE: i32 = 30;


fn main() {
   let numbers = get_numbers(SIZE);
   println!("{:?}", numbers);
   let mut solution = [0; LENGTH];
   let mut prevfitness = 1.0;
   let mut fitness = 1.0;
   let mut step: usize = 0;
   while fitness != 0.0{
        if(fitness < prevfitness){
            step +=1;
        }
        solution = get_solution(fitness, prevfitness, step, solution);
        prevfitness = fitness;
        fitness = evaluate_solution(solution, numbers);
        

        
        

   }
}


fn get_numbers(size: i32) -> [i32; LENGTH]{
    let mut rng = rand::thread_rng();
    let mut arr = [0; LENGTH];
    for x in 0..LENGTH{
        arr[x] = rng.gen_range(0..size);
    }
    return arr;
}

fn get_solution(fitness: f32, prevfitness: f32,mut step: usize,mut solution: [i32; LENGTH]) -> [i32; LENGTH]{
    
    solution[step] += 1;
    return solution;

}

fn evaluate_solution(solution: [i32; LENGTH], numbers: [i32; LENGTH]) -> f32 {
    let mut fitness = 1.0;
    for x in 0..LENGTH{
        if solution[x] == numbers[x]{
            let xmp = (1.0/LENGTH as f32) as f32;
            fitness -= xmp;
        }
    }
    return fitness;
}