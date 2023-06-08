#![deny(clippy::pedantic)]

use std::{process::Command, thread::sleep, time::Duration};

use image::imageops;

static mut SUDOKU: [[u8; 9]; 9] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0],
];
// 水平方向
static mut HORIZONTAL: [u16; 9] = [0; 9];
// 垂直方向
static mut VERTICAL: [u16; 9] = [0; 9];
// 3x3方格
static mut SQUARE: [u16; 9] = [0; 9];
// 是否结束
static mut FINISHED: bool = false;

// 计算次数
static mut COUNT: u32 = 0;

#[allow(clippy::cast_possible_truncation)]
fn main() {
    // 暂停
    touch(1000, 365);
    // 打开图片
    let mut image = image::open("screenshot.png").unwrap().into_rgb8();
    // 裁剪图片
    let width = 1044;
    let height = 1044;
    let mut buffer = imageops::crop(&mut image, 20, 443, width, height).to_image();

    let mut spaces: Vec<(usize, usize)> = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            let x = j * 116;
            let y = i * 116;
            let square = imageops::crop(&mut buffer, x as u32, y as u32, 110, 110).to_image();
            let result = tesseract::ocr_from_frame(&square, 110, 110, 3, 110 * 3, "num");
            if let Ok(n) = result.unwrap().trim().parse::<u8>() {
                unsafe {
                    SUDOKU[i][j] = n;
                    HORIZONTAL[i] += 1 << n;
                    VERTICAL[j] += 1 << n;
                    SQUARE[(i / 3) * 3 + j / 3] += 1 << n;
                }
            } else {
                spaces.push((i, j));
            }
        }
    }

    // 开始递归
    unsafe {
        let mut sudoku = SUDOKU;
        fill(0, &mut sudoku, &spaces);
        // 打印结果
        print_result(&sudoku);

        // 打印次数
        println!("{COUNT}");

        // 恢复游戏
        touch(540, 1700);
        sleep(Duration::from_millis(500));
        // 填入答案
        for (i, j) in spaces {
            // 空格坐标
            let x = 15 + 56 + j * 116;
            let y = 442 + 56 + i * 116;
            touch(x, y);
            // 填入数字
            let n = sudoku[i][j] as usize;
            let x: usize = 15 + 56 + (n - 1) * 116;
            let y = 2000;
            touch(x, y);
        }
    }
}

// 打印结果
fn print_result(sudoku: &[[u8; 9]; 9]) {
    sleep(Duration::from_millis(10));
    Command::new("clear").status().expect("failed to execute process");
    for (i, row) in sudoku.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if j % 3 == 0 {
                print!("|");
            }
            print!("{col} ");
        }
        if i % 3 == 2 {
            print!("\n---------------------");
        }
        println!();
    }
}

unsafe fn fill(pos: usize, sudoku: &mut [[u8; 9]; 9], spaces: &Vec<(usize, usize)>) {
    if pos == spaces.len() {
        FINISHED = true;
        return;
    }
    let (i, j) = spaces[pos];

    let mut n = 1;
    while !FINISHED && n <= 9 {
        let square = (i / 3) * 3 + j / 3;
        if (HORIZONTAL[i] >> n) & 1 == 1
            || (VERTICAL[j] >> n) & 1 == 1
            || (SQUARE[square] >> n) & 1 == 1
        {
            n += 1;
            continue;
        }
        sudoku[i][j] = n;
        HORIZONTAL[i] += 1 << n;
        VERTICAL[j] += 1 << n;
        SQUARE[square] += 1 << n;
        COUNT += 1;
        fill(pos + 1, sudoku, spaces);
        HORIZONTAL[i] -= 1 << n;
        VERTICAL[j] -= 1 << n;
        SQUARE[square] -= 1 << n;
        n += 1;
    }
}

// 模拟 Android 点击
fn touch(x: usize, y: usize) {
    Command::new("adb")
        .arg("shell")
        .arg("input")
        .arg("tap")
        .arg(format!("{x}"))
        .arg(format!("{y}"))
        .output()
        .expect("failed to execute process");
    sleep(Duration::from_millis(15));
}
