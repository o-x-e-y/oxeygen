use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Couldn't parse the thing you sent lmfao")]
    ParseError,
}

pub fn cli() -> Result<(), CliError> {
    // let keyboard = IsoAngle::keyboard();
    // let corpus = Corpus::load("./data/shai.json").expect("this should not fail");
    // let keyboard_stats = KeyboardStats::default_types(keyboard, corpus);
    // let layout_keys: [char; 30] = "bgdlzjfou,nstrkycaeiqvmhxpw';."
    //     .chars()
    //     .collect::<Vec<_>>()
    //     .try_into()
    //     .unwrap();
    // let layout = keyboard_stats.layout_with(layout_keys);
    // keyboard_stats.print_stats(&layout);

    Ok(())
}
