#[macro_use] extern crate structopt;
extern crate console;
extern crate dialoguer;
extern crate heck;

use std::io::Write;
use std::fs::File;
use std::path::PathBuf;

use structopt::StructOpt;
use console::style;
use heck::KebabCase;

#[derive(StructOpt)]
enum Command {
    Show,
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

fn main() -> Result<(), Box<::std::error::Error>> {
    let args = Cli::from_args();

    let steps = [
        "Zweck und Grundsätze",
        "Ergebnis",
        "Ideensammlung",
        "Organisieren",
        "Nächste Schritte",
    ];

    match args.cmd {
        Some(Command::Show) => show(&steps)?,
        None => new_doc(&steps)?,
    }

    Ok(())
}

fn show(steps: &[&str]) -> Result<(), Box<::std::error::Error>> {
    println!("{}", style("Natürlich Planung").bold());

    steps.iter().for_each(|step| println!("- {}", step));
    Ok(())
}

fn new_doc(steps: &[&str]) -> Result<(), Box<::std::error::Error>> {
    println!("{}", style("Natürlich Planung").bold());
    println!();

    let title = dialoguer::Input::new("Aufgabe").interact()?;
    let mut res = Vec::new();

    writeln!(&mut res, "# {}\n", title)?;

    for step in steps {
        let answer = dialoguer::Input::new(step).interact()?;
        writeln!(&mut res, "## {}", step)?;
        writeln!(&mut res, "{}\n", answer)?;
    }

    let filename = PathBuf::from(title.to_kebab_case()).with_extension("md");
    let mut file = File::create(&filename)?;
    file.write_all(&res)?;

    println!();
    println!("Als {} gespeichert.", style(filename.display()).bold());

    Ok(())
}
