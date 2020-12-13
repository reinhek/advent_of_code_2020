fn main() {
    let data = read_input("day2.txt");
    let mut result1 = 0;
    let mut result2 = 0;
    for (_i, line) in data.iter().enumerate() {
        let line_vec : Vec<String> = line.split_whitespace().map(|s| s.to_owned()).collect();
        let num_const : Vec<usize> = line_vec[0].split('-').map(|i| i.parse().unwrap()).collect();
        let a = num_const[0] - 1;
        let b = num_const[1] - 1;

        let character : String= line_vec[1].split(':').collect();

        let pass = &line_vec[2][..];

        if pass.matches(&character).count() >= a &&
            pass.matches(&character).count() <= b {
            result1 += 1;
        }

        if (pass.chars().nth(a).unwrap().to_string() == character && pass.chars().nth(b).unwrap().to_string() != character) ||
            (pass.chars().nth(a).unwrap().to_string() != character && pass.chars().nth(b).unwrap().to_string() == character){
            result2 += 1;
        }

    }
    println!("{} {}", result1, result2);


}

fn read_input(file: &str) -> Vec<String> {
    let string_read = std::fs::read_to_string(format!("input/{}",file)).unwrap();
    string_read.lines().map(|s| s.to_owned()).collect()
}
