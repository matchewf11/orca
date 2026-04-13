module Main (main) where

import Test.HUnit
import Lexer
import Error
import Token
import qualified Ast
import Parser

parseTest :: String -> String -> String -> Test
parseTest desc expect input  =
    TestCase $ assertEqual desc (Right expect) (show <$> (lexString input >>= parseTok))

lexTest :: String -> [Token] -> String -> Test
lexTest desc res input = TestCase $ assertEqual desc (Right res) (lexString input)

tests = TestList
    [ lexTest "empty case" [] ""
    , lexTest "space case" [] "   "
    , lexTest "space case" [Boolean True, Boolean False] " true false "
    , lexTest "symbol case" [Plus, Minus, Assign, Semicolon] "+-=;"
    , lexTest "ident case" [Ident "foo"] " foo "
    , lexTest "int case" [IntLit 40, IntLit 83] " 40 83 "
    , lexTest "add function"
        [ Ident "add"
        , Ident "x"
        , Ident "y"
        , Assign
        , Ident "x"
        , Plus
        , Ident "y"
        , Semicolon
        ] "add x y = x + y;"
    , parseTest "Test Statement Expr" "x;" "x"
    , parseTest "Test Statement Binding" "ah" ""
    , parseTest "Test Expr Infix" "ah" ""
    , parseTest "Test Expr IdentLit" "ah" ""
    , parseTest "Test Expr BoolLit" "ah" ""
    , parseTest "Test Expr IntLit" "ah" ""
    , parseTest "Test Complex" "ah" ""
    , parseTest "Test Precedence a b c == (a b) c, a b + c d = (a b) + (c d)" "ah" ""
    , parseTest "Test Precedence" "ah" ""
    , parseTest "Test Empty" "" ""
    ]

main :: IO ()
main = do
    _ <- runTestTT tests
    return ()
