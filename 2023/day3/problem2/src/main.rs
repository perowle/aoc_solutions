
use std::fs;


fn is_num(c:u8)->bool {
    if c >= 48  && c <= 57  {
        return true;
    }
    else {
        return false;
    }
}


fn check_neighbor(check:&Vec<u8>, i:usize)->usize {
    let mut res:usize = usize::MAX;
    let start:usize;
    let end:usize;

    if check.len() == 0
    {
        return usize::MAX;
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
        if is_num(check[j]) {
            res = j;
            break;
        }
    }

    return res;
}


fn get_number(check:&Vec<u8>, i:usize)->i32 {
    let mut val = 0;
    let mut start:usize = i;
    let mut end:usize = i;

    for _j in 1..=5 {
        if start > 0 && is_num(check[start-1]) {
            start = start - 1;
        }
        else {
            break;
        }
    }

    for _j in 1..=5 {
        if end < check.len()-1 && is_num(check[end+1]) {
            end = end + 1;
        }
        else {
            break;
        }
    }

    println!("S:{}  E:{}", start, end);
    for j in start..=end {
        val = val * 10 + (check[j] as i32 - 48);
    }

    return val;
}



fn main() {

    let contents = fs::read_to_string("day3.txt").unwrap();

    println!("{}", contents.len());
    let lines:Vec<_> = contents.split('\n').collect();
 

    let mut p_prev:&str = &"";
    let mut p_curr:&str = &lines[0];
    let mut p_next:&str = &lines[1];
    let mut val:[i32;2]= [0,0];
    let mut idx = 0;
    let mut res:usize;
    let mut sum = 0;

    for i in 2..lines.len()+2  {
        let prev:Vec<_> = p_prev.trim().bytes().collect();
        let curr:Vec<_> = p_curr.trim().bytes().collect();
        let next:Vec<_> = p_next.trim().bytes().collect();

        println!("P:{}", p_prev);
        println!("C:{}", p_curr);
        println!("N:{}", p_next);
        for j in 0..curr.len() {
            if curr[j] == 42 {
                println!("Prev");
                if is_num(prev[j]) {
                    res = check_neighbor(&prev, j);
                    if res != usize::MAX {
                        val[idx] = get_number(&prev, res);
                        idx = idx + 1;
                    }
                }
                else {
                    if j > 0 && is_num(prev[j-1]) {
                        res = check_neighbor(&prev, j-1);
                        if res != usize::MAX {
                            val[idx] = get_number(&prev, res);
                            idx = idx + 1;
                        }
                    }
                    if j < curr.len()-1 && is_num(prev[j+1]) {
                        res = check_neighbor(&prev, j+1);
                        if res != usize::MAX {
                            val[idx] = get_number(&prev, res);
                            idx = idx + 1;
                        }
                    }
                }

                if is_num(next[j]) {
                    res = check_neighbor(&next, j);
                    if res != usize::MAX {
                        val[idx] = get_number(&next, res);
                        idx = idx + 1;
                    }
                }
                else {
                    if j > 0 && is_num(next[j-1]) {
                        res = check_neighbor(&next, j-1);
                        if res != usize::MAX {
                            val[idx] = get_number(&next, res);
                            idx = idx + 1;
                        }
                    }
                    if j < curr.len()-1 && is_num(next[j+1]) {
                        res = check_neighbor(&next, j+1);
                        if res != usize::MAX {
                            val[idx] = get_number(&next, res);
                            idx = idx + 1;
                        }
                    }
                }     

                if (j != 0) && is_num(curr[j-1]) {
                    val[idx] = get_number(&curr, j-1);
                    idx = idx + 1;
                }

                if (j < curr.len()-1) && is_num(curr[j+1]) {
                    val[idx] = get_number(&curr, j+1);
                    idx = idx + 1;
                }
                if idx == 2 {
                    sum = sum + val[0] * val[1];
                }
                println!("Idx:{}  Val[1]:{}  Val[2]:{}", idx, val[0], val[1]);
            }
            else {
                idx = 0;
                val[0] = 0;
                val[1] = 0;
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

    }
    println!("SUM: {}", sum);
}
