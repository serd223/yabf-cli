use std::str::FromStr;

pub enum Command {
    // Regular commands
    Help,  // Print help text
    Begin, // Begin writing to current code buffer
    End,   // End writing to current code buffer
    Clear, // Clear current code buffer
    Run,   // Run current code buffer
    Debug, // Debug the currently selected program
    Show,  // Print current code buffer
    Set,   // Set current code buffer to be the current program
    Exit,  // Exit program

    // Debug mode commands
    Next,     // Step to the next instruction
    NextOut,  // Keep stepping until the next Out instruction and print the program output
    Dump,     // Dump current program memory
    Out,      // Show current program out buffer
    EndDebug, // Stop debugging
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_uppercase().as_str() {
            "HELP" => Ok(Self::Help),
            "BEGIN" => Ok(Self::Begin),
            "END" => Ok(Self::End),
            "RUN" => Ok(Self::Run),
            "DBG" | "DEBUG" => Ok(Self::Debug),
            "EXIT" => Ok(Self::Exit),
            "SHOW" => Ok(Self::Show),
            "CLEAR" => Ok(Self::Clear),
            "SET" => Ok(Self::Set),
            "N" | "NEXT" => Ok(Self::Next),
            "NO" | "NEXTOUT" => Ok(Self::NextOut),
            "D" | "DMP" | "DUMP" => Ok(Self::Dump),
            "O" | "OUT" => Ok(Self::Out),
            "ED" | "ENDDEBUG" => Ok(Self::EndDebug),
            _ => Err(()),
        };
    }
}
