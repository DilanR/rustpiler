# EBNF Spec

```ebnf
Prog        = FnDeclaration , { FnDeclaration } ;

FnDeclaration
            = "fn" , Ident , Parameters , [ "->" , Type ] , Block ;

Parameters  = "(" , [ Parameter , { "," , Parameter } , [ "," ] ] , ")" ;
Parameter   = [ "mut" ] , Ident , ":" , Type ;

Type        = "i32" | "bool" | "String" | "()" ;

Block       = "{" ,
                 [ Statement , { ";" , Statement } , [ ";" ] ] ,
              "}" ;

Statement   = Let
            | Assign
            | While
            | FnDeclaration        (* allowed inside blocks *)
            | Expr ;

Let         = "let" , [ "mut" ] , Ident ,
              [ ":" , Type ] ,
              [ "=" , Expr ] ;

Assign      = Expr , "=" , Expr ;

While       = "while" , Expr , Block ;

Expr        = IfThenElse
            | Block
            | BinaryExpr ;

IfThenElse  = "if" , Expr , Block ,
              [ "else" , ( Block | IfThenElse ) ] ;

BinaryExpr  = UnaryExpr ,
              { BinOp , UnaryExpr } ;

UnaryExpr   = [ UnOp ] , Primary ;

Primary     = Literal
            | Ident
            | Call
            | "(" , Expr , ")" ;

Call        = Ident , "!" , Arguments     (* macro call *)
            | Ident , Arguments           (* normal call *) ;

Arguments   = "(" , [ Expr , { "," , Expr } ] , ")" ;

Literal     = Integer | Bool | String | Unit ;
Integer     = Digit , { Digit } ;
Bool        = "true" | "false" ;
String      = "\"" , { any non-quote char } , "\"" ;
Unit        = "(" , ")" ;

BinOp       = "&&" | "||" | "==" | "<" | ">" | "+" | "-" | "*" | "/" ;
UnOp        = "!" | "-" ;

Ident       = letter , { letter | digit | "_" } ;
Digit       = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
```
