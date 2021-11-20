use std::{cmp::min, collections::HashSet, convert::TryInto, fs::File, io::{self, BufRead, BufReader, Write}};

fn lev(first: &str, second: &str) -> usize {
    let b_len = second.chars().count();
    
    let mut row: Vec<usize> = (1..b_len+1).collect();
    let mut distance = 0;

    for (i1, e1) in first.chars().enumerate() {
        distance = i1 + 1;
        let mut distance_b = i1;

        for (i2, e2) in second.chars().enumerate() {
            let distance_a = distance_b + (if e1 == e2 { 0 } else { 1 }) as usize;
            distance_b = row[i2];
            distance = min(distance + 1, min(distance_a, distance_b + 1));
            row[i2] = distance;
        }
    }

    distance
}

fn find_best<'a>(word: &'a str, d: &'a HashSet<String>) -> &'a str {
    if d.contains(word) {
        return word;
    }
    let mut best_score = 999999;
    let mut best_word: &str = d.iter().next().unwrap();
    let length_word:i32 = word.chars().count().try_into().unwrap();
    for matching in d {
        let len_match:i32 = matching.chars().count().try_into().unwrap();
        let dif_size = (len_match - length_word).abs();
        if dif_size > 5 {
            continue; // probabilistic check
        }
        let score = lev(word, &matching);
        if score == 0 {
            return matching;
        } else if score < best_score {
            best_score = score;
            best_word = matching;
        }
    }

    best_word
}

fn main() {
    let d_file = File::open("dictionary.txt").unwrap();
    let d_reader = BufReader::new(d_file);
    let mut d :HashSet<String> = HashSet::with_capacity(1024);
    for line in d_reader.lines() {
        let line = line.unwrap();
        d.insert(line);
    }

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let input = File::open("original.txt").unwrap();
    let d_input = BufReader::new(input);
    for line in d_input.lines() {
        let line = line.unwrap();
        for word in line.split_ascii_whitespace() {
            let best_match = find_best(word, &d);
            stdout.write(best_match.as_bytes());
            stdout.write(" ".as_bytes());
        }
        stdout.write("\n".as_bytes());
    }
}
