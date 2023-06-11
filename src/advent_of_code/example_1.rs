use std::{fs};

#[derive(Debug)]
struct Elf {
    calorie_list: Vec<u32>,
    calorie_count: u32,
}

impl Elf {
    fn new() -> Self {
        Self {
            calorie_list: Vec::new(),
            calorie_count: 0,
        }
    }

    fn add_calorie(&mut self, calorie: u32) {
        self.calorie_list.push(calorie);
        self.calorie_count += calorie;
    }
}

fn construct_elfs(contents: &str) -> Vec<Elf> {
    let mut elves: Vec<Elf> = Vec::new();
    let mut current_elf = Elf::new();

    for line in contents.lines(){
        let calories = line.parse::<u32>();

        match calories {
            Err(_) => {
                elves.push(current_elf);
                current_elf = Elf::new();
            }
            Ok(val) => {
                current_elf.add_calorie(val);
            }
        }
    }
    elves.push(current_elf);
    return elves;
}

pub fn run() {
    let contents = fs::read_to_string("./src/advent_of_code/inputs/input_1.txt").unwrap();
    let mut elves = construct_elfs(&contents);

    // sort the elves in descending order
    elves.sort_by( |a, b| b.calorie_count.cmp(& a.calorie_count) );

    let mut acc = 0;
    for i in 0..3{
        acc += elves[i].calorie_count;
    }

    print!("{}", acc);
}
