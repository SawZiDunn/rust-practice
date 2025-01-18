enum FileSize {
    Bytes(u64),
    KiloBytes(u64),
    MegaBytes(u64),
    GigaBytes(u64),
}

impl FileSize {
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{:.2} Bytes", bytes),
            FileSize::KiloBytes(kbs) => format!("{:.2} KB", *kbs as f64 / 1000.),
            FileSize::MegaBytes(mbs) => format!("{:.2} MB", *mbs as f64 / 1000.),
            FileSize::GigaBytes(gbs) => format!("{:.2} GB", *gbs as f64 / 1000.),
        }
    }
}

fn main() {
    let size = 6888837399;
    let file_size = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::KiloBytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::MegaBytes(size / 1_000_000),
        _ => FileSize::GigaBytes(size / 1_000_000_000),
    };
    let result = file_size.format_size();
    println!("{}", result);
}
