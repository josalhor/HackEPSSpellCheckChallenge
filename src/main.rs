use std::{cmp::min, collections::HashSet, fs::File, io::{self, BufRead, BufReader, Write}};

fn lev(first: &str, second: &str) -> i32 {
    let second_len = second.chars().count() as i32;
    
    let mut row: Vec<i32> = (1..second_len+1).collect();
    let mut distance:i32 = 0;

    for (i1, e1) in first.chars().enumerate() {
        let i1 = i1 as i32;
        distance = i1 + 1;
        let mut distance_b:i32 = i1;

        for (i2, e2) in second.chars().enumerate() {
            let distance_a = distance_b + (if e1 == e2 { 0 } else { 1 });
            distance_b = row[i2];
            distance = min(distance + 1, min(distance_a, distance_b + 1));
            row[i2] = distance;
        }
    }
    
    distance
}

fn find_best_it<'a, T>(word: &'a str, it: T) -> &'a str
where T: Iterator<Item=&'a String>
{
    let mut best_score = 999999;
    let mut best_word: Option<&str> = None;
    let length_word:i32 = word.chars().count() as i32;
    for matching in it {
        let len_match:i32 = matching.chars().count() as i32;
        let dif_size = (len_match - length_word).abs();
        if dif_size > best_score {
            /*
            Because we know that we are iterating on an ordered
            vector once we find that the delta is too large,
            it can only increase!
            WARNING: This check _cannot_ be equal.
            The equality check is only correct after the threshold of equal lengths
            with the word we are comparing.
            But it is NOT correct before the threshold!!
            */
            return best_word.unwrap();
        } else if dif_size > 5 {
            continue; // probabilistic check
        }
        let score = lev(word, &matching);
        /*
        Because we already checked
        against a dictionary, we know
        this is impossible
        if score == 0 {
            return matching;
        } */
        if score < best_score {
            best_score = score;
            best_word = Some(matching);
        }
    }

    best_word.unwrap()
}

#[inline]
fn find_best<'a>(word: &'a str, d: &'a HashSet<String>, v: &'a Vec<String>, len_shortest:i32, len_longest:i32) -> &'a str {
    if d.contains(word) {
        return word;
    }
    let length_word:i32 = word.chars().count() as i32;
    let dif_short = (len_shortest - length_word).abs();
    let dif_long = (len_longest - length_word).abs();
    if dif_short < dif_long {
        find_best_it(word, v.iter())
    } else {
        find_best_it(word, v.iter().rev())
    }
}

fn main() {
    let d_file = File::open("dictionary.txt").unwrap();
    let d_reader = BufReader::new(d_file);
    let mut dict :HashSet<String> = HashSet::with_capacity(1024);
    let mut ordered: Vec<String> = Vec::with_capacity(1024);
    for line in d_reader.lines() {
        let line = line.unwrap();
        ordered.push(line.to_string());
        dict.insert(line);
    }
    ordered.sort_by(|a, b| {
        let l1:i32 = a.chars().count() as i32;
        let l2:i32 = b.chars().count() as i32;
        l1.partial_cmp(&l2).unwrap()
    });
    let len_shortest:i32 = ordered.first().unwrap().chars().count() as i32;
    let len_longest:i32 = ordered.last().unwrap().chars().count() as i32;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let input = File::open("original.txt").unwrap();
    let d_input = BufReader::new(input);
    for line in d_input.lines() {
        let line = line.unwrap();
        for word in line.split_ascii_whitespace() {
            let best_match = find_best(word, &dict, &ordered, len_shortest, len_longest);
            let _ = stdout.write(best_match.as_bytes());
            let _ = stdout.write(" ".as_bytes());
        }
        let _ = stdout.write("\n".as_bytes());
    }
}
