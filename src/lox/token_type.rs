// thought about trying to nest by category but seems like a useless overhead. Clean pattern
// matching will be made, if I need separate category to get some cool traits behaviour, I will
// update this at this time
enum TokenType {
    // Single char token
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    
    // One or two char tokens
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This, True, Var, While,

    EOF
}
