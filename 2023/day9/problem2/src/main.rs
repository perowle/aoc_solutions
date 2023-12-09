use std::fs;

fn predict_next(vals:Vec<i64>) -> i64 {
    let mut matrix: Vec<i64> = Vec::new();
    let mut sub: i64 = 0;
    let mut ret:i64 = 0;
    let mut all_zeros = true;
    

    // Make the next matrix
    for idx in 1..vals.len() {
        sub = vals[idx] - vals[idx-1];
        matrix.push(sub);
        if sub != 0 {
            all_zeros = false;
        }
    }

    println!("{:?}", matrix);
    sub = vals[1] - vals[0];
    // Recurse until it's 0
    if !all_zeros {
        ret = predict_next(matrix);
        ret = sub - ret;
    }

    println!("Returning {}", ret);
    return ret;
}
    

fn main() {
    
    let contents = fs::read_to_string("..\\data\\day9.txt").unwrap();
    let lines:Vec<_> = contents.split('\n').
    collect();
    let mut total: i64 = 0;

    for line in lines {
        let vals:Vec<i64> = line.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        println!("{:?}", vals);
        let first_val = vals[0];
        let val = first_val - predict_next(vals);
        total = total + val;
        println!("Next value is {}", val);
    }

    println!("Total: {}", total);

}
