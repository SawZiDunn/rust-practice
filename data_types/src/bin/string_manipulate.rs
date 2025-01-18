fn main() {
    let sentence = String::from("The Good, the Bad, and the Ugly");
    println!("{}", &sentence[0..3]);

    let z = format!("Movie Title: {}", sentence);
    println!("{}", z);

    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("found a vowel!"),
    //         _ => continue,
    //     }
    // }

    let words: Vec<&str> = sentence.split_whitespace().collect();
    // or
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}
