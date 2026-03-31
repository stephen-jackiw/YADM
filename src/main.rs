pub(crate) mod config;
mod files;

use crate::config::YadmConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: YadmConfig = confy::load("yadm", Some("yadm"))?;
    dbg!(&cfg);

    let files = files::load_matching(&cfg.file);

    dbg!(files);

    Ok(())
}
