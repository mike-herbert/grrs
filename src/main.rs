use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

use failure::ResultExt;
use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    // let args = Cli::from_args();
    // println!("{path}, {pattern}", path=&args.path.display(), pattern=&args.pattern);
    // let content = std::fs::read_to_string(&args.path)
    //     .expect("could not read file");
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("Could not read file `{}`", &args.path.display()))?;
    // let content = match result {
    //     Ok(content) => { content },
    //     Err(error) => { return Err(error.into()); }
    // };
    
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}