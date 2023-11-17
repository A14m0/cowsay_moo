use clap::Parser;
use divrem::DivRem;
use std::path::PathBuf;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path for the cow to say
    #[arg(short, long)]
    file: Option<PathBuf>,
    
    /// Path to a file containing a custom cow
    #[arg(short, long)]
    cow_path: Option<PathBuf>,

    /// Prints the program version
    #[arg(short, long)]
    version: bool
}

fn get_cow(cow_path: Option<PathBuf>) -> String {
        // default cow :)
        let default_cow = "       \\   ^__^
        \\  (oo)\\_______
           (__)\\       )\\/
               ||----w |
               ||     ||
".to_string();


    if cow_path.is_some() {
        // we have a custom cow, lets throw him in
        let mut tmp_cow_lines: Vec<String> = Vec::new();
        let file_lines = match std::fs::read_to_string(cow_path.unwrap()) {
            Ok(a) => a,
            Err(_) => return default_cow
        };
        for line in file_lines.lines() {
            if tmp_cow_lines.len() == 0 {
                let mut tmp_line = "   \\ ".to_string();
                tmp_line.push_str(line);
                tmp_line.push('\n'); 
                tmp_cow_lines.push(tmp_line);
            } else if tmp_cow_lines.len() == 1 {
                let mut tmp_line = "    \\".to_string();
                tmp_line.push_str(line); 
                tmp_line.push('\n');
                tmp_cow_lines.push(tmp_line);
            } else {
                let mut tmp_line = "     ".to_string();
                tmp_line.push_str(line); 
                tmp_line.push('\n');
                tmp_cow_lines.push(tmp_line);
            }
        }

        let mut return_cow = String::new();
        for line in tmp_cow_lines {
            return_cow.push_str(&line);
        }

        return_cow
    } else {
        default_cow
    }
}


fn moo(input: Vec<String>, cow: String) {


    let (terminal_size::Width(width), _) = terminal_size::terminal_size().unwrap();

    let mut max_len: usize = 0;
    for line in input.iter() {
        if line.len() > max_len{
            max_len = line.len();
        }
    }

    if max_len + 4 > width as usize {
        max_len = width as usize - 4;
    }

    print!(" ");
    for _ in 0..max_len+2 {
        print!("_");
    }
    println!(" ");

    print!("/");
    for _ in 0..max_len+2 {
        print!(" ");
    }
    println!("\\");

    for line in input.iter() {
        let tlen = line.len();

        // minus 4 for two | and two spaces
        if tlen > max_len {
            let loops_req = tlen.div_rem(max_len);
            //println!("Loops: {:?} (max {})", loops_req, max_len);

            let mut start = 0;
            let mut end = max_len;
            for _ in 0..loops_req.0 + 1 {
                let current_data = (&line[start..end]).to_string();
                let llen = current_data.len();
//                println!("LLEN: {llen}, TLEN: {tlen}, MAX_LEN: {max_len}, start: {start}, end: {end}");

                print!("| {}", current_data);

                for _ in 0..(max_len-llen +1) {
                    print!(" ");
                }
                println!("|");
                start = end;
                if start + max_len > tlen {
                    end += loops_req.1;
                } else {
                    end += max_len;
                }
            }

        } else {

            print!("| {}", line);

            for _ in 0..(max_len - tlen +1) {
                print!(" ");
            }
            println!("|");
        }

    }

    print!("\\");
    for _ in 0..max_len+2 {
        print!("_");
    }
    println!("/");

    println!("{}", cow);

}




fn main() {

    let args = Args::parse();

    let mut result = Vec::new();
        

    if args.version {
        result.push(format!("cowsay_moo version {}", clap::crate_version!()))
    } else {
        if args.file.is_some() {
            let args_file = args.file.unwrap();
            let file_string = match std::fs::read_to_string(args_file) {
                Ok(a) => a,
                Err(e) => format!("Failed to open your text file!\nReason: {}", e)
            };
            
            for line in file_string.lines() {
                result.push(line.to_string())
            }
        } else {
            result.push(format!("Please provide a text file!"));
        }

    }

    
    let cow = get_cow(args.cow_path);

    moo(result, cow);

}
