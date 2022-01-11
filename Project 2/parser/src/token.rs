pub enum TypeTokens {
    NONE,
	INTCONSTANT,
	FLOATCONSTANT,
    OPERATOR,
    KEYWORD,
    VARIABLE,
	FUNCTION,
    INVALID
}

impl TypeTokens {
    pub fn as_str(&self) -> &'static str {
        match &self {
            TypeTokens::NONE => "None",
            TypeTokens::INTCONSTANT => "IntConstant",
            TypeTokens::FLOATCONSTANT => "FloatConstant",
            TypeTokens::OPERATOR => "Operator",
            TypeTokens::KEYWORD => "Keyword",
            TypeTokens::VARIABLE => "Variable",
            TypeTokens::FUNCTION => "Function",
            TypeTokens::INVALID => "Invalid"
        }   
    }   
}



pub struct Token {
    text: String,
    token_type: TypeTokens,
    line_number: i32,
    char_pos: i32 
}

impl Token {
    pub fn new(s: String, t: TypeTokens, linenum: i32, charpos: i32) -> Token {
        Token {
            text: s,
            token_type: t,
            line_number: linenum,
            char_pos: charpos
        }   
    }   

    pub fn get_text(&self) -> &str {
        &self.text
    }   

    pub fn get_type(&self) -> &TypeTokens {
        &self.token_type
    }   

    pub fn get_line_number(&self) -> i32 {
        self.line_number
    }   

    pub fn get_char_pos(&self) -> i32 {
        self.char_pos
    }   
}
