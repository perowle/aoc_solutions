
use std::fs;




fn main() {

    let contents = fs::read_to_string("day4.txt").unwrap();
    let mut card_scores:[i32;1000] = [0;1000];
    let mut card_instance:[i32;1000] = [1;1000];
    let mut idx = 0;

    let lines:Vec<_> = contents.split('\n').collect();
    let sz = lines.len();
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
                    score = score + 1;
                }
            }

        }
        card_scores[idx] = score;
        for i in 0..score as usize {
            for _j in 0..card_instance[idx] {
                card_instance[idx+i+1] = card_instance[idx+i+1] + 1;
            }
        }
        idx = idx + 1;
    }

    let mut total_instance = 0;
    for i in 0..sz {
        println!("Card score[{}] {} {}", i, card_scores[i], card_instance[i]);
        total_instance = total_instance + card_instance[i];
    }

    println!("Total score {}", total_instance);

}
