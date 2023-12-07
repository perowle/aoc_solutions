use std::fs;

struct PlayerStruct {
    hand: [usize;5],
    strength: u64,
    bid: u64,
}

struct DeckStruct {
    card_rank: usize,
    card_count: u64,
}


fn rank(card: u8) -> usize {
    let mut ret: usize = 0;

    match card {
        50 => ret = 0,     // 2
        51 => ret = 1,
        52 => ret = 2,
        53 => ret = 3,
        54 => ret = 4,
        55 => ret = 5,
        56 => ret = 6,
        57 => ret = 7,    // 9
        84 => ret = 8,    // T
        74 => ret = 9,    // J
        81 => ret = 10,   // Q
        75 => ret = 11,   // K
        65 => ret = 12,   // A
        _ => println!("WTH"),
    }
    return ret;
}

fn classify(cards: [usize;5]) -> u64 {
    let mut deck:[u64; 13] = [0;13];
    let five_oak = 7;
    let four_oak = 6;
    let full_house = 5;
    let three_oak = 4;
    let two_pair = 3;
    let two_oak = 2;
    let high_card = 1;
    let mut ret = high_card;

    let mut s_deck:Vec<DeckStruct> = Vec::new();
    let mut c:DeckStruct;
    for i in 0..cards.len()
    {
        deck[cards[i]] += 1;
    }

    for i in 0..deck.len()
    {
        c = DeckStruct{card_rank:0, card_count:0};
        if deck[i] != 0 {
            c.card_count = deck[i];
            c.card_rank = i;
            s_deck.push(c);
        }
    }

    s_deck.sort_by_key(| x | (x.card_count));
    s_deck.reverse();

    for i in 0..s_deck.len()
    {
        if s_deck[i].card_count == 5 {
            ret = five_oak;
            break;
        }
        else if s_deck[i].card_count == 4 {
            ret = four_oak;
            break;
        }
        else if s_deck[i].card_count == 3 {
            ret = three_oak;
        }
        else if s_deck[i].card_count == 2 {
            if ret == high_card {
                ret = two_oak;
            }
            else if ret == three_oak {
                ret = full_house;
                break;
            }
            else if ret == two_oak {
                ret = two_pair;
                break;
            }
        }
    }
    return ret;
}


fn main() {
    let mut play:PlayerStruct;
    let mut plays:Vec<PlayerStruct> = Vec::new();
    let mut total_score:u64 = 0;
    
    let contents = fs::read_to_string("..\\data\\day7.txt").unwrap();
    let lines:Vec<_> = contents.split('\n').collect();
    for line in lines {
        play = PlayerStruct{strength:0,bid:0,hand:[0;5]};
        let player:Vec<_> = line.split(' ').collect();
        let t = player[0].as_bytes();
        for i in 0..t.len() {
            play.hand[i] = rank(t[i]);
        }
        play.strength = classify(play.hand);
        println!("Hand: {}    Strength: {}", player[0], play.strength);

        play.bid = player[1].trim().parse().unwrap();
        plays.push(play);
        
    }

    plays.sort_by_key(|x| (x.strength, x.hand[0], x.hand[1], x.hand[2], x.hand[3], x.hand[4]));


    let mut s: u64 = 1;
    for i in 0..plays.len() {
        total_score += plays[i].bid * s;
        s += 1;

    }
    println!("Total score: {}", total_score);    
}
