## O'Caml

### Function Associativity

Every Ocaml function takes exactly one argument.

e.g.
`let f x1 x2 ... xn = e`

is equivalent to:

```
let f = 
    fun x 1 ->
        (fun x2 ->
            (...)
        )
```

The type of the function would be:

`t1 -> t2 -> t3...`

OCaml function types are right associative. There are implicit parenthesis from right to left.

```
e1 e2 e3 e -> ((e1 e2) e3) e4)
```

### Tail Recursion

In O'Caml we can help the compiler solve problems without getting into a stack overflow by using tail recursion.

```
(** [count n] is [n], computed by adding 1 to itself [n] times.  That is,
    this function counts up from 1 to [n]. *)
let rec count n =
    if n = 0 then 0 else 1 + count (n - 1)
```

To prevent a stack overflow we use an accumulator (acc)

```
let rec count_aux n acc =
    if n = 0 then acc else count_aux (n - 1) (acc + 1)

let count_tr n = count_aux n 0
```

This function adds an extra parameter. The addition of 1 happens before the recursive call not after.

A recursive call in tail position does not need a new stack frame. It resuses its existin one.

Note: Don't fixate too much on this at the beginning. 

Formula for tail recursion:
1. Change the function into a helper function. Add an extra argument: the accumulator, often named acc.
2. Write a new “main” version of the function that calls the helper. It passes the original base case’s return value as the initial value of the accumulator.
3. Change the helper function to return the accumulator in the base case.
4. Change the helper function’s recursive case. It now needs to do the extra work on the accumulator argument, before the recursive call. This is the only step that requires much ingenuity.

### Printing

```ocaml
let print_stat name num =
    print_string name;
    print_string ": ";
    print_float num;
    print_newline ()

print_stat "mean" 84.39

out: mean: 84.39

let print_stat name num =
    Printf.printf "%s: %F\n%!" name num
```

### Debugging in OCaml

*Using Print Statements*

```
let inc x =
    let () = print_int x in x+1
```

*Using Function Traces*

```
# let rec fib x = if x <= 1 then 1 else fib (x - 1) + fib (x - 2);; 
# #trace fib;;
```

*Defensive Programming*

```ocaml
(* possibility 1 *)
let random_int bound =
    assert (bound > 0 && bound < 1 lsl 30);
(* proceed with the implementation of the function *)

(* possibility 2 *)
let random_int bound =
    if not (bound > 0 && bound < 1 lsl 30)
    then invalid_arg "bound";
(* proceed with the implementation of the function *)

(* possibility 3 *)
let random_int bound =
    if not (bound > 0 && bound < 1 lsl 30)
    then failwith "bound";
(* proceed with the implementation of the function *)

```

## Data Types

Forms for lists:

```ocaml
[]
e1 :: e2
[e1; e2; ...; en]
```

We can operate on lists with pattern matching:

```
let rec length lst =
    match lst with
        | [] -> 0
        | h :: t -> 1 + length t
```

If we don't need some of the values we can keep them as "_"

```
let rec length lst =
    match lst with
        | [] -> 0
        | _ :: t -> 1 + length t
```

*Pattern matching with lists*

- The pattern x matches any value v
- The pattern _ matches any value and produces no bindings
- The pattern [] matches the value []

The compiler will check for unused branches and return an error if the pattern is not exhaustive.

```ocaml
let rec sum lst =
    match lst with
        | h :: t -> h + sum t 
        | [ h ] -> h
        | [] -> 0
```

The case [ h ] is unused. The first branch will match anything the second one matches.

### Variants

A variant can represent a value with one or more possibilities.

```ocaml
type day = Sun | Mon | Tue
let d = Tue
```

```ocaml
let int_of_day day =
    match day with 
        | Sun -> 1
        | Mon -> 1
```

The type defined the latest will prevail in case there are overlaps.

### Records and Variants

We can create new data types:

```ocaml
type point2d = {x: float; y:float}

let p = {x=3. ; y=-4.}

let magnitude { x = x_pos; y = y_pos} = 
    sqrt (x_pos ** 2. +. y_pos **2.)

let magnitude_terse {x;y} = sqrt(x**2. +. y**2.)

(* Using dot notation to access fields *)

# let distance v1 v2 =
magnitude { x = v1.x -. v2.x; y = v1.y -. v2.y };;
    val distance : point2d -> point2d -> float = <fun>
```

We can declare some of the fields to be mutable:

```
type running_sum =
{
    mutable sum: float;
    mutable sum_sq: float; (* sum of squares *)
    mutable samples: int;
}
;;
```

### Testing 

```
open OUnit2
open Sum

let tests = "test suite for sum" >::: [
    "empty" >:: (fun _ -> assert_equal 0 (sum [])); 
    "singleton" >:: (fun _ -> assert_equal 1 (sum [1])); 
    "two_elements" >:: (fun _ -> assert_equal 3 (sum [1; 2]));
]
let _ = run_test_tt_main testees
```

We can also consolidate the tests:

```
let make_sum_test name expected_output input =
    name >:: (fun _ -> assert_equal expected_output (sum input) ~printer:string_of_int)

let tests = "test suite for sum" >::: [
    make_sum_test "empty" 0 []; 
    make_sum_test "singleton" 1 [1]; 
    make_sum_test "two_elements" 3 [1; 2];
]
```

The ideal process when building tests is:
 - **Write a failing unit test case**. Run the test suite to prove that the test case fails.
 - Implement **just enough functionality** to make the test case pass. Run the test suite to prove that the test case passes.
 - **Improve code as needed**. In the example above we refactored the test suite, but often we’ll need to refactor the functionality being implemented.
 - **Repeat** until you are satisfied that the test suite provides evidence that your implementation is correct.











 
