use bc_unshit::{cli::ArgumentParser, Unshitter};

fn main() -> Result<(), String> {
    let args = ArgumentParser::arguments();

    let unshitter = Unshitter::new(args.source, args.destination, args.remove_source);

    unshitter.go()?;

    Ok(())
}
