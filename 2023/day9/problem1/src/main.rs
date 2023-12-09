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

    // Recurse until it's 0
    if !all_zeros {
        ret = predict_next(matrix);
        ret = ret + sub;
    }

//    println!("Returning {}", ret);
    return ret;
}
    

fn main() {
    
    let contents = fs::read_to_string("..\\data\\day9.txt").unwrap();
    let lines:Vec<_> = contents.split('\n').collect();
    let mut total: i64 = 0;

    for line in lines {
        let vals:Vec<i64> = line.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        println!("{:?}", vals);
        let last_val = vals[vals.len()-1];
        let val = predict_next(vals) + last_val;
        total = total + val;
        println!("");
 //       println!("Next value is {}", val);
    }

    println!("Total: {}", total);

}
