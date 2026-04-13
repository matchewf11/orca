module Lexer where

import Token
import Error
import Data.Char

lexString :: String -> Result [Token]
lexString [] = Right []
lexString (' ':xs) = lexString xs
lexString ('=':xs) = (Assign:) <$> lexString xs
lexString ('+':xs) = (Plus:) <$> lexString xs
lexString ('-':xs) = (Minus:) <$> lexString xs
lexString (';':xs) = (Semicolon:) <$> lexString xs
lexString (x:xs)
    | isIdent x = readNext readIdent (x:xs) lookupIdent
    | isDigit x = readNext readIntLit (x:xs) IntLit
    | otherwise = Left $ LexError ("Invalid Token: " ++ show x)

readNext :: (p -> (t, String)) -> p -> (t -> Token) -> Result [Token]
readNext readFn input tokFn =
    let (val, rest) = readFn input
    in ((tokFn val):) <$> lexString rest

readIdent :: String -> (String, String)
readIdent xs = span isIdent xs

readIntLit :: String -> (Integer, String)
readIntLit xs = let (val, rest) = span isDigit xs in (read val, rest)

isIdent :: Char -> Bool
isIdent c = isLetter c || c == '_'
