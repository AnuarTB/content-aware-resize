# Introduction

Content aware resize is a smart resizing of an image, which takes into account the contents of the image and resizes them in an optimal manner. This tool was inspired by [this article](https://avikdas.com/2019/05/14/real-world-dynamic-programming-seam-carving.html) from Avik Das and the original paper[1] of Avidan and Shamir.

## Usage
```sh
Resizes your images in a smart way!

USAGE:
    content-aware-resize [OPTIONS] <dx> -i <INPUT_FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i <INPUT_FILE>         input image path
    -o <OUTPUT_FILE>        output file name [default: output.jpg]

ARGS:
    <dx>    By what number of pixels reduce the width of image
```

Sample invocation
```sh
cargo run --release -- -i input.jpg -o output.jpg 600
```

## Citations
[1] Shai Avidan and Ariel Shamir. 2007. Seam carving for content-aware image resizing. ACM Trans. Graph. 26, 3 (July 2007), 10–es. DOI:https://doi.org/10.1145/1276377.1276390

