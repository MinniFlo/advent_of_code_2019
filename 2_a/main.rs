use std::fs::File;
use std::io::Read;

fn convert_to_vec(s: &mut String) -> Vec<i32> {
    s.split(',')
        .map(|str| str.trim())
        .filter(|str| !str.is_empty())
        .map(|str| str.parse().unwrap())
        .collect()
}



fn process_intcode(vec: &mut Vec<i32>, noun: i32, verb: i32) {
    vec[1] = noun;
    vec[2] = verb;
    for i in (0..vec.len()).filter(|x| x % 4 == 0) {
        match vec[i] {
            1 => {
                let write_index: usize = vec[i + 3] as usize;
                vec[write_index] = vec[vec[i + 1] as usize] + vec[vec[i + 2] as usize]
                },
            2 => {
                let write_index: usize = vec[i + 3] as usize;
                vec[write_index] = vec[vec[i + 1] as usize] * vec[vec[i + 2] as usize]
                },
            99 => break,
            _ => panic!("falsche schrittweite")
        }
    }
}

fn find_noun_verb(vec: &mut Vec<i32>) -> (i32, i32) {
    let mut vec = vec;
    let vec_org = vec.clone();
    for i in 0 .. 100 {
        for j in 0 .. 100 {
            process_intcode(&mut vec, i as i32, j as i32);
            if vec[0] == 19690720 {
               return (i, j);
            }
            *vec = vec_org.clone();
        }
    }
    (0, 0)
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut icode = convert_to_vec(&mut s);
    let n_v_tup = find_noun_verb(&mut icode);
    print!("{:?}", n_v_tup);
    Ok(())
}
