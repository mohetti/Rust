use std::collections::HashMap;
fn main() {
    return_median(vec![2, 2, 2, 3, 3, 3]);
    println!("*****NEXT*****");
    return_median(vec![2, 2, 3, 3, 3]);
    println!("*****NEXT*****");
    return_median(vec![2, 2, 2, 1, 3, 3, 3]);
    println!("*****NEXT*****");
    return_median(vec![2, 2, 2, 1, 4, 1, 3, 3, 3, 3]);

    fn return_median(list: Vec<i32>) {
        let mut map = HashMap::new();
        let mut mode = 0;
        let mut modes = Vec::new();
        let mut mode_final: i32 = 0;

        for integer in &list {
            let count = map.entry(integer).or_insert(0);
            *count += 1;
        }

        for (val, key) in map {
            if key > mode {
                mode = key;
                modes = Vec::new();
                modes.push(val);
            } else if key == mode {
                modes.push(val);
            }
        }
        println!("modes length {}", &modes.len());
        if modes.len() == 1 {
            mode_final = *modes[0];
            println!("The mode is {}", mode_final);
        } else {
            for value in modes {
                mode_final += value;
            }
            let else_mode_final: f32;
            else_mode_final = mode_final as f32 / 2 as f32;
            println!("The mode is {}", else_mode_final);
        }

        if list.len() % 2 != 0 {
            let length = list.len() / 2;
            println!("The median is {}", list[length])
        } else {
            let length = list.len() / 2;
            let median = (list[length] as f32 + list[length - 1] as f32) / 2 as f32;
            println!("The median is {}", median)
        }
    }
}
