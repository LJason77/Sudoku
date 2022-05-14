#![deny(clippy::pedantic)]

// const SUDOKU: [[u8; 9]; 9] = [
//     [0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0],
//     // -------------------------
//     [0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0],
//     // -------------------------
//     [0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0],
//     [0, 0, 0, 0, 0, 0, 0, 0, 0],
// ];
const SUDOKU: [[u8; 9]; 9] = [
    [9, 2, 0, 0, 1, 0, 3, 0, 0],
    [8, 5, 0, 0, 9, 0, 0, 2, 0],
    [0, 0, 3, 0, 0, 0, 0, 0, 0],
    // -------------------------
    [0, 0, 0, 0, 0, 2, 0, 0, 0],
    [3, 0, 0, 0, 0, 1, 6, 0, 0],
    [1, 9, 7, 0, 0, 0, 2, 5, 0],
    // -------------------------
    [0, 0, 0, 5, 0, 9, 0, 6, 2],
    [0, 8, 5, 0, 2, 0, 4, 0, 0],
    [0, 0, 9, 7, 4, 0, 0, 3, 0],
];
// 水平方向
static mut HORIZONTAL: [u16; 9] = [0; 9];
// 垂直方向
static mut VERTICAL: [u16; 9] = [0; 9];
// 3x3方格
static mut SQUARE: [u16; 9] = [0; 9];

fn main() {
    // 获得空格的位置
    let mut spaces: Vec<(usize, usize)> = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            let n = SUDOKU[i][j];
            if n == 0 {
                spaces.push((i, j));
            } else {
                unsafe {
                    HORIZONTAL[i] += 1 << n;
                    VERTICAL[j] += 1 << n;
                    SQUARE[(i / 3) * 3 + j / 3] += 1 << n;
                }
            }
        }
    }

    // 开始递归
    let mut sudoku = SUDOKU;
    unsafe {
        fill(0, &mut sudoku, &spaces);
    }

    // 打印结果
    for (i, row) in sudoku.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if j % 3 == 0 {
                print!("|");
            }
            print!("{} ", col);
        }
        if i % 3 == 2 {
            print!("\n---------------------");
        }
        println!();
    }
}

unsafe fn fill(pos: usize, sudoku: &mut [[u8; 9]; 9], spaces: &Vec<(usize, usize)>) {
    if pos == spaces.len() {
        return;
    }
    let (i, j) = spaces[pos];

    for n in 1..10 {
        let square = (i / 3) * 3 + j / 3;
        if (HORIZONTAL[i] >> n) & 1 == 1
            || (VERTICAL[j] >> n) & 1 == 1
            || (SQUARE[square] >> n) & 1 == 1
        {
            continue;
        }
        sudoku[i][j] = n;
        HORIZONTAL[i] += 1 << n;
        VERTICAL[j] += 1 << n;
        SQUARE[square] += 1 << n;
        fill(pos + 1, sudoku, spaces);
        HORIZONTAL[i] -= 1 << n;
        VERTICAL[j] -= 1 << n;
        SQUARE[square] -= 1 << n;
    }
}
