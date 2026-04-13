module Main (main) where

import Test.HUnit
import Lexer
import Error
import Token
import qualified Ast
import Parser

main :: IO ()
main = do
    _ <- runTestTT tests
    return ()

tests :: Test
tests = TestList [lexTests, parseExprTests]

lexTest :: String -> [Token] -> String -> Test
lexTest desc res input = TestCase $ assertEqual desc (Right res) (lexString input)

lexTests :: Test
lexTests = TestList
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
    ]

exprTest :: String -> String -> Test
exprTest expect input = TestCase $ assertEqual ":)" expect
    (
    case lexString input >>= parseExpr of
        Left e -> "Error: " ++ show e
        Right (expr, _) -> show expr
    )

parseExprTests :: Test
parseExprTests = TestList
    [ exprTest "foo" "foo"
    , exprTest "true" "true"
    , exprTest "4" "5"
    , exprTest "(foo bar)" "foo bar"
    , exprTest "((foo bar) baz)" "foo bar baz"
    , exprTest "(foo (bar baz))" "foo (bar baz)"
    , exprTest "(1 + 2)" "1 + 2"
    , exprTest "((1 + 2) + 3)" "1 + 2 + 3"
    , exprTest "(1 + (2 + 3))" "1 + (2 + 3)"
    , exprTest "((app 1) + 2)" "app 1 + 2"
    -- todo show rest
    ]
