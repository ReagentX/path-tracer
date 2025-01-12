use indicatif::{ProgressBar, ProgressStyle};

pub fn build_progress_bar(pixels: u64) -> ProgressBar {
    let pb = ProgressBar::new(pixels);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.red} [{elapsed}] [{bar:.blue}] {pos}/{len} ({per_sec}, ETA: {eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_position(0);
    pb
}
