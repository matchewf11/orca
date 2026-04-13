module Token where

data Token
    = Ident String
    | Plus
    | Minus
    | Assign
    | Semicolon
    | IntLit Integer
    | Boolean Bool
    deriving (Eq, Show)

lookupIdent :: String -> Token
lookupIdent "true" = Boolean True
lookupIdent "false" = Boolean False
lookupIdent s = Ident s
