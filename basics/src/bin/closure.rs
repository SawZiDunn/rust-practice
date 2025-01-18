use std::cmp::Ordering;

fn main() {
    let mut arr: [i32; 8] = [4, 8, 1, 10, 0, 45, 12, 7];
    arr.sort(); // rearrange the original array
    println!("{:?}", arr);

    let sum: i32 = arr.iter().sum();
    println!("{}", arr.iter().sum::<i32>());

    // or
    let sum = arr.iter().fold(0, |acc, &x| acc + x);
    println!("{}", sum);

    fn desc(a: &i32, b: &i32) -> Ordering {
        if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }

    arr.sort_by(desc);

    // arr.sort_by(|a, b| {
    //     if a < b {
    //         Ordering::Greater
    //     } else if a > b {
    //         Ordering::Less
    //     } else {
    //         Ordering::Equal
    //     }
    // });
    println!("{:?}", arr);

    let (x, y) = (2, 5);
    println!("{:?}", x.cmp(&y));
}
