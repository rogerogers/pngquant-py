#[pyo3::pymodule]
mod pngquant_py {
    use imagequant::Attributes;
    use pyo3::{prelude::*, types::PyBytes};

    /// Quantize an image (passed as bytes) and return the compressed PNG bytes.
    ///
    /// Args:
    ///     input_data (bytes): The input image file content (PNG, JPEG, etc.).
    ///     quality_min (int): Minimum quality (0-100). Default 0.
    ///     quality_max (int): Maximum quality (0-100). Default 100.
    ///     speed (int): Speed/quality trade-off (1-10). 1 is slow/best, 10 is fast/worst. Default 3.
    ///
    /// Returns:
    ///     bytes: The quantized PNG file content.
    #[pyfunction]
    #[pyo3(signature = (input_data, quality_min=0, quality_max=100, speed=3))]
    fn quantize(
        py: Python,
        input_data: &[u8],
        quality_min: u8,
        quality_max: u8,
        speed: i32,
    ) -> PyResult<Py<PyAny>> {
        // 1. Decode the input image using the `image` crate to get raw RGBA pixels.
        let img = image::load_from_memory(input_data)
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to decode image: {}",
                    e
                ))
            })?
            .to_rgba8();

        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixels: Vec<imagequant::RGBA> = img
            .pixels()
            .map(|p| imagequant::RGBA {
                r: p.0[0],
                g: p.0[1],
                b: p.0[2],
                a: p.0[3],
            })
            .collect();

        // 2. Configure imagequant attributes.
        let mut attr = Attributes::new();
        attr.set_quality(quality_min, quality_max).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid quality range: {}", e))
        })?;
        attr.set_speed(speed).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid speed: {}", e))
        })?;

        // 3. Create the imagequant image from raw RGBA pixels.
        // 0.0 gamma lets imagequant guess or use default.
        let mut iq_image = attr.new_image(pixels, width, height, 0.0).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to create imagequant image: {}",
                e
            ))
        })?;

        // 4. Quantize the image.
        let mut res = attr.quantize(&mut iq_image).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Quantization failed: {}", e))
        })?;

        // 5. Remap the image to the generated palette.
        let (palette, indices) = res.remapped(&mut iq_image).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Remapping failed: {}", e))
        })?;

        // 6. Encode the result as a PNG with a palette using the `png` crate.
        let mut out_buffer = Vec::new();
        let mut encoder = png::Encoder::new(&mut out_buffer, width as u32, height as u32);
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Eight);

        // Convert imagequant palette (RGBA) to png crate palette (RGB) + Transparency chunk (tRNS).
        let mut palette_bytes = Vec::with_capacity(palette.len() * 3);
        let mut trans_bytes = Vec::with_capacity(palette.len());
        let mut has_transparency = false;

        for px in palette {
            palette_bytes.extend_from_slice(&[px.r, px.g, px.b]);
            trans_bytes.push(px.a);
            if px.a < 255 {
                has_transparency = true;
            }
        }

        encoder.set_palette(palette_bytes);
        if has_transparency {
            encoder.set_trns(trans_bytes);
        }

        let mut writer = encoder.write_header().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to write PNG header: {}",
                e
            ))
        })?;

        writer.write_image_data(&indices).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to write PNG data: {}",
                e
            ))
        })?;

        writer.finish().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to finish PNG: {}",
                e
            ))
        })?;

        Ok(PyBytes::new(py, &out_buffer).into())
    }
}
