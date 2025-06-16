use image::{GenericImageView, DynamicImage, imageops::FilterType};
use terminal_size::{Width, Height, terminal_size};
use std::env;
use std::fs::{File, write};
use std::io::Write;
use std::path::Path;

const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";

fn image_to_ascii(img: DynamicImage, width: u32) -> String {
    let (original_width, original_height) = img.dimensions();
    let ratio = original_height as f32 / original_width as f32;
    let height = (width as f32 * ratio * 0.55) as u32;

    let img = img.resize_exact(width, height, FilterType::Nearest).grayscale();

    let mut ascii_art = String::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let luma = pixel[0] as f32 / 255.0;
            let idx = (luma * (ASCII_CHARS.len() - 1) as f32).round() as usize;
            ascii_art.push(ASCII_CHARS[idx] as char);
        }
        ascii_art.push('\n');
    }

    ascii_art
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} <caminho_da_imagem>", args[0]);
        return;
    }

    let img_path = &args[1];
    let img = image::open(&Path::new(img_path)).expect("Não foi possível abrir a imagem");

    let terminal_width = if let Some((Width(w), Height(_))) = terminal_size() {
        w.min(100) as u32
    } else {
        80
    };

    let ascii = image_to_ascii(img, terminal_width);

    // Salvar o resultado no arquivo
    let output_path = "ascii_output.txt";
    write(output_path, &ascii).expect("Não foi possível escrever o arquivo");

    println!("✅ ASCII Art salva em '{}'", output_path);
}
