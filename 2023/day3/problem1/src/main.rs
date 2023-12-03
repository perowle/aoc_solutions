
use std::fs;


fn is_num(c:u8)->bool {
    if c >= 48  && c <= 57  {
        return true;
    }
    else {
        return false;
    }
}

fn is_symbol(c:u8)->bool {
    if c != 46  && is_num(c) == false {
        return true;
    }
    else {
        return false;
    }
}



fn check_neighbor(check:&Vec<u8>, i:usize)->bool {
    let mut res:bool = false;
    let mut start:usize = 0;
    let mut end:usize = 0;

    if check.len() == 0
    {
        return false;
    }

    if i == 0 {
        start = 0;
        end = 1;
    }
    else if i == check.len()-1 {
        start = i-1;
        end = i;
    }
    else {
        start = i-1;
        end = i+1;
    }

    for j in start..=end {
        if is_symbol(check[j]) {
            res = true;
        }
    }

    return res;
}



fn main() {

    let contents = fs::read_to_string("day3.txt").unwrap();

    println!("{}", contents.len());
    let lines:Vec<_> = contents.split('\n').collect();
 

    let mut p_prev:&str = &"";
    let mut p_curr:&str = &lines[0];
    let mut p_next:&str = &lines[1];
    let mut result:bool = false;
    let mut val:i32 = 0;
    let mut sum = 0;

    for i in 2..lines.len()+2  {
        let prev:Vec<_> = p_prev.trim().bytes().collect();
        let curr:Vec<_> = p_curr.trim().bytes().collect();
        let next:Vec<_> = p_next.trim().bytes().collect();
        println!("P:{}", p_prev);
        println!("C:{}", p_curr);
        println!("N:{}", p_next);
        for j in 0..curr.len() {
            if is_num(curr[j]) {
                if check_neighbor(&prev, j) == true {
                    result = true;
                }
                if check_neighbor(&next, j) == true {
                    result = true;
                }
                if (j != 0) && is_symbol(curr[j-1]) {
                    result = true;
                }
                if (j < curr.len()-1) && is_symbol(curr[j+1]) {
                    result = true;
                }

                val = val * 10 + (curr[j] as i32 - 48);
            }
            else {
                if val != 0 && result == true {
                    sum = sum + val;
                }
                val = 0;
                result = false;
            }
        }

        p_prev = p_curr;
        p_curr = p_next;
        if i < lines.len() {
            p_next = &lines[i];
        }
        else {
            p_next = &"";
        }
        println!("Sum: {}", sum)

    }




 //   for line in contents.lines() {
 //       println!("\n{}\n",line);
 //       let line_parse: Vec<_> = line.trim().split(':').collect();
 //       let game_parse:Vec<_> = line_parse[0].split(' ').collect();
 //       let game_id = game_parse[1].parse::<i32>().unwrap();
 //       
 //       let trys = line_parse[1].split(';');
 //       let mut good_try = true;
//
 //       for try in trys {
 //           let cubes = try.split(',');
 //           for cube in cubes {
 //               let cube_parse:Vec<_> = cube.trim().split(" ").collect();
 //               let cube_count = cube_parse[0].parse::<i32>().unwrap();
//
 //               if (cube_parse[1] == "red") && (cube_count > max_red) {
 //                   good_try = false;
 //               }
 //               if (cube_parse[1] == "green") && (cube_count > max_green) {
 //                   good_try = false;
 //               }
 //               if (cube_parse[1] == "blue") && (cube_count > max_blue) {
 //                   good_try = false;
 //               }                
 //           }
 //       }
//
 //       if good_try == true {
 //           println!("Good try\n");
 //           game_counter = game_counter + game_id;
 //       }
 //       else {
 //           println!("Bad try\n");
 //       }
//
 //   }
 //   println!("Total: {}", game_counter);
}
