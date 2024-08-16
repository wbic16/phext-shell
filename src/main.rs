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
    let mut handled = false;

    // exit: terminate the shell session
    // quit: synonym
    // :q! because VIM is awesome
    if trimmed.starts_with("exit") ||
       trimmed.starts_with("quit") ||
       trimmed.starts_with(":q!") {
        state.status = true;
        handled = true;
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
        handled = true;
    }

    // vex: open phext
    let vex_command = "vex ";
    if trimmed.starts_with(vex_command) {
        let filename = trimmed[vex_command.len()..].to_owned();
        let error_message = format!("Unable to locate {}", filename);
        state.phext = fs::read_to_string(filename).expect(error_message.as_str());
        state.scroll = phext::fetch(state.phext.as_str(), state.coordinate);
        println!("{}", phext::textmap(state.phext.as_str()));
        handled = true;
    }

    if handled == false && trimmed.starts_with("help") == false {
        println!("Error: unknown command '{}'. Displaying built-in help...", trimmed);
        println!("");
    }

    // help: display hints for the user
    if trimmed.starts_with("help") || handled == false {
        let mut help_request = "";
        if trimmed.len() > 5 {
            help_request = &trimmed[5..];
        }
        show_help(help_request);
    }

    if should_dump_scroll {
        println!("{}", state.scroll);
    }
}

fn show_help(area: &str) {
    let version = env!("CARGO_PKG_VERSION");
    println!("phext-shell v{}", version);

    let lowercase = area.to_ascii_lowercase();
    let area = lowercase.as_str();

    if area.starts_with("vex") {
        println!("summary: vex parses a phext from your local file system.");
        println!("example: `vex <file>`");
        println!("");
        println!("The vex command loads the contents of the given file into memory.");
        println!("This makes it available for use with other commands, such as cs (change scroll).");
        return;
    }

    if area.starts_with("cs") {
        println!("summary: cs changes your current coordinate and dumps state to the screen");
        println!("example: `cs 50.14.88/25.23.17/8.6.4`");
        println!("");
        println!("The cs command instructs phext-shell to navigate to the specified coordinate.");
        println!("If you are currently vexing a phext, the scroll at your request coordinate will be displayed.");        
        return;
    }

    if area.starts_with("coordinate") {
        println!("concept: Coordinate");
        println!("");
        println!("Phext coordinates assist you with navigating subspace buffers using a 9-dimensional space. Each dimension has a name associated with it, purely for aesthetic reasons. The format of a phext coordinate is of the form: <LB>.<SF>.<SR>/<CN>.<VM>.<BK>/<CH>.<SN>.<SC>.");
        println!("");
        println!(" * LB: Library - the first digit");
        println!(" * SF: Shelf - the second digit");
        println!(" * SR: Series - the third digit");
        println!(" * CN: Collection - the fourth digit");
        println!(" * VM: Volume - the fifth digit");
        println!(" * BK: Book - the sixth digit");
        println!(" * CH: Chapter - the seventh digit");
        println!(" * SN: Section - the eighth digit");
        println!(" * SC: Scroll - the ninth digit");
        println!("");
        println!("For a more in-depth understanding of the phext encoding, refer to https://github.com/wbic16/libphext-rs.");
        return;
    }

    if area.starts_with("delimiter") {
        println!("Concept: delimiters of unusual size enable text compression.");
        println!("");
        println!("Phexts are just text designed for the 22nd century. By extending the process of encoding text into a 1D buffer, phext gives us a blueprint for hierarchical digital memory.");
        println!("Whenever a delimiter is encountered, it causes the reader to re-evaluate the current coordinate.");
        println!("");
        println!("Let's start small, with a normal line break. Upon encountering a line break, our column counter resets to 1 and our line counter increments by 1.");
        println!("");
        println!("Line 1<LINE-BREAK>Line 2 -- The text 'Line 2' starts at Column 1, Line 2.");
        println!("");
        println!("We will apply this logic recursively to arrive at a natural intution for how phext works.");
        println!("");
        println!("Upon encountering a scroll break, we'll reset our line and column counters to 1, and advance our scroll counter. This is the right-most coordinate in a phext address.");
        println!("");
        println!("Scroll 1<SCROLL-BREAK>Scroll 2 -- The text 'Scroll 2' starts at Column 1, Line 1, Scroll 2");
        println!("");
        println!("Phext continues this progression, allowing you to encapsulate 8 additional layers - forming an 11D space overall. A summary of coordinate transformation rules is given below.");
        println!("");
        println!("Delimiter Type    LB  SF  SR   CN  VM  BK   CH  SN  SC  Line  Column");
        println!("--------------    --  --  --   --  --  --   --  --  --  ----  ------");
        println!("Line Break                                               +1   =1");
        println!("Scroll Break                                        +1   =1   =1");
        println!("Section Break                                   +1  =1   =1   =1");
        println!("Chapter Break                               +1  =1  =1   =1   =1");
        println!("Book Break                             +1   =1  =1  =1   =1   =1");
        println!("Volume Break                       +1  =1   =1  =1  =1   =1   =1");
        println!("Collection Break               +1  =1  =1   =1  =1  =1   =1   =1");
        println!("Series Break              +1   =1  =1  =1   =1  =1  =1   =1   =1");
        println!("Shelf Break           +1  =1   =1  =1  =1   =1  =1  =1   =1   =1");
        println!("Library Break     +1  =1  =1   =1  =1  =1   =1  =1  =1   =1   =1");
        return;
    }

    if area.starts_with("exocortex") {
        println!("Concept: Exocortex - the next stage of neural evolution.");
        println!("");
        println!("We are building a global brain. Phext is designed to scale planet-wide, enabling collaboration at scale.");
        println!("");
        return;
    }

    if area.starts_with("phext") {
        println!("Phext is plain hypertext - hierarchical digital memory for the 22nd century.");
        println!("");
        println!("At the core, phext is just normal plain utf8 text. The introduction of delimiters of unusual size provide you with exocortical powers.");
        println!("");
        println!("Be sure to check out the #phext hashtag on twitter/X for more info.");
        println!("Contact me at https://x.com/wbic16 with any questions.");
        return;
    }

    if area.starts_with("subspace") {
        println!("Concept: Subspace - the plain text substrate that enables phext.");
        println!("");
        println!("Phext can be manipulated as a DAG of scrolls, or you can just edit it directly via subspace.");
        println!("");
        println!("Note: Subspace is a direct nod to Star Trek. Live long, and prosper, friends. :)");
        return;
    }

    println!("");
    println!("Welcome to Phext! This cli tool gives you exocortical powers.");
    println!("Phexts are composed of plain text separated by hierarchical delimiters.");
    println!("You can ask for additional help about the commands and concepts listed below.");
    println!("");
    println!("Available Commands");
    println!("------------------");
    println!(" * vex: loads a phext from disk, allowing you to explore it via `cs` commands");
    println!(" * cs: Change Scroll: sets your current coordinate and displays any data found in the current phext");
    println!("");
    println!("Concepts");
    println!("--------");
    println!(" * coordinate: Phext coordinates provide a 9D space to explore subspace with");
    println!(" * delimiter: A collection of 10 delimiter types provide us with 11D phext");
    println!(" * exocortex: our global brain");
    println!(" * phext: plain hypertext - hierarchical digital memory");
    println!(" * subspace: the 1D buffer that encodes phexts");
    println!("");
}