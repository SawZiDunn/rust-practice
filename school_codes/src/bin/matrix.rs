fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix.iter() {
        for item in row.iter() {
            print!("{} ", item);
        }
        println!();
    }
}

fn sum_row(matrix: &Vec<Vec<i32>>, row: usize) -> i32 {
    let mut total = 0;
    for i in matrix.iter() {
        for j in 0..i.len() {
            if j + 1 == row {
                total += i[j];
                break;
            }
        }
    }
    total
}

fn sum_column(matrix: &Vec<Vec<i32>>, column: usize) -> i32 {
    let mut total = 0;
    for row in matrix.iter() {
        for (i, item) in row.iter().enumerate() {
            if i == column - 1 {
                total += item;
            }
        }
    }
    total
}

fn sum_diagonal(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    let mut count = 0;
    for row in matrix.iter() {
        for (i, item) in row.iter().enumerate() {
            if count == i {
                total += item;
                count += 1;
                break;
            }
        }
    }
    total
}

fn sum_anti_diagonal(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut total = 0;
    let mut count: i32 = (matrix.len() - 1) as i32;
    for row in matrix.iter() {
        for i in 0..row.len() {
            if count == i as i32 {
                total += row[i];
                count -= 1;
                break;
            }
        }
    }
    total
}

fn main() {
    let matrix1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    print_matrix(&matrix1);

    let row_sum = sum_row(&matrix1, 1);
    println!("The sum of the row: {}", row_sum);

    let col_sum = sum_column(&matrix1, 1);
    println!("The sum of the column: {}", col_sum);

    let diagonal_sum = sum_diagonal(&matrix1);
    println!("The sum of the main diagonal: {}", diagonal_sum);

    let anti_diagonal_sum = sum_anti_diagonal(&matrix1);
    println!("The sum of the main anti-diagonal: {}", anti_diagonal_sum);
}
