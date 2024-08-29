use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufWriter;
use image::{ImageBuffer, Rgba};
use resvg::usvg::{Tree, Options, FitTo};
use resvg::{render, tiny_skia};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <input.svg> <output.ico|webp> <size>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_filename = &args[2];
    let size: u32 = args[3].parse()?;

    let svg_data = std::fs::read(input_path)?;
    let tree = Tree::from_data(&svg_data, &Options::default())?;

    let mut pixmap = tiny_skia::Pixmap::new(size, size).unwrap();
    render(
        &tree,
        FitTo::Size(size, size),
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    );

    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
        size,
        size,
        pixmap.data().to_vec(),
    ).unwrap();

    let desktop_path = get_desktop_path()?;
    let output_path = desktop_path.join(output_filename);

    if output_filename.ends_with(".ico") {
        convert_to_ico(&image, &output_path, size)?;
    } else if output_filename.ends_with(".webp") {
        convert_to_webp(&image, &output_path)?;
    } else {
        eprintln!("Unsupported output format. Use .ico or .webp");
        std::process::exit(1);
    }

    println!("Conversion completed successfully! File saved to: {:?}", output_path);
    Ok(())
}

fn convert_to_ico(image: &ImageBuffer<Rgba<u8>, Vec<u8>>, output_path: &Path, size: u32) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    let icon_image = ico::IconImage::from_rgba_data(size, size, image.to_vec());
    icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image)?);
    icon_dir.write(&mut writer)?;

    Ok(())
}

fn convert_to_webp(image: &ImageBuffer<Rgba<u8>, Vec<u8>>, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    image.save(output_path)?;
    Ok(())
}

fn get_desktop_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    Ok(home_dir.join("Desktop"))
}