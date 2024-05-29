use artlist::*;
use miette::IntoDiagnostic as _;

const FILE_NAME: &str = "artists.kdl";

fn main() -> miette::Result<()> {
    let text = std::fs::read_to_string(FILE_NAME).into_diagnostic()?;
    let accounts = knuffel::parse::<Vec<Art>>(FILE_NAME, &text)?;
    dbg! {accounts};
    Ok(())
}
