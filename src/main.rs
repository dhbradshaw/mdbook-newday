use mdbook_newday;
use std::env;
use std::io::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = args
        .get(1)
        .expect("The first argument must be the filepath for SUMMARY.md.");
    mdbook_newday::update_summary(path)
}
