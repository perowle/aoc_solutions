
use std::fs;


fn main() {
    let contents = fs::read_to_string("..\\data\\day6.txt").unwrap();
    let lines:Vec<_> = contents.split('\n').collect();
    let t_times:Vec<_> = lines[0].trim().split(' ').collect();    
    let t_distances:Vec<_> = lines[1].trim().split(' ').collect();

    let mut times:Vec<u64> = Vec::new();
    let mut distances:Vec<u64> = Vec::new();
    for idx in 0..t_times.len() {
        times.push(t_times[idx].parse().unwrap());
        distances.push(t_distances[idx].parse().unwrap());
    }

    let mut win_sum = 1;
    for race_idx in 0..times.len() {
        let mut win_count = 0;
        for hold_time in 1..times[race_idx] {
            let distance: u64 = hold_time * (times[race_idx] - hold_time);
            if distance > distances[race_idx] {
                win_count += 1;
            }
        }
        win_sum *= win_count;
        println!("Race {} - win count {}", race_idx, win_count);
    }
    println!("Wins {}", win_sum);

}
    
