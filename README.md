# rusty-lambda

rusty-lambda is an evaluator for a simply-typed lambda calculus
programming language.

## Type Checker

## Evaluator

## Parser

rusty-lambda uses a hand crafted recursive descent parser made
using combinators from the nom crate. The syntax is quite simple,
with the normal rules for operator precedence and featuring
haskell style function application with the "space" operator.

A few noteworthy quirks:

   * conditional statements are of the form:
     ```
     if *cond* then *true-case* else *false-case* endif
     ```
   * lambdas are introduced with a backslash and must be enclosed
     in parenthesis.
     ```
     (\a. (\b. a + b * 3))
     ```
   * At the top level, assignments are also allowed:
     ```
     const3 := (\_. 3)
     ```

For more examples see the .lam files in the tests/ directory.
