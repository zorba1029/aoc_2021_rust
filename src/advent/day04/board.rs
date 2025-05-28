#[derive(Debug, Clone)]
pub struct Board {
    pub values: Vec<i32>,
    pub size_x: usize,
    pub size_y: usize,
    pub marks: Vec<bool>,
}

impl Board {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        let v = vec![0i32; size_x * size_y];
        let m = vec![false; size_x * size_y];
        Board {
            values: v,
            size_x,
            size_y,
            marks: m,
        }
    }

    pub fn set_value_xy(&mut self, x: usize, y: usize, value: i32) {
        // println!("set(x, y) = {}, {}, value: {}", x, y, value);
        self.values[(x * self.size_x) + y] = value;
    }

    // pub fn set_value(&mut self, index: usize, value: i32) {
    //     // println!("set(x, y) = {}, {}, value: {}", x, y, value);
    //     self.values[index] = value;
    //     self.set_mark(index, value);
    //
    // }

    pub fn find_value(&self, draw_value: i32) -> (bool, usize) {
        for (i, v) in self.values.iter().enumerate() {
            // if self.values[i] == draw_value {
            if *v == draw_value {
                return (true, i);
            }
        }

        (false, 0)
    }

    // pub fn set_mark_xy(&mut self, x: usize, y: usize) {
    //     self.marks[(x * self.size_x) + y] = true;
    // }
    // pub fn get_mark_xy(&self,  x: usize, y: usize) -> bool {
    //     if self.marks[(x * self.size_x) + y] == true {
    //         return true;
    //     }

    //     return false;
    // }

    pub fn set_mark(&mut self, index: usize, _value: i32) {
        self.marks[index] = true;
        let x = index / self.size_x;
        let y = index % self.size_y;
        self.marks[(x * self.size_x) + y] = true;
        // println!("self.marks({})[{},{}], value:{}", index, row, col, value);
    }

    // pub fn get_mark(&self, index: usize) -> bool {
    //     if self.marks[index] == true {
    //         return true;
    //     }

    //     return false;
    // }

    // pub fn is_bingo_row(&self) -> (bool, usize) {
    //     let mut bingo = false;
    //     for row_index in 0..self.size_y {
    //         let mut bingo = true;
    //         for col_index in 0..self.size_x {
    //             if self.marks[(row_index * self.size_y) + col_index] != true {
    //                 bingo = false;
    //             }
    //         }
    //         if bingo == true {
    //             return (true, row_index);
    //         }
    //     }
    //     return (bingo, 0);
    // }

    pub fn is_bingo_row_col(&self) -> (bool, usize) {
        let mut bingo = false;
        for row_index in 0..self.size_y {
            bingo = true;
            for col_index in 0..self.size_x {
                if self.marks[(row_index * self.size_y) + col_index] != true {
                    bingo = false;
                }
            }
            if bingo == true {
                return (true, row_index);
            }
        }

        for row_index in 0..self.size_y {
            let mut bingo = true;
            for col_index in 0..self.size_x {
                if self.marks[row_index + (self.size_y * col_index)] != true {
                    bingo = false;
                }
            }
            if bingo == true {
                return (true, row_index);
            }
        }

        (bingo, 0)
    }

    // pub fn get_bingo_value(&self, row_index: usize) -> i32 {
    //     let mut bingo_sum = 0;
    //     for col_index in 0..self.size_x {
    //         bingo_sum += self.values[(row_index * self.size_y) + col_index];
    //     }
    //     for col_index in 0..self.size_x {
    //         println!("BINGO_VALUE: {} ", self.values[(row_index * self.size_y) + col_index]);
    //     }
    //
    //     println!("BINGO_SUM: {} ", bingo_sum);
    //     return bingo_sum;
    // }

    pub fn get_sum_unmarked_values(&self) -> i32 {
        let mut bingo_sum = 0;
        for row in 0..self.size_y {
            for col in 0..self.size_x {
                if self.marks[(row * self.size_y) + col] != true {
                    bingo_sum += self.values[(row * self.size_y) + col]
                }
            }
        }

        println!("BINGO_SUM: {} ", bingo_sum);
        bingo_sum
    }

    pub fn print_board_values(&self) {
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                // print!("{:3} ", board.values[(x * board.size_x) + y]);
                if self.marks[(x * self.size_x) + y] == true {
                    print!("{:3} ", self.values[(x * self.size_x) + y]);
                } else {
                    print!("  . ");
                }
            }
            println!();
        }
    }

    pub fn print_board_marks(&self) {
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                if self.marks[(x * self.size_x) + y] == true {
                    print!("  T ");
                } else {
                    print!("  . ");
                }
            }
            println!();
        }
    }
}
