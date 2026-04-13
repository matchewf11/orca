module Parser where

import Error
import Ast
import Token

parseTok :: [Token] -> Result [Statement]
parseTok _ = Left $ ParseError "Not Implemented"
