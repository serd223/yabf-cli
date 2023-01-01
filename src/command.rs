use std::str::FromStr;

pub enum Command {
    Help,  // Print help text
    Begin, // Begin writing to current code buffer
    End,   // End writing to current code buffer
    Clear, // Clear current code buffer
    Run,   // Run current code buffer
    Show,  // Print current code buffer
    Set,   // Set current code buffer to be the current program
    Exit,  // Exit program
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_uppercase().as_str() {
            "HELP" => Ok(Self::Help),
            "BEGIN" => Ok(Self::Begin),
            "END" => Ok(Self::End),
            "RUN" => Ok(Self::Run),
            "EXIT" => Ok(Self::Exit),
            "SHOW" => Ok(Self::Show),
            "CLEAR" => Ok(Self::Clear),
            "SET" => Ok(Self::Set),
            _ => Err(()),
        };
    }
}
