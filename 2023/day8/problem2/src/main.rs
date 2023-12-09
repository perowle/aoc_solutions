use std::fs;

struct PathStruct {
    node: u64,
    end: u8,
    left_path: u64,
    right_path: u64,
}


fn makenode(n: &str) -> u64 {
    let v = n.as_bytes();
    let ret:u64 = v[0] as u64*255*255 + v[1] as u64*255 + v[2] as u64;

    return ret;
}

fn makeend(n: &str) -> u8 {
    let v = n.as_bytes();
    let ret:u8 = v[2] as u8;

    return ret;
}

fn main() {
    
    let contents = fs::read_to_string("..\\..\\data\\day8.txt").unwrap();
    let lines:Vec<_> = contents.split('\n').collect();
    let instr = lines[0].trim();

    let mut paths:Vec<PathStruct> = Vec::new();

    for idx in 2..lines.len() {
        let mut path = PathStruct{node:0, left_path:0, right_path:0, end: 0};
        let block:Vec<_> = lines[idx].split('=').collect();
        path.node = makenode(block[0].trim());
        path.end = makeend(block[0].trim());
        let b1 = block[1].replace("(", " ");
        let b2 = b1.replace(")", " ");
        let u:Vec<_> = b2.trim().split(',').collect();
        path.left_path = makenode(u[0].trim());
        path.right_path = makenode(u[1].trim());

        println!("Path: {}   L: {}   R: {}", path.node, path.left_path, path.right_path);
        paths.push(path);
    }

    let mut count:u64 = 0;
    let mut idx:Vec<usize> = Vec::new();
    let mut lcm:Vec<u64> = Vec::new();
    let dest: u8 = 'Z' as u8;

    paths.sort_by_key(|x| (x.node));
 
    for i in 0..paths.len() {
        if paths[i].end == 'A' as u8 {
            idx.push(i);
        }
    }

    let mut cur_node:usize;
    let mut found:bool = false;

    for q in 0..idx.len() {
        cur_node = idx[q];
        while found == false {
            for i in instr.chars() {   
                count += 1;
                if i == 'L' {   
//                    println!("Left");
                    if let Ok(s_idx) = paths.binary_search_by_key(&paths[cur_node].left_path, |x| x.node) {
                        cur_node = s_idx;
                    }
                    else {
                        println!("Nowhere to go");
                    }
                }
                else {
 //                   println!("Right");
                    if let Ok(s_idx) = paths.binary_search_by_key(&paths[cur_node].right_path, |x| x.node) {
                        cur_node = s_idx;
                    }
                    else {
                        println!("Nowhere to go");
                    }
                }

                if paths[cur_node].end == dest {
                    found = true;
                    break;
                }
            }
        }
        lcm.push(count);
        println!("{}", count);
        count = 0;
        found = false;
    }


}
