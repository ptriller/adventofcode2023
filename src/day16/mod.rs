use std::fs::read_to_string;
use std::path::Path;

use enumset::{EnumSet, EnumSetType};

use Direction::*;

#[derive(EnumSetType, Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

struct Data {
    width: usize,
    height: usize,
    field: Vec<Vec<u8>>,
    state: Vec<Vec<EnumSet<Direction>>>,
    rays: Vec<(usize, usize, Direction)>,
}


impl Data {
    fn init(path: &Path) -> Data {
        let field: Vec<Vec<u8>> = read_to_string(path).unwrap()
            .lines().map(|l| l.chars().map(|c| c as u8).collect()).collect();
        let (width, height) = (field[0].len(), field.len());
        Data {
            width,
            height,
            field: field,
            rays: vec![],
            state: vec![vec![EnumSet::new(); width]; height],
        }
    }

    fn reset(&mut self) {
        self.rays = vec![];
        self.state = vec![vec![EnumSet::new(); self.width]; self.height];
    }


    fn find_max_energy(&mut self) -> u32 {
        let mut result = 0;
        for col in 0..self.width {
            self.reset();
            self.rays.push((0, col, DOWN));
            result = result.max(self.calc_energy());
            self.reset();
            self.rays.push((self.height - 1, col, UP));
            result = result.max(self.calc_energy())
        }
        for row in 0..self.height {
            self.reset();
            self.rays.push((row, 0, RIGHT));
            result = result.max(self.calc_energy());
            self.reset();
            self.rays.push((row, self.width - 1, LEFT));
            result = result.max(self.calc_energy())
        }
        result
    }
    fn calc_energy(&mut self) -> u32 {
        while let Some((x, y, dir)) = self.rays.pop() {
            if self.state[x][y].insert(dir) {
                match (self.field[x][y], dir) {
                    (b'.', _) | (b'|', DOWN) | (b'|', UP)
                    | (b'-', RIGHT) | (b'-', LEFT) => self.move_ray(x, y, dir),
                    (b'\\', UP) => self.move_ray(x, y, LEFT),
                    (b'\\', RIGHT) => self.move_ray(x, y, DOWN),
                    (b'\\', DOWN) => self.move_ray(x, y, RIGHT),
                    (b'\\', LEFT) => self.move_ray(x, y, UP),
                    (b'/', UP) => self.move_ray(x, y, RIGHT),
                    (b'/', RIGHT) => self.move_ray(x, y, UP),
                    (b'/', DOWN) => self.move_ray(x, y, LEFT),
                    (b'/', LEFT) => self.move_ray(x, y, DOWN),
                    (b'|', RIGHT) | (b'|', LEFT) => {
                        self.move_ray(x, y, DOWN);
                        self.move_ray(x, y, UP);
                    }
                    (b'-', UP) | (b'-', DOWN) => {
                        self.move_ray(x, y, LEFT);
                        self.move_ray(x, y, RIGHT);
                    }
                    _ => panic!("Argh")
                }
            }
        }
        self.state.iter().map(|l|
            l.iter().filter(|s| !s.is_empty()).count() as u32).sum()
    }

    fn move_ray(&mut self, x: usize, y: usize, dir: Direction) {
        match (x, y, dir) {
            (x, y, UP) if x > 0 => self.rays.push((x - 1, y, UP)),
            (x, y, LEFT) if y > 0 => self.rays.push((x, y - 1, LEFT)),
            (x, y, DOWN)  if x + 1 < self.field.len() =>
                self.rays.push((x + 1, y, DOWN)),
            (x, y, RIGHT) if y + 1 < self.field[0].len() =>
                self.rays.push((x, y + 1, RIGHT)),
            _ => {}
        }
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::day16::Data;
    use crate::day16::Direction::RIGHT;

    #[test]
    fn do_problem1() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day16/input.txt");
        let mut data = Data::init(test_data.as_path());
        data.rays.push((0, 0, RIGHT));
        println!("Day 16, Problem 1: Fields: {}", data.calc_energy());
    }

    #[test]
    fn do_problem2() {
        let mut test_data = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data.push("resources/day16/input.txt");
        let mut data = Data::init(test_data.as_path());
        println!("Day 16, Problem 1: Fields: {}", data.find_max_energy());
    }
}