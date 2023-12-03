use std::fs;



fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("day2.txt").unwrap();
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut game_counter:i32 = 0;

    for line in contents.lines() {
        println!("\n{}\n",line);
        let line_parse: Vec<_> = line.trim().split(':').collect();
        let game_parse:Vec<_> = line_parse[0].split(' ').collect();
        let game_id = game_parse[1].parse::<i32>().unwrap();
        
        let trys = line_parse[1].split(';');
        let mut good_try = true;

        for try in trys {
            let cubes = try.split(',');
            for cube in cubes {
                let cube_parse:Vec<_> = cube.trim().split(" ").collect();
                let cube_count = cube_parse[0].parse::<i32>().unwrap();

                if (cube_parse[1] == "red") && (cube_count > max_red) {
                    good_try = false;
                }
                if (cube_parse[1] == "green") && (cube_count > max_green) {
                    good_try = false;
                }
                if (cube_parse[1] == "blue") && (cube_count > max_blue) {
                    good_try = false;
                }                
            }
        }

        if good_try == true {
            println!("Good try\n");
            game_counter = game_counter + game_id;
        }
        else {
            println!("Bad try\n");
        }

    }
    println!("Total: {}", game_counter);
}
