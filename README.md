# pngquant-py

A high-performance image quantization library written in Rust with Python bindings provided via PyO3. It efficiently converts 24/32-bit PNG images to 8-bit indexed color images while maintaining excellent visual quality.

## Features

- **High Performance**: Written in Rust for exceptional speed and efficiency
- **Superior Quality**: Based on the [libimagequant](https://github.com/ImageOptim/libimagequant) library, offering state-of-the-art color quantization algorithms
- **Python-Friendly**: Provides a clean and intuitive Python API via PyO3
- **Flexible Configuration**: Supports adjustable quality, speed, and other parameters
- **Multiple Input Formats**: Compatible with PNG, JPEG, and other common image formats

## Installation

### From PyPI

```bash
pip install pngquant-py
```

### From Source

Requires Rust and Python environments. Follow these steps:

```bash
# Clone the repository
git clone https://github.com/rogerogers/pngquant_py.git
cd pngquant_py

# Install dependencies and package using uv
uv sync
```

Build directly with maturin:

```bash
maturin develop
```

## Usage Examples

### Basic Usage

```python
import pngquant_py

# Read input image
with open("input.png", "rb") as f:
    input_data = f.read()

# Quantize the image
output_data = pngquant_py.quantize(input_data)

# Save the output image
with open("output.png", "wb") as f:
    f.write(output_data)
```

### Advanced Configuration

```python
import pngquant_py

with open("input.png", "rb") as f:
    input_data = f.read()

# Customize quality and speed parameters
output_data = pngquant_py.quantize(
    input_data,
    quality_min=60,  # Minimum quality (0-100)
    quality_max=90,  # Maximum quality (0-100)
    speed=4          # Speed/quality trade-off (1-10, 1=slowest/best, 10=fastest/acceptable)
)

with open("output.png", "wb") as f:
    f.write(output_data)
```

## API Documentation

### `quantize(input_data, quality_min=0, quality_max=100, speed=3)`

Quantizes an input image (provided as bytes) and returns the compressed PNG bytes.

#### Parameters

- `input_data` (bytes): Input image file content (PNG, JPEG, etc.).
- `quality_min` (int, optional): Minimum quality (0-100). Default: 0.
- `quality_max` (int, optional): Maximum quality (0-100). Default: 100.
- `speed` (int, optional): Speed/quality trade-off (1-10). 1 is slowest/best quality, 10 is fastest/lower quality. Default: 3.

#### Returns

- `bytes`: Quantized PNG file content.

#### Exceptions

- `ValueError`: Raised if input data cannot be decoded as an image, or if quality/speed parameters are invalid.
- `RuntimeError`: Raised if an error occurs during the quantization process.

## Development Guide

### Environment Setup

1. Install Rust: <https://www.rust-lang.org/tools/install>
2. Install Python 3.12+
3. Install uv: `pip install uv`
4. Install maturin: `uv install maturin`

### Building the Project

```bash
# Build in development mode
maturin develop

# Build in release mode
maturin build --release
```

### Running Tests

```bash
# Run Rust unit tests
cargo test

# Run Python examples
python examples/main.py
```
