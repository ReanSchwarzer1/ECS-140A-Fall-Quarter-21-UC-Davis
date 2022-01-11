extern crate phf;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Tkn {
    INVALID,EOF,IDENT(Vec<char>),INT(Vec<char>),FLOAT(Vec<char>),
    MAIN(Vec<char>), VOID(Vec<char>),
    ASSIGN(char),PLUS(char),COMMA(char),SEMICOLON(char),
    LPAREN(char),RPAREN(char),LBRACE(char),RBRACE(char),
    FUNCTION,LET,TRUE,FALSE,IFStatement,ELSEStatement,
    WHILELOOP,RETURNStatement,MINUS(char),BANG(char),
    ASTERISK(char),SLASH(char),LT(char),GT(char),
    FLOATCONSTANT,DOUBLECONSTANT,LONGCONSTANT,INTCONSTANT,SHORTCONSTANT
}

pub fn get_token(ident: &Vec<char>) -> Result<Tkn, String> {
    let identifier: String = ident.into_iter().collect();
    match &identifier[..] {
        "float" => Ok(Tkn::FLOATCONSTANT),
        "double" => Ok(Tkn::DOUBLECONSTANT),
        "long" => Ok(Tkn::LONGCONSTANT),
        "short" => Ok(Tkn::SHORTCONSTANT),
        "int" => Ok(Tkn::INTCONSTANT),
        "fn" => Ok(Tkn::FUNCTION),
        "let" => Ok(Tkn::LET),
        "true" => Ok(Tkn::TRUE),
        "false" => Ok(Tkn::FALSE),
        "if" => Ok(Tkn::IFStatement),
        "while" => Ok(Tkn:: WHILELOOP),
        "else" => Ok(Tkn::ELSEStatement),
        "return" => Ok(Tkn::RETURNStatement),
        _ => Err(String::from("Not a keyword"))
    }
}