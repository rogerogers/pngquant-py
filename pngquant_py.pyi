def quantize(
    input_data: bytes, quality_min: int = 0, quality_max: int = 100, speed: int = 3
) -> bytes:
    """
    Quantize an image (passed as bytes) and return the compressed PNG bytes.

    Args:
        input_data (bytes): The input image file content (PNG, JPEG, etc.).
        quality_min (int, optional): Minimum quality (0-100). Default 0.
        quality_max (int, optional): Maximum quality (0-100). Default 100.
        speed (int, optional): Speed/quality trade-off (1-10). 1 is slowest/best quality, 10 is fastest/lower quality. Default 3.

    Returns:
        bytes: The quantized PNG file content.

    Raises:
        ValueError: If input data cannot be decoded as an image, or if quality/speed parameters are invalid.
        RuntimeError: If an error occurs during the quantization process.
    """
    ...
