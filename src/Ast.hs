module Ast where

newtype Program = Program [Statement]
    deriving (Eq)

instance Show Program where
    show (Program exprs) = unlines $ show <$> exprs

data Statement
    = Expr Expression
    | Binding String [String] Expression
    deriving (Eq)

instance Show Statement where
    show (Expr expr) = show expr ++ ";"
    show (Binding name params expr) = unwords (name:params)  ++ " = " ++ show expr

data Expression
    = Infix Expression Operation Expression
    | IdentLit String
    | BoolLit Bool
    | IntLit Integer
    | App Expression Expression
    deriving (Eq)

instance Show Expression where
    show (Infix l o r) =
        "(" ++ show l ++ " " ++ show o ++ " " ++ show r ++ ")"
    show (IdentLit s) = s
    show (BoolLit b) = show b
    show (IntLit i) = show i

data Operation
    = Plus
    | Minus
    deriving (Eq)

instance Show Operation where
    show Plus = "+"
    show Minus = "-"
