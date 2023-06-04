use std::fmt::Write;

use indicatif::{FormattedDuration, ProgressBar, ProgressState, ProgressStyle};
use yafo::pipeline::ProgressReporter;

#[derive(Debug)]
pub struct Reporter {
    progress_bar: ProgressBar,
    processed_size: usize,
}

impl Reporter {
    pub fn new(forward: bool) -> Self {
        let pb = ProgressBar::new(0);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} {prefix:.yellow} [{bar:40.white}] {bytes}/{total_bytes} ({eta_prompt:.blue} {eta_precise:.blue})",
            )
            .unwrap()
            .with_key("eta_prompt", |_state: &ProgressState, w: &mut dyn Write| {
                write!(w, "ETA").unwrap()
            })
            .tick_chars("⠈⠐⠠⢀⡀⠄⠂⠁⠈")
            .progress_chars("=> "),
        );
        pb.set_prefix(if forward { "Encrypting" } else { "Decrypting" });
        Self {
            progress_bar: pb,
            processed_size: 0,
        }
    }
}

impl ProgressReporter for Reporter {
    fn bytes_processed(&mut self, n: usize, total: Option<usize>) {
        let Some(total) = total else {
            return;
        };
        self.processed_size += n;

        let pb = &self.progress_bar;
        pb.set_length(total as u64);
        pb.set_position(self.processed_size as u64);
    }
}

impl Drop for Reporter {
    fn drop(&mut self) {
        self.progress_bar.finish_and_clear();
        let ela = self.progress_bar.elapsed();
        println!("\u{2728} Done in {}.", FormattedDuration(ela));
    }
}
