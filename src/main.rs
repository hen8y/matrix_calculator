use std::io;


#[derive(Debug)]
struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<Vec<i32>>
}


impl Matrix {

    fn new(rows: usize, columns: usize) -> Matrix {
        Matrix {
            rows,
            columns,
            data: vec![vec![0; columns]; rows]
        }
    }

    fn set_data(&mut self, name: &str) {
        println!("Enter the numbers for the {name} {}x{} matrix", self.rows, self.columns);

        for i in 0..self.rows {
            let mut number_input = String::new();

            io::stdin()
                .read_line(&mut number_input)
                .expect("failed to read input");

            let row: Vec<i32> = number_input.trim()
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().expect("Input is not a number"))
                    .collect();
            self.data[i] = row
        }
    }

    fn add(&self, second_matrix: Matrix) -> Matrix {
        let mut result = Matrix::new(self.rows, self.columns);

        for r in 0..self.rows {
            for c in 0..self.columns {
                result.data[r][c] = self.data[r][c] + second_matrix.data[r][c]
            }
        }
        result
    }



    fn subtract(&self, second_matrix: Matrix) -> Matrix {
        let mut result = Matrix::new(self.rows, self.columns);

        for r in 0..self.rows {
            for c in 0..self.columns {
                result.data[r][c] = self.data[r][c] - second_matrix.data[r][c]
            }
        }
        result
    }

    fn show_result(&self) {
        println!("You result \n");
        for row in &self.data {
            let result = row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
            println!("{result}")
        }
    }
}


fn get_dimensions(name: &str) -> (usize, usize) {
    println!("Enter the {name} matrix dimension\nFormat:(row column)");
    let mut dimension_input = String::new();
    io::stdin()
        .read_line(&mut dimension_input)
        .expect("failed to read input");

    let dimensions: Vec<usize> = dimension_input.trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().expect("Input is not a number"))
                .collect();
    (dimensions[0], dimensions[1])
}



fn main() {

    let (row1, column1) = get_dimensions("first");
    let (row2, column2) = get_dimensions("second");

    let mut first_matrix = Matrix::new(row1, column1);
    let mut second_matrix = Matrix::new(row2, column2);

    first_matrix.set_data("first");
    second_matrix.set_data("second");

    println!("Choose the operation sign.\nOptions('+', '-')");
    let mut operator_input = String::new();

    io::stdin()
        .read_line(&mut operator_input)
        .expect("failed to read input");

    let operator: &str = operator_input.trim();


    match operator {
        "+" => first_matrix.add(second_matrix).show_result(),
        "_" => first_matrix.subtract(second_matrix).show_result(),
        _ => println!("choose either add or subtract")
    }

}
