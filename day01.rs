fn main() {
    let data = read_input("day1.txt");
    
    //part 1 solution 1
    for (i, num1) in data.iter().enumerate() {
        for num2 in data.iter().skip(i) {
            if num1 + num2 == 2020 {
                println!("{} {}", num1, num2);
                println!("{}", num1*num2);
            }
        }
    }
    
    //part 1 solution 2
    //array should be sorted before executing below
    //data.sort_unstable();
    let product = data.iter()
        .find(|&&num| data.binary_search(&(2020 - num)).is_ok())
        .map(|&num| num * (2020-num))
        .unwrap();
    println!("{}", result);
    
    
    //part 2 solution
    for (i, num1) in data.iter().enumerate() {
        for (j, num2) in data.iter().enumerate().skip(i) {
            for num3 in data.iter().skip(j) {
                if num1 + num2 + num3 == 2020 {
                    println!("{} {} {}", num1, num2, num3);
                    println!("{}", num1 * num2 * num3);
                }
            }
        }
    }

}

fn read_input(file: &str) -> Vec<isize> {
    let string_read = std::fs::read_to_string(format!("input/{}",file)).unwrap();
    let int_read : Vec<String> = string_read.split_whitespace().map(|s| s.to_owned()).collect();
    int_read.into_iter().map(|i| i.parse().unwrap()).collect()
}
