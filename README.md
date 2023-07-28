# ECHO-RUST

Implémentation de la commande linux "echo" en rust pour entraînement.

Tout  les caractères d'échappements ne sont pas gérés, uniquement \\, \n,\r et \t.

## Utilisation
 ```bash
 cargo run -- "Hello world!"
 >Hello world!

 cargo run -- "Hello " "world!"
 >Hello 
 >world!

 cargo run -- -n "Hello " "world!"
 >Hello world!

 cargo run -- --version
 >1.0.0


 cargo run -- --help
 >Synopsis
 >echo [SHORT-OPTION]... [STRING]...
 >echo LONG-OPTION
 >
 >Description
 >Echo the STRING(s) to standard output.
 >-n : do not output the trailing newline 
 >-e : enable interpretation of backslash escapes 
 >-E : disable interpretation of backslash escapes (default) 
 >--help : display this help and exit 
 >--version : output version information and exit
 >
 >If -e is in effect, the following sequences are recognized:
 >\\\\ : backslash 
 >\\n : new line 
 >\\r : carriage return 
 >\\t : horizontal tab
 ```