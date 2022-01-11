pub mod token;

pub struct Lex {
    input: Vec<char>,
    pub pos: usize,
    pub pos_read: usize,
    pub character: char,
}

fn is_float(character: char) -> bool {('0' <= character && character <= '9') || (character =='.')}
fn is_main(character: char) -> bool {'m'; 'a'; 'i'; 'n' == character}
fn is_void(character: char) -> bool {'v'; 'o'; 'i'; 'd' == character}
fn is_let(character: char) -> bool {'a' <= character && character <= 'z' || 'A' <= character && character <= 'Z' || character == '_'}
fn is_dig(character: char) -> bool {'0' <= character && character <= '9'}



impl Lex {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            pos: 0,
            pos_read: 0,
            character: '0',
        }
    }


    pub fn token_next(&mut self) -> token::Tkn {
        self.skip_whtspc();

        let float_read = |l: &mut Lex| -> Vec<char> {
            let pos = l.pos;
            while l.pos < l.input.len() && is_float(l.character) {
                l.char_read();
            }
            l.input[pos..l.pos].to_vec()
        };

        let iden_read = |l: &mut Lex| -> Vec<char> {
            let pos = l.pos;
            while l.pos < l.input.len() && is_let(l.character) {
                l.char_read();
            }
            l.input[pos..l.pos].to_vec()
        };

        let main_read = |l: &mut Lex| -> Vec<char> {
            let pos = l.pos;
            while l.pos < l.input.len() && is_main(l.character) {
                l.char_read();
            }
            l.input[pos..l.pos].to_vec()
        };

        let void_read = |l: &mut Lex| -> Vec<char> {
            let pos = l.pos;
            while l.pos < l.input.len() && is_void(l.character) {
                l.char_read();
            }
            l.input[pos..l.pos].to_vec()
        };

        let num_read = |l: &mut Lex| -> Vec<char> {
            let pos = l.pos;
            while l.pos < l.input.len() && is_dig(l.character) {
                l.char_read();
            }
            l.input[pos..l.pos].to_vec()
        };

        let t: token::Tkn;
        self.skip_whtspc();
        
        match self.character {
            '=' => {t = token::Tkn::ASSIGN(self.character);},
            '+' => {t = token::Tkn::PLUS(self.character);},
            '-' => {t = token::Tkn::MINUS(self.character);},
            '!' => {t = token::Tkn::BANG(self.character);},
            '/' => {t = token::Tkn::SLASH(self.character);},
            '*' => {t = token::Tkn::ASTERISK(self.character);},
            '<' => {t = token::Tkn::LT(self.character);},
            '>' => {t = token::Tkn::GT(self.character);},
            ';' => {t = token::Tkn::SEMICOLON(self.character);},
            '(' => {t = token::Tkn::LPAREN(self.character);},
            ')' => {t = token::Tkn::RPAREN(self.character);},
            ',' => {t = token::Tkn::COMMA(self.character);},
            '{' => {t = token::Tkn::LBRACE(self.character);},
            '}' => {t = token::Tkn::RBRACE(self.character);},
            '0' => {t = token::Tkn::EOF;},
            ' ' => {self.char_read(); t = token::Tkn::EOF;}
            _ => {
                if is_let(self.character) {
                    let ident: Vec<char> = iden_read(self);
                    match token::get_token(&ident) {
                        Ok(keywork_token) => {
                            return keywork_token;
                        },
                        Err(_err) => {
                            return token::Tkn::IDENT(ident);
                        }
                    }
                } 
                else if is_float(self.character) {let ident: Vec<char> = float_read(self); return token::Tkn::FLOAT(ident);}
                else if is_main(self.character) {let ident: Vec<char> = main_read(self); return token::Tkn::MAIN(ident);}
                else if is_void(self.character) {let ident: Vec<char> = void_read(self); return token::Tkn::VOID(ident);}
                else if is_dig(self.character) {let ident: Vec<char> = num_read(self); return token::Tkn::INT(ident);}
                else {self.char_read(); return token::Tkn::INVALID}
            }
        }
        self.char_read();
        t
    }

    pub fn char_read(&mut self) {
       
        if self.pos_read >= self.input.len() {
            self.character = '0';
        } else {
            self.character = self.input[self.pos_read];
        }
        self.pos = self.pos_read;
        self.pos_read = self.pos_read + 1;
    }
    /*
    pub fn string_read(&mut self) {
       
        if self.pos_read >= self.input.len() {
            self.character = '0';
        } else {
            self.character = self.input[self.pos_read];
        }
        self.pos = self.pos_read;
        self.pos_read = self.pos_read + 1;
    }
    */

    pub fn skip_whtspc(&mut self) {
        let character = self.character;
        if character == ' ' || character == '\t' || character == '\n' || character == '\r' {
            self.char_read();
        }
    }
}
