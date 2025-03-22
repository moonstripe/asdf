# **ASDF-oxide**

## A CLI Utility for Sorting Pixels in Images

## **Overview**

ASDF-oxide is a command-line utility that sorts pixels in images based on their brightness or color value. It can be used to create interesting effects such as sorting by brightness, creating a "scanline" effect, or simply to experiment with different image processing techniques.

## **Usage**

### Installation

Install using cargo:

```sh
cargo install asdf-oxide
```

### Sorting Modes

- `white`: Sorts pixels from darkest to lightest
- `black`: Sorts pixels from lightest to darkest
- `bright`: Sorts pixels by their brightness (lightest to darkest)
- `dark`: Sorts pixels by their darkness (darkest to lightest)

### Commands

```bash
asdf-oxide --input <input_file> --output <output_file> --direction <direction> --mode <mode>
```

- `<input_file>`: Input file path or read from stdin if not provided
- `<output_file>`: Output file path or write to stdout if not provided
- `<direction>`: Processing direction ('h' for columns first, 'v' for rows first)
- `<mode>`: Sorting mode (white, black, bright, dark)

## **Examples**

### Sort by Brightness in Columns

```bash
asdf-oxide --input input.png --output output.png --direction h --mode bright
```

### Sort by Darkness in Rows

```bash
asdf-oxide --input input.png --output output.png --direction v --mode dark
```

## **License**

ASDF-OXIDE is licensed under the MIT License.
