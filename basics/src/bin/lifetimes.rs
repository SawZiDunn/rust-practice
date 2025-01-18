fn main() {
    // let s1 = "abcd";
    // let s2 = "def";

    // let s3 = longer_str(s1, s2);
    // println!("s3: {}", s3);

    let s1 = String::from("abcd");
    let s2 = String::from("def");

    {
        let s3 = longer_str(&s1, &s2);
        let s3 = String::from(s3);
        println!("{:?}", s3);
    }
}

fn longer_str<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// generic, trait bound, and lifetimes altogether
fn longest_str_announce<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
