type t =
  | Illegal
  | Eof
  | Ident of string
  | Int of string
  | Assign
  | Plus
  | Minus
  | Bang
  | Asterisk
  | Slash
  | Lt
  | Gt
  | Eq
  | NotEq
  | Comma
  | Semicolon
  | LParen
  | RParen
  | LBrace
  | RBrace
  | Function
  | Let
  | True
  | False
  | If
  | Else
  | Return
