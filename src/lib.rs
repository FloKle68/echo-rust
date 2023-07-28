const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP: &str = "Synopsis
echo [SHORT-OPTION]... [STRING]...
echo LONG-OPTION

Description
Echo the STRING(s) to standard output.
-n : do not output the trailing newline 
-e : enable interpretation of backslash escapes 
-E : disable interpretation of backslash escapes (default) 
--help : display this help and exit 
--version : output version information and exit

If -e is in effect, the following sequences are recognized:
\\\\ : backslash 
\\n : new line 
\\r : carriage return 
\\t : horizontal tab";



pub fn echo<I>(args: I) 
where I: Iterator<Item = String>
{
    let mut new_line = true;
    let mut fin_params = false;
    let mut interpretation = false;

    for arg in args.skip(1) {
        if !fin_params {
            match arg.as_str() {
                "-n" => new_line = false,
                "-e" => interpretation = true,
                "-E" => interpretation = false,
                "--help" => println!("{HELP}"),
                "--version" => println!("{VERSION}"),
                _ => fin_params = true,
            }
        }

        if fin_params {
            let mut texte: String = arg;
            if interpretation {
                texte = texte.replace("\\\\", "\\");
                texte = texte.replace("\\n", "\n"); 
                texte = texte.replace("\\r", "\r"); 
                texte = texte.replace("\\t", "\t");
            }

            print!("{}", texte);
            if new_line {println!()}
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use super::HELP;

    fn run(args: &[&str], attendu: &str) {
        let expected = attendu.to_string();
        Command::cargo_bin("echo-rust")
                    .unwrap()
                    .args(args)
                    .assert()
                    .success()
                    .stdout(expected);
    }

    #[test]
    fn it_works() {
        run(&["It works!"], "It works!\n")
    }

    #[test]
    fn it_works_no_new_line() {
        run(&["-n", "It works!"], "It works!")
    }

    #[test]
    fn version() {
        run(&["--version"], "1.0.0\n")
    }

    #[test]
    fn help() {
        run(&[HELP], (HELP.to_owned() + "\n").as_str());
    }

    #[test]
    fn interpreation_backslash() {
        run(&["-e", "-n", "\\\\"], "\\");
    }
    
    #[test]
    fn interpreation_n() {
        run(&["-e", "-n", "\\n"], "\n");
    }
    
    #[test]
    fn interpreation_r() {
        run(&["-e", "-n", "\\r"], "\r");
    }
    
    #[test]
    fn interpreation_t() {
        run(&["-e", "-n", "\\t"], "\t");
    }    
}