pub(crate) mod config;

use crate::config::YadmConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: YadmConfig = confy::load("yadm", Some("yadm"))?;
    dbg!(&cfg);

    let files = cfg.file.load_matching();

    dbg!(files);

    Ok(())
}
