use std::str::FromStr;

pub enum Command {
    Begin, // Begin writing to current code buffer
    End,   // End writing to current code buffer
    Run,   // Run current code buffer
    Show,  // Print current code buffer
    Exit,  // Exit program
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_uppercase().as_str() {
            "BEGIN" => Ok(Self::Begin),
            "END" => Ok(Self::End),
            "RUN" => Ok(Self::Run),
            "EXIT" => Ok(Self::Exit),
            "SHOW" => Ok(Self::Show),
            _ => Err(()),
        };
    }
}
