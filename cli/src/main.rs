use structopt::StructOpt;
use regex::Regex;

use swizzler::{
    errors::ErrorKind,
    ChannelDescriptor,
    GenericAssetReader,
    GenericTarget,
    GenericWriter,
    AssetMatcher,
    SessionBuilder,
    to_rgba,
    to_rgb,
    to_luma,
    to_lumaA,
    to_dynamic
};

use image::{
    DynamicImage,
    RgbaImage,
    open
};

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct ManualCommand {

    #[structopt(long = "input", short)]
    descriptors: Vec<String>,

    #[structopt(
        long = "output",
        short,
        parse(from_os_str),
        default_value = "output.png"
    )]
    output: std::path::PathBuf

}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct SessionCommand {

    #[structopt(long = "folder", short, parse(from_os_str))]
    folders: Vec<std::path::PathBuf>,

}

#[derive(StructOpt)]
enum Command {

    Manual(ManualCommand),

    Session(SessionCommand)

}

#[derive(StructOpt)]
#[structopt(
    name = "swizzler-cli",
    about = "Swizzle images components intp a single output",
    rename_all = "kebab-case"
)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command
}

fn process_manual(command: &ManualCommand) -> Result<(), ErrorKind> {
    let descriptors: Vec<Option<ChannelDescriptor>> =
        (command.descriptors
            .iter()
            .map(|s| -> Result<Option<ChannelDescriptor>, ErrorKind> {
                Ok(Some(ChannelDescriptor::from_description(&s)?))
            })
            .collect::<Result<Vec<Option<ChannelDescriptor>>, ErrorKind>>()
        )?;

    let image = to_dynamic(&descriptors)?;
    image.save(&command.output)?;
    Ok(())
}

fn process_session(command: &SessionCommand) -> Result<(), ErrorKind> {
    let mut builder = SessionBuilder::new().add_folders(&command.folders);
    let generic_reader = GenericAssetReader::new(
        Regex::new(r"(.*)_.*").unwrap(),
        vec![
            AssetMatcher::new("metalness", Regex::new(r"(?i)metal(ness)?").unwrap()),
            AssetMatcher::new("roughness", Regex::new(r"(?i)rough(ness)?").unwrap())
        ]
    );

    let generic_writer = GenericWriter::new(vec![
        GenericTarget::new(vec! [
            Some((String::from("metalness"), 0)),
            None,
            None,
            Some((String::from("roughness"), 0))
        ])
    ]);

    let session = builder.build(&generic_reader)?;
    session.run(&generic_writer);
    Ok(())
}

fn main() -> Result<(), ErrorKind> {
    let args = Opt::from_args();

    match &args.cmd {
        Command::Manual(manual) => {
            process_manual(&manual)
        },
        Command::Session(session) => {
            process_session(&session)
        },
        _ => Ok(())
    }

}
