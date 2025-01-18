fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
    let mut largest = list[0];
    for i in list {
        if *i > largest {
            largest = *i;
        }
    }
    largest
}

fn largest0<T>(list: &[T]) -> &T
where
    T: PartialOrd + Copy,
{
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn main() {
    let list = vec![5, 4, 2, 4, 9];
    let list0 = vec![1.1, 5.5, 2.2, 3.6, 0.8];

    let largest = largest(&list);
    let largest0 = largest0(&list);
    println!("largest: {}, largest0: {}", largest, largest0);
}
