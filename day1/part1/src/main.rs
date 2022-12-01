use std::fs;

fn main() {
    let file_path = "input1.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    let mut x: Vec<_> = contents
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .into_iter()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|e| e.iter().sum::<i32>())
        .collect();

    x.sort();
    x.reverse();

    println!("Part 1 {}", x.first().unwrap());
    println!("Part 2 {}", x.iter().take(3).sum::<i32>());
}
