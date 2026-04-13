module Parser where

import Error
import qualified Ast as A
import qualified Token as T

parseExpr :: [T.Token] -> Result (A.Expression, [T.Token])
parseExpr _ = Left $ LexError "foo"
parseExpr _ = Left $ LexError "foo"
