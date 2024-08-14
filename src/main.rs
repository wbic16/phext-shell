use std::io::Write;

use libphext::phext;

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
struct PhextShellResult
{
    pub coordinate:phext::Coordinate,
    pub status:bool
}

// -----------------------------------------------------------------------------------------------------------
// @fn main
// -----------------------------------------------------------------------------------------------------------
fn main() {    
    let mut done = false;
    let mut user_coordinate = phext::to_coordinate("1.1.1/1.1.1/1.1.1");

    while done == false {
        print!("{} > ", user_coordinate);
        std::io::stdout().flush().expect("output error");

        let mut request = String::new();
        let total = std::io::stdin().read_line(&mut request).expect("Failed to read line");
        
        if total == 0 { continue; }

        let result = handle_request(request, user_coordinate);
        done = result.status;
        user_coordinate = result.coordinate;
    }
}

// -----------------------------------------------------------------------------------------------------------
// @fn handle_request
// -----------------------------------------------------------------------------------------------------------
fn handle_request(request: String, incoming_coordinate:phext::Coordinate) -> PhextShellResult {
    let mut result = PhextShellResult { status: false, coordinate: incoming_coordinate };
    let trimmed = request.trim();

    // exit: terminate the shell session
    if trimmed.starts_with("exit") {
        result.status = true;
    }

    // cs: change scroll
    if trimmed.starts_with("cs ") {
        let address = trimmed[2..].to_owned();
        result.coordinate = phext::to_coordinate(&address);
        result.status = false;
    }

    return result;
}