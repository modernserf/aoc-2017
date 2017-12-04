use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};


fn main() {
    let contents = get_file_contents();

    let valid_passphrases = count_matches(&contents, |word| String::from(word));
    println!("part 1: {}", valid_passphrases);

    let valid_passphrases = count_matches(&contents, |word| Anagram::from(word));
    println!("part 2: {}", valid_passphrases);
}


fn get_file_contents() -> String {
    let mut file = File::open("./src/input.txt")
        .expect("could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file as string");

    contents
}


fn validate_with_hasher<H: Hash + Eq>(s: &str, hasher: fn(&str) -> H) -> bool {
    if s.len() == 0 { return false; }

    let mut used_words = HashSet::new();

    for word in s.split(" ") {
        let hashed_word = hasher(word);
        if used_words.contains(&hashed_word) {
            return false;
        }
        used_words.insert(hashed_word);
    }

    true
}


fn count_matches<H: Hash + Eq>(contents: &str, hasher: fn(&str) -> H) -> u32 {
     contents
        .split("\n")
        .filter(|x| validate_with_hasher(x, hasher))
        .fold(0, |count, _| { count + 1 })
}


#[derive(Eq)]
struct Anagram {
    data: HashMap<String, u32>
}

impl Anagram {
    fn from(s: &str) -> Anagram {
        let mut bag = HashMap::new();

        for ch in s.split("") {
            let ch_reified = String::from(ch);
            let count = bag.entry(ch_reified).or_insert(0);
            *count += 1;
        }

        Anagram { data: bag }
    }
}

impl PartialEq for Anagram {
    fn eq(&self, other: &Anagram) -> bool {
        self.data == other.data
    }
}

impl Hash for Anagram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut list = self.data.iter()
            .collect::<Vec<(&String, &u32)>>();

        list.sort_by_key(|&(k, _)| k);
        list.hash(state);
    }
}
