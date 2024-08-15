use std::{fs, io::Write};

use libphext::phext;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct PhextShellState
{
    pub coordinate:phext::Coordinate,
    pub status:bool,
    pub phext:String,
    pub scroll:String
}

// -----------------------------------------------------------------------------------------------------------
// @fn main
// -----------------------------------------------------------------------------------------------------------
fn main() {
    let mut state:PhextShellState = PhextShellState {
        coordinate: phext::to_coordinate("1.1.1/1.1.1/1.1.1"),
        status: false,
        phext: String::new(),
        scroll: String::new()
    };

    while state.status == false {
        print!("{} > ", state.coordinate);
        std::io::stdout().flush().expect("output error");

        let mut request = String::new();
        let total = std::io::stdin().read_line(&mut request).expect("Failed to read line");
        
        if total == 0 { continue; }

        handle_request(request, &mut state);
    }
}

// -----------------------------------------------------------------------------------------------------------
// @fn handle_request
// -----------------------------------------------------------------------------------------------------------
fn handle_request(request: String, state:&mut PhextShellState) {
    let trimmed = request.trim();
    let mut should_dump_scroll = false;

    // exit: terminate the shell session
    // quit: synonym
    // :q! because VIM is awesome
    if trimmed.starts_with("exit") ||
       trimmed.starts_with("quit") ||
       trimmed.starts_with(":q!") {
        state.status = true;
    }

    // cs: change scroll
    let cs_command = "cs ";
    if trimmed.starts_with(cs_command) {
        let address = trimmed[cs_command.len()..].to_owned();
        state.coordinate = phext::to_coordinate(&address);
        state.status = false;
        if state.phext.is_empty() == false {
            state.scroll = phext::fetch(state.phext.as_str(), state.coordinate);
            should_dump_scroll = true;
        }
    }

    // vex: open phext
    let vex_command = "vex ";
    if trimmed.starts_with(vex_command) {
        let filename = trimmed[vex_command.len()..].to_owned();
        let error_message = format!("Unable to locate {}", filename);
        state.phext = fs::read_to_string(filename).expect(error_message.as_str());
        state.scroll = phext::fetch(state.phext.as_str(), state.coordinate);
        println!("{}", phext::textmap(state.phext.as_str()));
    }

    if should_dump_scroll {
        println!("{}", state.scroll);
    }
}