use bc_unshit::{cli::ArgumentParser, utils::copy_dir};

fn main() -> Result<(), String> {
    let args = ArgumentParser::arguments();

    copy_dir(args.source, args.destination)?;

    Ok(())
}
