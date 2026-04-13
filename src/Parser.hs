module Parser where

import Error
import Ast
import Token

parseTok :: [Token] -> Result Program
parseTok [] = Right $ Program []
parseTok _ = Left $ ParseError "Not Implemented"
