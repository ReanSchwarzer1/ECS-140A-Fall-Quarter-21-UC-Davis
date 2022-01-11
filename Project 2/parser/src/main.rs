use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
mod lexer;
mod data;

fn main() {
    // reading example1.x file-------------------
    let mut file= File::open("input.x").expect("something went wrong");
    let mut contents=String::new();
    file.read_to_string(&mut contents).expect("can not read file!");

  //lexical analyser 
    let mut lex = lexer::Lex::new(contents.chars().collect());
    lex.char_read();
    loop {
        let token = lex.token_next();
        if token == lexer::token::Tkn::EOF {
            break;
        } else {
            println!("{:?}", token);
        }
    }
//creating output xhtml file
    let path = Path::new("example1.xhtml");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(data::DATA.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    println!("{} {} {}", char::from(lex.character), lex.pos, lex.pos_read);
}
