use std::fs;



fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("day2.txt").unwrap();
    let mut total_power:i32 = 0;

    for line in contents.lines() {
        println!("\n{}\n",line.trim());
        let line_parse: Vec<_> = line.trim().split(':').collect();
        let trys = line_parse[1].split(';');

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for try in trys {
            let cubes = try.split(',');
            for cube in cubes {
                let cube_parse:Vec<_> = cube.trim().split(" ").collect();
                let cube_count = cube_parse[0].parse::<i32>().unwrap();

                if (cube_parse[1] == "red") && (cube_count > min_red) {
                    min_red = cube_count;
                }
                if (cube_parse[1] == "green") && (cube_count > min_green) {
                    min_green = cube_count;
                }
                if (cube_parse[1] == "blue") && (cube_count > min_blue) {
                    min_blue = cube_count;
                }                
            }
        }

        let try_power = min_red * min_blue * min_green;
        println!("Try power: {}\n", try_power);

        total_power = total_power + try_power;
        
    }
    println!("Total: {}", total_power);
}
