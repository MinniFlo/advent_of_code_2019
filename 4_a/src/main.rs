use std::u32::MAX;


fn _digits_rise_hard(num: u32) -> bool {
    let d1 = (num / 100_000) as u32;
    let d2 = ((num % 100_000) / 10_000) as u32;
    let d3 = ((num % 10_000) / 1000) as u32;
    let d4 = ((num % 1000) / 100) as u32;
    let d5 = ((num % 100) / 10) as u32;
    let d6 = (num % 10) as u32;

    (d1 <= d2 && d2 <= d3 && d3 <= d4 && d4 <= d5 && d5 <= d6) &&
    (d1 == d2 || d2 == d3 || d3 == d4 || d4 == d5 || d5 == d6)
}

fn digits_rise(num: u32) -> bool {
    let mut last = MAX;
    let mut num = num;
    while num > 0 {
        let current = num % 10;
        if current <= last {
            num = (num / 10) as u32;
            last = current;
        } else {
            return false
        }
    }
    true
}

fn multiple_digits(num: u32) -> bool {
    let mut last = MAX;
    let mut num = num;
    while num > 0 {
        let current = num % 10;
        if current == last {
            return true
        }
        num = (num / 10) as u32;  
        last = current
    }
    false
}

fn double_digits(num: u32) -> bool {
    let mut last = MAX;
    let mut num = num;
    let mut combo = MAX;
    let mut double = false;
    while num > 0 {
        let current = num % 10;
        if combo != current {
            if current == last {
                if double {
                    double = false;
                    combo = current;
                    last = current;
                } else {
                    double = true;
                    last = current;
                }
            } else {
                if double {
                    return true
                } else {
                    last = current;
                }
            }
        }
        num = (num / 10) as u32;  
    }
    if double {
        return true
    }
    false
}

fn possible_passwords(start: u32, end: u32) -> usize {    
    let result_vec: Vec<u32> = 
        (start .. end)
        .filter(|num| digits_rise(*num) && double_digits(*num))
        .collect();
    result_vec.len()
}

fn main() {
    let start = 130_254;
    let end = 678_275;
    print!("{}", possible_passwords(start, end));
    print!("{}, {}, {}", double_digits(110303), double_digits(124444), double_digits(123444))
}

