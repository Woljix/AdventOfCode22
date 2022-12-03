/* WIP */
static ASCII_ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
    'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y',
    'Z'
    ];

fn main() {
    let rucksack = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];
    /*
    THIS WORKS
    let mut alphabet_iter = ASCII_ALPHABET.iter();

    let lol = alphabet_iter.position(|x| x == &'a').unwrap();
    println!("{}", lol);
    */
    for text in rucksack {
        let mut num = 0;
        for character in text.chars() {
            let mut alphabet_iter = ASCII_ALPHABET.iter();

            let lol = (alphabet_iter.position(|x| x == &character).unwrap() + 1);
            println!("{}", lol);
            num += lol;
        }
        println!("Sum: {}", num);
        num = 0;
    }

    //let lol = alphabet.position(|x| x == 'c').unwrap();

    //println!("{}", lol);
}
