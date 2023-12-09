use std::fs;

fn main() {
    println!("Day 9 - Part 1");

    let str = fs::read_to_string("../_input/day09.txt").unwrap();
    let lines: Vec<&str> = str.lines().collect();

    let total = process(lines);
    println!("Total: {}", total);
}

fn process(lines: Vec<&str>) -> i32 {
    let readings: Vec<Vec<i32>> = lines
        .iter()
        .map(|s| s.split_whitespace().map(str_to_i32).collect())
        .collect();

    let mut total = 0;
    for reading in readings {
        let mut stack = vec![reading];
        let mut i = 0;
        while !stack[i].iter().all(|e| *e == 0) {
            let v = stack[i].windows(2).map(|w| w[1] - w[0]).collect();
            stack.push(v);
            i += 1;
        }

        stack.reverse();
        stack[0].push(0);

        let mut i = 1;
        while i < stack.len() {
            let a = {
                let a = stack.get(i - 1).unwrap();
                *a.last().unwrap()
            };
            let b = {
                let b = stack.get(i).unwrap();
                *b.last().unwrap()
            };

            stack.get_mut(i).unwrap().push(a + b);

            i += 1;
        }

        total += stack.last().unwrap().last().unwrap();
    }

    total
}

fn str_to_i32(str: &str) -> i32 {
    str.parse().unwrap()
}

#[cfg(test)]
mod test {
    use crate::process;

    #[test]
    fn test() {
        assert_eq!(
            process(
                r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
                    .split('\n')
                    .collect()
            ),
            114
        );
    }
}
