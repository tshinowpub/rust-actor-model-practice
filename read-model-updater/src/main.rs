use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub mod consumer;

/*
 * @see https://docs.rs/signal-hook/latest/signal_hook/
 * @see https://dev.to/talzvon/handling-unix-kill-signals-in-rust-55g6
 * @see https://qiita.com/qnighy/items/4bbbb20e71cf4ae527b9
 */
fn main() -> Result<()> {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;

    while !term.load(Ordering::Relaxed) {
        println!("Hello, world!");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {}
}
