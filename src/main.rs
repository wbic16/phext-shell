use std::{fs, io::Write};
use libphext::phext;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct PhextShellState
{
    pub filename:String,
    pub coordinate:phext::Coordinate,
    pub status:bool,
    pub phext:String,
    pub scroll:String,
    pub history:String
}

// -----------------------------------------------------------------------------------------------------------
// @fn main
// -----------------------------------------------------------------------------------------------------------
fn main() {
    let mut state:PhextShellState = PhextShellState {
        filename: String::new(),
        coordinate: phext::to_coordinate("1.1.1/1.1.1/1.1.1"),
        status: false,
        phext: String::new(),
        scroll: String::new(),
        history: String::new()
    };

    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        let command = args[1].clone();
        let request = args[1..].join(" ");
        handle_request(request, &mut state);

        if command.starts_with("help") {
            return;
        }
    }

    while state.status == false {
        let mut display_coordinate = state.coordinate.to_string();
        while display_coordinate.starts_with("1.1.1/") {
            display_coordinate = display_coordinate[6..].to_string();
        }
        print!("{} > ", display_coordinate);
        std::io::stdout().flush().expect("output error");

        let mut request = String::new();
        let total = std::io::stdin().read_line(&mut request).expect("Failed to read line");

        if total == 0 { continue; }

        handle_request(request, &mut state);
    }

    let filename = state.filename + ".history";
    let error_message = format!("Unable to save session history to {}", filename);
    fs::write(filename.clone(), state.history.as_bytes()).expect(error_message.as_str());
}

// -----------------------------------------------------------------------------------------------------------
// @fn handle_request
// -----------------------------------------------------------------------------------------------------------
fn handle_request(request: String, state:&mut PhextShellState) {
    let trimmed = request.trim();
    let (command, args) = trimmed.split_once(' ').unwrap_or((trimmed, ""));
    
    let mut should_dump_scroll = false;

    let prior_history = phext::fetch(state.history.as_str(), state.coordinate);
    let updated_history = prior_history + "\n" + trimmed;
    state.history = phext::replace(state.history.as_str(), state.coordinate, updated_history.as_str());

    match command {
        // exit: terminate the shell session
        // quit: synonym
        // :q! because VIM is awesome
        // (TODO) Ctrl-z: thanks, python
        "exit" | "quit" | ":q!" => state.status = true,

        // af: append file to the current coordinate
        "af" => {
            if args.len() < 1 {
                println!("Expected 1 argument");
            } else {
                let filename = args;
                match fs::read_to_string(filename) {
                    Ok(content) => {
                        let update = phext::fetch(state.phext.as_str(), state.coordinate) + content.as_str();
                        state.phext = phext::replace(state.phext.as_str(), state.coordinate, update.as_str());
                        println!("Appended {}", filename);
                        println!("");
                        println!("{}", update.as_str());
                    },
                    Err(e) => println!("Error reading file '{}': {}", filename, e)
                }
            }
        },

        // cs: change scroll
        "cs" => {
            if args.len() < 1 {
                println!("Location: {}", state.coordinate);
            }
            else {
                let address = args;
                state.coordinate = phext::to_coordinate(&address);
                state.status = false;
                if state.phext.is_empty() == false {
                    state.scroll = phext::fetch(state.phext.as_str(), state.coordinate);
                    should_dump_scroll = true;
                }
            }

        },

        // ds: display scroll
        "ds" => {
            state.scroll = phext::fetch(state.phext.as_str(), state.coordinate);
            should_dump_scroll = true;
        },

        // pi: phext index
        "pi" => {
            let index = phext::index(state.phext.as_str());
            println!("{}", phext::textmap(index.as_str()));
            let filename = state.filename.clone() + ".index";
            match fs::write(filename.clone(), index.as_bytes()) {
                Ok(()) => (),
                Err(e) => println!("Unable to locate {}: {}", filename, e)
            }
        },
        
        // ps: phext soundex
        "ps" => {
            let soundex = phext::soundex_v1(state.phext.as_str());
            println!("{}", phext::textmap(soundex.as_str()));
            let filename = state.filename.clone() + ".soundex";
            match fs::write(filename.clone(), soundex.as_bytes()) {
                Ok(()) => (),
                Err(e) => println!("Unable to locate {}: {}", filename, e)
            }
        },
        
        // ph: phext hash
        "ph" => {
            let manifest = phext::manifest(state.phext.as_str());
            let filename = state.filename.clone() + ".checksum";

            match fs::write(filename.clone(), manifest.as_bytes()) {
                Ok(()) => (),
                Err(e) => println!("Unable to locate {}: {}", filename, e)
            }

            let checksum = phext::checksum(manifest.as_str());
            println!("Checksum: {} ({}).", checksum, filename);
        },

        // lp: open phext
        "lp" => {
            if args.len() < 1 {
                println!("Location: {}", state.coordinate);
            }
            else {
                state.filename = args.to_string();
                if std::path::Path::new(&state.filename).exists() {
                    match fs::read_to_string(state.filename.clone()) {
                        Ok(content) => {
                            state.phext = content;
                            state.scroll = phext::fetch(state.phext.as_str(), state.coordinate);
                            println!("{}", phext::textmap(state.phext.as_str()));
                        },
                        Err(e) => println!("Unable to locate {}: {}", state.filename, e)
                    }
                } else {
                        println!("No file for {} found. Initializing an empty phext...", state.filename);
                        state.phext = String::new();
                        state.scroll = String::new();
                }
            }
        },


        // os: overwrite
        // if no text is provided, should default behavior be reset scroll?
        "os" => {
            if trimmed.len() > 3 {
                state.phext = phext::replace(state.phext.as_str(), state.coordinate, &trimmed[3..]);
            } else {
                state.phext = phext::replace(state.phext.as_str(), state.coordinate, "");
            }
            state.scroll = phext::fetch(state.phext.as_str(), state.coordinate);
            should_dump_scroll = true;
        }

        // rp: deploy the ion cannon and clear the entire phext
        "rp" => {
            state.phext = String::new();
            state.scroll = String::new();
            should_dump_scroll = true;
        },
    
        // rs: reset scroll
        "rs" => {
            state.phext = phext::replace(state.phext.as_str(), state.coordinate, "");
            state.scroll = phext::fetch(state.phext.as_str(), state.coordinate);
            should_dump_scroll = true;
        },

        // sp: save phext
        "sp" => {
            if args.len() < 1 {
                println!("Expected 1 argument");
            } else {
                let filename = args;
                match fs::write(filename, state.phext.as_bytes()) {
                    Ok(()) => println!("Saved {}.", filename),
                    Err(e) => println!("Unable to locate {}: {}", filename, e)
                }
            }
        },
        
        // help: display hints for the user
        "help" => {
            show_help(args);
        },
        
        "run" => {
            let (run_command, run_args) = args.split_once(' ').unwrap_or((args, ""));
            println!("Executing '{}'...", args);

            use std::process::Command;
            match Command::new(run_command)
                .args(run_args.split_whitespace())
                .output() {
                Ok(output) => {
                    let program_output = String::from_utf8_lossy(&output.stdout).to_string();
                    state.phext = phext::replace(state.phext.as_str(), state.coordinate, program_output.as_str());
                    state.scroll = phext::fetch(state.phext.as_str(), state.coordinate);
                    println!("Collected {} bytes into {}", program_output.len(), state.coordinate);
                    if output.stderr.len() > 0 {
                        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
                    }
                },
                Err(e) => println!("Failed to execute process: {}", e)
            }
        }

        _ => {
            println!("Unknown command '{}'", command);
        }
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

    if area.starts_with("lp") {
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
    println!(" * af: Appends the contents of a File to the current scroll");
    println!(" * cs: Change Scroll: sets your current coordinate and displays any data found in the current phext");
    println!(" * ds: Displays the current Scroll");
    println!(" * lp: loads a phext from disk, allowing you to explore it via `cs` commands");
    println!(" * rp: Resets the current Phext");
    println!(" * rs: Resets the current Scroll");
    println!(" * os: Overwrites the current Scroll with text");
    println!(" * ph: computes the xxh3-based manifest of your phext");
    println!(" * pi: computes the index of your phext");
    println!(" * ps: computes the soundex of your phext");
    println!(" * sp: saves the current phext to disk in the file specified");
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
