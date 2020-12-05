extern crate iconwriter;
extern crate crossterm;
extern crate regex;

mod parse;
mod error;

use std::{env, io, fs, path::{Path, PathBuf}, collections::HashMap};
use error::Error;
use iconwriter::{Icon, IconOptions, IconType, SourceImage, FromFile};
use crossterm::{style, Color};

pub enum Command {
    Help,
    Icon(HashMap<IconOptions, PathBuf>, IconType, PathBuf)
}

const TITLE: &str = r"
_____ _____ ____  _   _ _____ _____ _____ 
|_   _/ ____/ __ \| \ | |_   _|_   _/ ____|
  | || |   | |  | |  \| | | |   | || |     
  | || |   | |  | | . ` | | |   | || |     
 _| || |___| |__| | |\  |_| |_ _| || |____ 
|_____\_____\____/|_| \_|_____|_____\_____|

BETA 0.1.0";
const USAGE: &str = "iconiic (-e <file path> <size>... [-i | --interpolate] [-p | --proportional])... (-o <output path> | -png <output path>) | -h";
const EXAMPLES: [&str;2] = [
    "iconiic -e small.svg 16 20 24 -e big.png 32 64 -o output.ico",
    "iconiic -e image.png 32x12 64x28 48 -i -png output.zip"
];

const COMMANDS: [&str;3] = ["Specify an entrys options.", "Outputs to .ico or .icns file.", "Outputs a .png sequence as a .zip file."];
const OPTIONS:  [&str;3] = [
    "Apply linear interpolation when resampling the image.",
    "Preserves the aspect ratio of the image in the output.",
    "This option is only valid when outputing to png sequences."
];

macro_rules! help {
    () => {
        println!(
            "{}\n\n{}\n   {}\n\n{}{}\n{}{}\n{}{}\n\n{}\n{}{}\n{}{}\n                       {}\n\n{}\n   {}\n   {}\n",
            style(TITLE).with(Color::Green),
            style("Usage:").with(Color::Blue),
            style(USAGE).with(Color::Green),
            style("   -e (<options>)      ").with(Color::Green),
            COMMANDS[0],
            style("   -o <output path>    ").with(Color::Green),
            COMMANDS[1],
            style("   -png <output path>  ").with(Color::Green),
            COMMANDS[2],
            style("Options:").with(Color::Blue),
            style("   -i, --interpolate   ").with(Color::Green),
            OPTIONS[0],
            style("   -p, --proportional  ").with(Color::Green),
            OPTIONS[1],
            OPTIONS[2],
            style("Examples:").with(Color::Blue),
            style(EXAMPLES[0]).with(Color::Green),
            style(EXAMPLES[1]).with(Color::Green)
        );
    };
}

macro_rules! catch {
    ($e:expr, $p:expr) => {
        match $e {
            Ok(()) => Ok(()),
            Err(err) => match err {
                iconwriter::Error::Io(err) => Err(Error::Io(err, $p)),
                _ => Err(Error::Iconiic(err))
            }
        }
    };
}

fn main() {
    match parse::args(env::args_os().collect()) {
        Ok(cmd) => match cmd {
            Command::Icon(entries, icon_type, output_path) => if let Err(err) =  create_icon(&entries, icon_type, &output_path) {
                err.show();
            } else {
                let path = Path::new(&output_path);
                println!(
                    "{} File {} saved at {}",
                    style("[Iconiic]").with(Color::Green),
                    style(path.file_name().unwrap().to_string_lossy()).with(Color::Blue),
                    style(path.canonicalize().unwrap_or(env::current_dir().unwrap()).display()).with(Color::Blue)
                );
            },
            Command::Help => help!()
        },
        Err(err)  => err.show()
    }
}

fn create_icon(entries: &HashMap<IconOptions, PathBuf>, icon_type: IconType, output_path: &PathBuf) -> Result<(), Error> {
    let mut source_map = HashMap::with_capacity(entries.len());

    for path in entries.values() {
        if let None = source_map.get(path) {
            if let Some(source) = SourceImage::from_file(path) {
                source_map.insert(path, source);
            } else {
                return Err(Error::Io(io::Error::from(io::ErrorKind::NotFound), path.clone()));
            }
        }
    }

    let s_len = source_map.len();
    let mut icon = match icon_type {
        IconType::Ico  => Icon::ico(s_len),
        IconType::Icns => Icon::icns(s_len),
        IconType::PngSequence => Icon::png_sequence(s_len)
    };

    for (opts, path) in entries {
        if let Err(err) = icon.add_entry(opts.clone(), source_map.get(path)
            .expect("Variable 'source_map' should have a key for String 'path'")) {
            return catch!(Err(err), path.clone());
        }
    }

    match fs::File::create(output_path) {
        Ok(file) => catch!(icon.write(file), output_path.clone()),
        Err(err) => Err(Error::Io(err, output_path.clone()))
    }
}
