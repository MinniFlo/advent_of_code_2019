use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::i32::MAX;

#[derive(Debug)]
enum Dir { Up, Right }

#[derive(Debug)]
struct Line {
    start: i32,
    end: i32,
    constant: i32,
    dir: Dir,
    end_coord: (i32, i32),
}

impl Line {
    fn new(begin: (i32, i32), dir_char: char, len: i32) -> Line {
        match dir_char {
            'U' => {
                let start = begin.1;
                let end = start + len;
                let constant = begin.0;
                let dir = Dir::Up;
                let end_coord: (i32, i32) = (constant, end);
                Line { start, end, constant, dir, end_coord }
            }
            'R' => {
                let start = begin.0;
                let end = start + len;
                let constant = begin.1;
                let dir = Dir::Right;
                let end_coord: (i32, i32) = (end, constant);
                Line { start, end, constant, dir, end_coord }
            }
            'D' => {
                let end = begin.1;
                let start = end - len;
                let constant = begin.0;
                let dir = Dir::Up;
                let end_coord: (i32, i32) = (constant, start);
                Line { start, end, constant, dir, end_coord }
            }
            'L' => {
                let end = begin.0;
                let start = end - len;
                let constant = begin.1;
                let dir = Dir::Right;
                let end_coord: (i32, i32) = (start, constant);
                Line { start, end, constant, dir, end_coord }
            }
            _ => panic!("convert function does not work")
        }
    }

    fn intersections_with(&self, other: &Line) -> Option<(i32, i32)> {
        if self.compare_dir(&other.dir) {
            None
        }else if (self.start..=self.end).contains(&other.constant) &&
                (other.start..=other.end).contains(&self.constant) {
                if self.compare_dir(&Dir::Right) {
                    Some((other.constant, self.constant))
                } else {
                    Some((self.constant, other.constant))
                }
        } else {
            None
        }
    }

    fn compare_dir(&self, dir: &Dir) -> bool {
        match (&self.dir, dir) {
            (Dir::Right, Dir::Right) => true,
            (Dir::Up, Dir::Up) => true,
            _ => false
        }
    }

    fn contains_tup(&self, tup: (i32, i32)) -> bool {
        match self.dir {
            Dir::Up => tup.0 == self.constant && (self.start..=self.end).contains(&tup.1),

            Dir::Right => tup.1 == self.constant && (self.start..=self.end).contains(&tup.0)
        }
    }

    fn distants_in_line(&self, tup: (i32, i32)) -> i32 {
        match self.dir {
            Dir::Up => {
                if self.end == self.end_coord.1 {
                    tup.1 - self.start
                } else {
                    self.end - tup.1
                }
            }
            Dir::Right => {
                if self.end == self.end_coord.0 {
                    tup.0 - self.start
                } else {
                    self.end - tup.0
                }
            }
        }
    }
}

fn convert_to_vec(s_o: &mut String) -> Vec<&str> {
    s_o.split(',')
        .map(|s| s.trim())
        .collect()
}

fn convert_to_line(vec: Vec<&str>) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut origin: (i32, i32) = (0, 0);
    for i in vec.iter() {
        let dir_char: char = i[0..1].parse().unwrap();
        let len: i32 = i[1..].parse().unwrap();
        let line: Line = Line::new(origin, dir_char, len);
        origin = line.end_coord;
        lines.push(line);
    }
    lines
}

fn _find_short_manhatten(vec1: Vec<&str>, vec2: Vec<&str>) -> i32 {
    let line1 = convert_to_line(vec1);
    let line2 = convert_to_line(vec2);
    let mut intersections: HashSet<(i32, i32)> = HashSet::new();
    for i in line1.iter() {
        for j in line2.iter() {
            if let Some(tup) = i.intersections_with(j) {
                if tup != (0, 0) {
                    intersections.insert(tup);
                }
            }
        }
    }

    let mut nearest = MAX;
    for (x, y) in intersections.iter() {
        let value = x.abs() + y.abs();
        if value < nearest {
            nearest = value
        }
    }
    nearest
}

fn length_to_tup(line: &Vec<Line>, intersection: &(i32, i32)) -> i32 {
   let mut sum = 0;
   for i in line.iter() {
       if i.contains_tup(*intersection) {
           sum += i.distants_in_line(*intersection);
           return sum
       }
       sum += (i.start - i.end).abs();
   }
   panic!("intersection is not an intersection");
}

fn find_short_delay(vec1: Vec<&str>, vec2: Vec<&str>) -> i32 {
   let line1 = convert_to_line(vec1);
   let line2 = convert_to_line(vec2);
   let mut intersections: HashSet<(i32, i32)> = HashSet::new();
   for i in line1.iter() {
       for j in line2.iter() {
           if let Some(tup) = i.intersections_with(j) {
               if tup != (0, 0) {
                   intersections.insert(tup);
               }
           }
       }
   }
   let mut length_sum = MAX;
   for tup in intersections.iter() {
       let line1_sum = length_to_tup(&line1, tup);
       let line2_sum = length_to_tup(&line2, tup);
       if line1_sum + line2_sum < length_sum {
           length_sum = line1_sum + line2_sum;
       }
   }
   length_sum
}

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let mut str_1 = String::new();
    let mut str_2 = String::new();
    let mut reader = BufReader::new(file);
    reader.read_line(&mut str_1).unwrap();
    reader.read_line(&mut str_2).unwrap();
    // let mut str_1 = String::from("R8,U5,L5,D3");
    // let mut str_2 = String::from("U7,R6,D4,L4");
    let vec1 = convert_to_vec(&mut str_1);
    let vec2 = convert_to_vec(&mut str_2);
    print!("{}", find_short_delay(vec1, vec2));
    Ok(())
}

