module Error where

type Result a = Either Error a

data Error
    = LexError String
    | ParseError String
    deriving (Show, Eq)
