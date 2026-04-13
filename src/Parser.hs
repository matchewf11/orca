module Parser where

import Error
import Ast
import Token

data Precedence
    = Lowest
    | Sum
    | Call

parseProg :: [Token] -> Result Program
parseProg tok = Program <$> parseProgAux [] tok

parseProgAux :: [Statement] -> [Token] -> Result [Statement]
parseProgAux acc [] = Right acc
parseProgAux acc xs = parseStmt xs >>= (\(stmt, rest) -> parseProgAux (stmt:acc) rest)

-- this is either a binding or a expression stmt?
-- [<ident>(non-zero list)] = <expr>;
-- <expr>;
--
-- assume this is a bind then if it fails, try expression
parseStmt :: [Token] -> Result (Statement, [Token])
parseStmt _ = Left $ ParseError "parseStmt Not Implemented"

parseExpr :: Precedence -> [Token] -> Result (Expression, [Token])
parseExpr _ _ = Left $ ParseError "parseExpr Not Implmented"
