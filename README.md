# Sudoku

解数独

[![build badge](https://github.com/LJason77/Sudoku/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/LJason77/Sudoku/actions/workflows/rust.yml)
![GitHub forks](https://img.shields.io/github/forks/LJason77/Sudoku?style=social)
![GitHub Repo stars](https://img.shields.io/github/stars/LJason77/Sudoku?style=social)

## 依赖

```shell
# 依赖
sudo apt install -y libleptonica-dev libtesseract-dev tesseract-ocr bc
```

需要安装 adb 并且配置好环境变量。

## 训练模型

```shell
git clone --depth=1 https://github.com/tesseract-ocr/tesstrain.git ; cd tesstrain
pip3 install -r requirements.txt
make tesseract-langdata
mkdir data/num-ground-truth/
# 将 1-9 的图片（每个数字一张图片） 放到 data/num-ground-truth/ 中
for i in {1..9}; do echo $i > data/num-ground-truth/$i.gt.txt; done
mkdir data/tessdata
wget -c 'https://raw.githubusercontent.com/tesseract-ocr/tessdata_best/main/eng.traineddata' -O data/tessdata/eng.traineddata
git clone --depth=1 https://github.com/tesseract-ocr/langdata_lstm.git data/langdata
make training MODEL_NAME=num START_MODEL=eng TESSDATA=data/tessdata
sudo mv data/num.traineddata /usr/share/tesseract-ocr/4.00/tessdata/
```

## 运行

```shell
# 先截图再运行
adb exec-out screencap -p > screenshot.png ; cargo run --release
```

## 许可

[![996.icu](https://img.shields.io/badge/link-996.icu-red.svg)](https://996.icu)
[![LICENSE](https://img.shields.io/badge/license-Anti%20996-blue.svg)](https://github.com/996icu/996.ICU/blob/master/LICENSE)
![GitHub](https://img.shields.io/github/license/LJason77/Sudoku)
