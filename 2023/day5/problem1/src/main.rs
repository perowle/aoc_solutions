
use std::fs;

struct MapsType{
    start_src:u64,
    start_dst:u64,
    length:u64,
}


fn main() {
    let mut seed_soil:Vec<MapsType> = Vec::new();
    let mut soil_fert:Vec<MapsType> = Vec::new();
    let mut fert_water:Vec<MapsType> = Vec::new();
    let mut water_light:Vec<MapsType> = Vec::new();
    let mut water_light:Vec<MapsType> = Vec::new();
    let mut light_temp:Vec<MapsType> = Vec::new();
    let mut temp_humid:Vec<MapsType> = Vec::new();
    let mut humid_location:Vec<MapsType> = Vec::new();
     

    let contents = fs::read_to_string("..\\data\\day5.txt").unwrap();
    let lines:Vec<_> = contents.split('\n').collect();
    let mut line_idx:usize = 0;
 
    let mut seeds:Vec<_> = lines[0].trim().split(' ').collect();
    seeds.remove(0);

    println!("Seeds: {}:{}    {}", seeds[0], seeds[1], seeds.len());

    let mut category = 0;
    let mut categoryswitch:bool = false;

    for line_idx in 3..lines.len() {
        if lines[line_idx].trim() == "" {
            println!("Switching categories");
            category += 1;
            categoryswitch = true;
            continue;
        }

        if categoryswitch == true {
            categoryswitch = false;
            continue;
        }

        let mut value:MapsType = MapsType {start_dst:0, start_src:0, length:0};
        let line:Vec<_> = lines[line_idx].trim().split(' ').collect();
        value.start_dst = line[0].parse().unwrap();
        value.start_src = line[1].parse().unwrap();
        value.length = line[2].parse().unwrap();

        println!("Value: {}, {}, {}", value.start_src, value.start_dst, value.length);

        match category {
            0 => seed_soil.push(value),
            1 => soil_fert.push(value),
            2 => fert_water.push(value),
            3 => water_light.push(value),
            4 => light_temp.push(value),
            5 => temp_humid.push(value),
            6 => humid_location.push(value),
            _ => println!("Unknown category"),
        }
    }
    println!("SS: {}", seed_soil.len());
    println!("SF: {}", soil_fert.len());
    println!("FW: {}", fert_water.len());
    println!("WL: {}", water_light.len());
    println!("LT: {}", light_temp.len());
    println!("TH: {}", temp_humid.len());
    println!("HL: {}", humid_location.len());


    let mut soil:u64 = 0;
    let mut fert:u64 = 0;
    let mut water:u64 = 0;
    let mut light:u64 = 0;
    let mut temp:u64 = 0;
    let mut humid:u64 = 0;
    let mut location:u64 = 0;
    let mut min_location:u64 = u64::MAX;

    for seed in seeds {
        let seed_num:u64 = seed.parse().unwrap();

        soil = seed_num;
        for i in 0..seed_soil.len() {
            if seed_num >= seed_soil[i].start_src && seed_num <= (seed_soil[i].start_src + seed_soil[i].length) {
                soil = seed_soil[i].start_dst + (seed_num - seed_soil[i].start_src);
                break;
            }
        }

        fert = soil;
        for i in 0..soil_fert.len() {
            if soil >= soil_fert[i].start_src && soil <= (soil_fert[i].start_src + soil_fert[i].length) {
                fert = soil_fert[i].start_dst + (soil - soil_fert[i].start_src);
                break;
            }
        }

        water = fert;
        for i in 0..fert_water.len() {
            if fert >= fert_water[i].start_src && fert <= (fert_water[i].start_src + fert_water[i].length) {
                water = fert_water[i].start_dst + (fert - fert_water[i].start_src);
                break;
            }
        }

        light = water;
        for i in 0..water_light.len() {
            if water >= water_light[i].start_src && water <= (water_light[i].start_src + water_light[i].length) {
                light = water_light[i].start_dst + (water - water_light[i].start_src);
                break;
            }
        }

        temp = light;
        for i in 0..light_temp.len() {
            if light >= light_temp[i].start_src && light <= (light_temp[i].start_src + light_temp[i].length) {
                temp = light_temp[i].start_dst + (light - light_temp[i].start_src);
                break;
            }
        }

        humid = temp;
        for i in 0..temp_humid.len() {
            if temp >= temp_humid[i].start_src && temp <= (temp_humid[i].start_src + temp_humid[i].length) {
                humid = temp_humid[i].start_dst + (temp - temp_humid[i].start_src);
                break;
            }
        }

        location = humid;
        for i in 0..humid_location.len() {
            if humid >= humid_location[i].start_src && humid <= (humid_location[i].start_src + humid_location[i].length) {
                location = humid_location[i].start_dst + (humid - humid_location[i].start_src);
                break;
            }
        }
        println!("{} {} {} {} {} {} {} {}", seed_num, soil, fert, water, light, temp, humid, location);

        if (location < min_location) {
            min_location = location;
        }
    }
    println!("Location: {}", min_location);

}
    
