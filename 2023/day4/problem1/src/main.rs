
use std::fs;




fn main() {

    let contents = fs::read_to_string("day4.txt").unwrap();

    println!("{}", contents.len());
    let lines:Vec<_> = contents.split('\n').collect();
    let mut total_score = 0;

    for line in lines {
        println!("{}", line);
        let line_parse: Vec<_> = line.trim().split(':').collect();
        let temp_str = line_parse[1].replace("  "," ");
        let nums:Vec<_> = temp_str.split("|").collect();

        println!("{}", nums[0]);
        println!("{}", nums[1]);

        let winners:Vec<_> = nums[0].trim().split(" ").collect();
        let my_nums:Vec<_> = nums[1].trim().split(" ").collect();

        let mut score = 0;
        for i in 0..my_nums.len() {
            for j in 0..winners.len() {
                if my_nums[i] == winners[j] {
                    println!("Match:{}", my_nums[i]);
                    if score == 0 {
                        score = 1;
                    }
                    else {
                        score = score * 2;
                    }    
                }
            }

        }
        println!("Line score {}", score);
        total_score = total_score + score;
    }

    println!("Total score {}", total_score);

}
