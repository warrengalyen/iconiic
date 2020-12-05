use std::{io, path::PathBuf};
use crossterm::{style, Color};

#[derive(Debug)]
pub enum Error {
    Syntax(SyntaxError),
    Iconiic(iconwriter::Error),
    Io(io::Error, PathBuf)
}

#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxError {
    UnexpectedToken(String),
    MissingOutputFlag,
    MissingOutputPath,
    UnsupportedOutputType(String),
    UnsupportedPngOutput(String)
}

const VALID_ICNS_SIZES: &str = "16x16, 32x32, 64x64, 128x128, 512x512 and 1024x1024";

impl Error {
    pub fn show(&self) {
        match &self {
            Error::Syntax(err) => show_syntax(err),
            Error::Iconiic(err) => show_iconwriter(err),
            Error::Io(err, path) => show_io(err, path.clone())
        }
    }
}

fn show_syntax(err: &SyntaxError) {
    match err {
        SyntaxError::MissingOutputFlag => println!(
            "{} Missing output details. Type {} for more details on Iconiic's usage.",
            style("[Syntax Error]").with(Color::Red),
            style("iconiic -h").with(Color::Blue)
        ),
        SyntaxError::MissingOutputPath => println!(
            "{} Missing output path: No path for the output file was specified. Type {} for more details on Iconiic's usage.",
            style("[Syntax Error]").with(Color::Red),
            style("iconiic -h").with(Color::Blue)
        ),
        SyntaxError::UnexpectedToken(token) => println!(
            "{} Unexpected token: {}.",
            style("[Syntax Error]").with(Color::Red),
            style(token).with(Color::Red)
        ),
        SyntaxError::UnsupportedOutputType(ext) => println!(
            "{} Files with the {} file extension are not supported.", 
            style("[IO Error]").with(Color::Red),
            style(format!(".{}", ext.to_lowercase())).with(Color::Blue)
        ),
        SyntaxError::UnsupportedPngOutput(ext) => println!(
            "{} The {} option only supports the {} file format. The {} file extension is not supported",
            style("[IO Error]").with(Color::Red),
            style("-png").with(Color::Blue),
            style(".zip").with(Color::Blue),
            style(format!(".{}", ext.to_lowercase())).with(Color::Blue)
        )
    }
}

fn show_iconwriter(err: &iconwriter::Error) {
    match err {
        iconwriter::Error::InvalidIcnsSize((w, h)) => if w == h {
            println!(
                "{} The {} file format only supports {} icons: {}x{} icons aren't supported.",
                style("[Icns Error]").with(Color::Red),
                style(".icns").with(Color::Blue),
                VALID_ICNS_SIZES,
                w, h
            )
        } else {
            println!(
                "{} The {} file format only supports square icons: {}x{} icons aren't supported.",
                style("[Icns Error]").with(Color::Red),
                style(".icns").with(Color::Blue),
                w, h
            )
        },
        iconwriter::Error::InvalidIcoSize((w, h)) => if w == h {
            println!(
                "{} The {} file format only supports icons of dimensions up to 256x256: {}x{} icons aren't supported.",
                style("[Ico Error]").with(Color::Red),
                style(".ico").with(Color::Blue),
                w, h
            )
        } else {
            println!(
                "{} The {} file format only supports square icons: {}x{} icons aren't supported.",
                style("[Ico Error]").with(Color::Red),
                style(".ico").with(Color::Blue),
                w, h
            )
        },
        iconwriter::Error::SizeAlreadyIncluded((_w, _h)) => unimplemented!(),
        iconwriter::Error::Io(_) => unreachable!(),
        _ => panic!("{:?}", err)
    }
}

fn show_io(err: &io::Error, path: PathBuf) {
    match err.kind() {
        io::ErrorKind::NotFound => println!(
            "{} File {} could not be found on disk.",
            style("[IO Error]").with(Color::Red),
            style(path.display()).with(Color::Blue)
        ),
        io::ErrorKind::PermissionDenied => println!(
            "{} Permission denied: File {} is inaccecible.",
            style("[IO Error]").with(Color::Red),
            style(path.display()).with(Color::Blue)
        ),
        io::ErrorKind::AddrInUse | io::ErrorKind::AddrNotAvailable => println!(
            "{} File {} is unavaiable. Try closing any application that may be using it.",
            style("[IO Error]").with(Color::Red),
            style(path.display()).with(Color::Blue)
        ),
        io::ErrorKind::InvalidData | io::ErrorKind::InvalidInput => println!(
            "{} File {} couln't be parsed. This file may be corrupted.",
            style("[IO Error]").with(Color::Red),
            style(path.display()).with(Color::Blue)
        ),
        _ => panic!("{:?}", err)
    }
} 