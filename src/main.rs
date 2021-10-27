use std::vec::Vec;
use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};
use std::{thread, time};
use rand::prelude::*;

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

struct Map<T> {
    tiles: Vec<T>,
    width: usize,
    height: usize
}

impl<T> Map<T> {
    fn new(tiles: Vec<T>, width: usize, height: usize) -> Map<T> {
        Map {
            tiles: tiles,
            width: width,
            height: height
        }
    }

    fn get_random(&self) -> &T {
        let index = rand::thread_rng().gen_range(0..self.tiles.len());
        &self.tiles[index]
    }

    fn in_bounds(&self, p: Position) -> bool {
        p.x < self.width && p.y < self.height
    }

    fn mut_element_at(&mut self, p: Position) -> &mut T {
        &mut self.tiles[p.y * self.width + p.x]
    }

    fn element_at(&self, p: Position) -> Option<&T> {
        if self.in_bounds(p) {
            Some(&self.tiles[p.y * self.width + p.x])
        } else {
            None
        }
    }

}

impl Map<char> {
    fn print_map_by_line(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.tiles[y * self.width + x]);
            }
            println!("");
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)] 
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    /*
     * Returns the distance between this point and another
     */
    fn distance(&self, other: &Position) -> usize {
        let larger_x = self.x.max(other.x);
        let larger_y = self.y.max(other.y);
        let smaller_x = self.x.min(other.x);
        let smaller_y = self.y.min(other.y);
        let x_diff = larger_x - smaller_x;
        let y_diff = larger_y - smaller_y;
        ((x_diff * x_diff + y_diff * y_diff) as f64).sqrt() as usize
    }

    /*
     * Returns the neighbors of this position
     */
    fn neighbors(&self) -> Vec<Position> {
        let mut neighbors = Vec::new();
        neighbors.push(Position::new(self.x + 1, self.y));
        if self.x > 0 { neighbors.push(Position::new(self.x - 1, self.y)); }
        neighbors.push(Position::new(self.x, self.y + 1));
        if self.y > 0 { neighbors.push(Position::new(self.x, self.y - 1)); }
        return neighbors;
    }

    fn reconstruct_path(came_from: HashMap<Position, Position>, current: Position) -> Vec<Position> {
        let mut path = Vec::new();
        path.push(current);
        let mut current = current;
        while came_from.contains_key(&current) {
            if current == came_from[&current] {
                break;
            }
            current = came_from[&current];
            path.push(current);
        }
        return path;
    }

    /*
    * Sorts a vector of points by their distance to a given point
    */
    fn sort_by_distance(&self, points: &mut Vec<Position>) {
        points.sort_by(|a, b| {
            let dist_a = a.distance(self);
            let dist_b = b.distance(self);
            dist_a.cmp(&dist_b)
        });
        points.reverse();
    }

    /*
    * Returns the path between two positions of a map
    */
    fn path_between(&self, pos: Position, map: &Map<char>) -> Vec<Position> {
        let mut open_set = Vec::new();
        let mut closed_set = HashSet::new();
        let mut came_from = HashMap::new();

        open_set.push(*self);

        while !open_set.is_empty() {
            pos.sort_by_distance(&mut open_set);
            let current = open_set.pop().unwrap();
            closed_set.insert(current);

            if current == pos {
                return Position::reconstruct_path(came_from, current);
            }

            let neighbors = current.neighbors();
            for neighbor in neighbors {
                if !closed_set.contains(&neighbor) 
                    && map.in_bounds(neighbor) 
                    && map.element_at(neighbor).unwrap() == &'.' 
                {
                    open_set.push(neighbor);
                    came_from.insert(neighbor, current);
                }
            }
        }

        return closed_set.into_iter().collect();
    }

}

fn gen_random_char() -> char {
     if rand::thread_rng().gen_range(0..5) != 4 { '.' } else { '$' }
}

fn gen_map(width: usize, height: usize) -> Vec<char> {
    let mut map = Vec::new();
    for _ in 0..width * height {
        map.push(gen_random_char());
    }
    return map;
}

fn main_once() {
    let mut map = Map::new(gen_map(18, 9), 18, 9);
    let pos1 = Position::new(0, 0);
    let pos2 = Position::new(
        rand::thread_rng().gen_range(0..map.width),
        rand::thread_rng().gen_range(0..map.height));
    
    *map.mut_element_at(pos2) = '.';
    let vec_path = pos1.path_between(pos2, &map);
    //println!("Map output: {:?}", vecPath);
    for pos in vec_path {
        *(map.mut_element_at(pos)) = '#';
    }
    map.print_map_by_line();
    thread::sleep(time::Duration::from_millis(5000));
    clear_screen();
}

fn main() {

    clear_screen();
    //Generate a 9x9 vector of '.'
    for _ in 0..100 {
        main_once();
    }
}


