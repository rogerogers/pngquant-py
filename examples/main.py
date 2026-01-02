import pngquant_py


def main():
    # Read input image
    with open("input.png", "rb") as f:
        input_data = f.read()

    # Quantize
    # quality_min=60, quality_max=90, speed=4
    output_data = pngquant_py.quantize(input_data, 60, 90, 4)

    # Save output image
    with open("output.png", "wb") as f:
        f.write(output_data)


if __name__ == "__main__":
    main()
