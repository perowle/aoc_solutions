use std::fs;

struct PathStruct {
    node: u64,
    left_path: u64,
    right_path: u64,
}


fn makenode(n: &str) -> u64 {
    let v = n.as_bytes();
    let ret:u64 = v[0] as u64*255*255 + v[1] as u64*255 + v[2] as u64;

    return ret;
}

fn main() {
    
    let contents = fs::read_to_string("..\\data\\day8.txt").unwrap();
    let lines:Vec<_> = contents.split('\n').collect();
    let instr = lines[0].trim();

    let mut paths:Vec<PathStruct> = Vec::new();

    for idx in 2..lines.len() {
        let mut path = PathStruct{node:0, left_path:0, right_path:0};
        let block:Vec<_> = lines[idx].split('=').collect();
        path.node = makenode(block[0].trim());
        let b1 = block[1].replace("(", " ");
        let b2 = b1.replace(")", " ");
        let u:Vec<_> = b2.trim().split(',').collect();
        path.left_path = makenode(u[0].trim());
        path.right_path = makenode(u[1].trim());

        println!("Path: {}   L: {}   R: {}", path.node, path.left_path, path.right_path);
        paths.push(path);
    }

    let mut count:i32 = 0;
    let mut idx:usize = 0;
    let dest:u64 = makenode("ZZZ");
    paths.sort_by_key(|x| (x.node));

    while paths[idx].node != dest {
        println!("Iterating again");
        for i in instr.chars() {
            if paths[idx].node == dest {
                break;
            }

            count += 1;

            if i == 'L' {
                for i in 0..paths.len() {
                    if paths[i].node == paths[idx].left_path {
                        idx = i;
                        break;
                    }
                }
            }
            else {
                for i in 0..paths.len() {
                    if paths[i].node == paths[idx].right_path {
                        idx = i;
                        break;
                    }
                }
            }
        }
    }

    println!("Result: {}", count);
}
