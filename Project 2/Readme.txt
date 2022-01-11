Step 1 Read the file --- 
by using std::fs::File & std::io::prelude::*; library

In rust we cannot read file line by line we need to read the whole file together and store it in vectors of string and then 
read it character by character

Step 2 
Toeknizer (lexical analyzer) 

for every character check if it is an identifier, keyword, expression, or variable by using token.rs and mod.rs file

Step 3 
Parse (syntactic analysis)
check  for any syntax error 

Step 4 Genertaing XHTML file and highlighting functions according format.csv file

Step 5 You may need to execute "rustup default nightly" (in the terminal) for it to work

Step 6 Run "cargo run" from terminal from the parser/src directory



                                  References
-----------------------------------------------------------------------------------------
=>https://michael-f-bryan.github.io/static-analyser-in-rust/book/lex.html
=>https://youtu.be/4m7ubrdbWQU
=>https://adriann.github.io/rust_parser.html
=>https://github.com/mohitk05/monkey-rust
