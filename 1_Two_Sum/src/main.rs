use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}
fn main() {
    let mut hnum = HashMap::new();
    let mut sol = vec![];
    read_vec!(num as i32);
    if (2 <= num.len()) && (num.len() <= 104) {
        read!(target as i32);
        if (-109 <= target) && (target <= 109) {
            for x in &num {
                hnum.insert(x, x);
            }
            for i in 0..num.len() {
                if (-109 <= num[i]) && (num[i] <= 109) {
                    if target-num[i] != num[i] {
                        if hnum.contains_key(&(target - num[i])) {
                            //println!("{}", i);
                            sol.push(i);
                        } else {break;}
                    } else {break;}
                } else {break;}
            }
            println!("{:?}", sol);
        } else {return;}
    } else {return;}
}