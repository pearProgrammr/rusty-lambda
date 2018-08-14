# rusty-lambda

rusty-lambda is an evaluator for a simply-typed lambda calculus
programming language. The types and values supported for this
language are unsigned integers, booleans, and functions (including
closures).


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


## Type Checker

The type checker evaluates all parts of the AST to ensure that
each term is well-typed. Type checking follows a standard format
that can be found in sources such as chapter 6 of Types and
Programming Languages (by Benjamin Pierce).

rusty-lambda uses a recursive type checker that passes a type
environment (where a variable is mapped to a type) and a term.
In some implementations, it is common to return the environment
and a type value as a pair. In this implementation, only the type
value is returned. To handle type checking of assignment, the
right-hand side of the expression is type-checked and a special
assignm TermType is returned. This is caught by the caller of the
type checking and added to the environment to be used for
subsequent type-checking.


## Evaluator

Like the type checker, the expression evaluator is recursive that
passes an environment (where varaibles are bound to values) and
returns a value. Evaluating assignment is identical to the type
checker: the evaluation of assignment results in an assignment
value that is added to the environment by the caller of top-level
caller of eval.
