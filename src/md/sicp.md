
## Structure and Interpretation of Computer Programs

Every language has:
- Primitive expressions
- Means of combination
- Means of abstraction

E.g.,
```lisp
(- 136 214)

(define size 2)
(define radius 10)
```

To evaluate combinations we:
1. Evaluate the sub-expressions
2. Apply the procedure that is the value of the leftmost subexpression to the other arguments

This is recursive in nature.

```lisp

(* (+ 2 (* 4 6))
   (+ 3 5 7))  

```

Compound procedures:
- Numbers and arithmetic are primitive data and procedures
- Nesting combinations provides a means of combining operations
- Definitions that associate names with values provide a limited means of abstraction

```lisp
(define (square x)(* x x))

(square 21)
441
```

There are two ways of evaluating:
- Applicative order: evaluate the arguments and then apply
- Normal order: fully expand and then reduce

Lisp uses applicative-order evaluation.

Conditionals

```lisp
(define (abs x)
  (cond ((> x 0) x)
        ((= x 0) 0)
        ((< x 0) (- x))))

(define (abs x)
  (cond ((< x 0) (- x))
        (else x)))
```


