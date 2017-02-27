# termcalc

This program is like gnome-calculator, but
- running in the terminal
- with variables
- with functions (but without recursion, sorry)
- capable of loading simple scripts

## Install

- clone
- run ```cargo build --release```

## Usage

Calculate a term just by typing it:

```
> 2+2
    $0: 4 [int]
> 4+2*2-3^3
    $1: -19 [int]
> (5+3)*12
    $2: 96 [int]
```

You can also use floats:

```
> 2.5+1.3
    $0: 3.8 [float]
> 2*3e8
    $1: 600000000 [float]
> .5*123
    $2: 61.5 [float]
```

Reference the last calculations:

```
> 2^3
    $0: 8 [int]
> $0/2
    $1: 4 [float]
> $0*$1
    $2: 32 [float]
```

Or reference a variable:

```
> :var X 5^2
> X/2
    $0: 12.5 [float]
```

Use a custom or builtin function:

```
> sin(0.5)
    $0: 0.479425538604203 [float]
> sin! 0.5
    $1: 0.479425538604203 [float]
> :fn test X = X+5
> test 17
    $2: 22 [int]
```

Load scripts:

test.txt:
```
--this is a comment
:var X 5
:fn testfn X = X^3
```

termcalc:
```
> :load test.txt
> X+5
    $0: 10 [int]
> testfn X
    $1: 125 [int]
```

## Some rules:

- __```function! X``` equals ```function(X)```__
- __function names start with a lowercase, and variables with an uppercase character__
- __recursion is not possible in the current state__

```:fn sumall X = if (X = 0, 1, X+sumall(X-1))``` is syntactically possible, but crashes

Calling other functions within a function is perfectly fine, though.

- function parameters before variables:
```
:var X 5
:fn testfn X = X^3 [-> X is referring to the argument, not the var]
```

## List of all arithmetic functions

- ```add (x y)``` alias ```x+y```: add the two values (int + int = int, else float)
- ```sub (x y)``` alias ```x-y```: subtract the two values (int - int = int, else float)
- ```div (x y)``` alias ```x/y```: divide the two values (always float)
- ```mul (x y)``` alias ```x*y```: multiply the two values (int * int = int, felse loat)
- ```pow (x,y)``` alias ```x^y```: x to the power of y (x = int -> int, y = float -> float)
- ```neg (x)``` alias ```-x```: returns negative x, neg (int) = int, neg (float) = float
- ```sqrt (x)```: the square root of x (always float)
- ```sin (x), cos (x), tan (x), sinh (x), cosh (x), tanh (x)``` (always float)
- ```abs (x)```: |x| (x = int -> int, y = float -> float)
- ```floor (x)```: round to the nearest integer < x
- ```ceil (x)```: round to the nearest integer > x
- ```if (a,b,c)```: if a is 0, return c, else return b
- ```eq (x,y)``` alias ```x = y```: return 1 when x = y, else 0 (can only compare int/int or float/float)
- ```lt (x,y)``` alias ```x < y```: return 1 when x < y, else 0
- ```gt (x,y)``` alias ```x > y```: return 1 when x > y, else 0

## List of all commands

- ```:q``` or ```:quit```: exit the program
- ```:vars```: show all variables and their current value
- ```:fns```: show all user functions and their current value
- ```:var [name] [term]```: assign term to name
- ```:fn [name] [arg1] [arg2] [...] = [term]```: create a function
- ```:rvar [name]```: delete variable,
- ```:rfn [name]```: delete function,
- ```:load [path]```: loads a script

### Precedence

In order:
- +, -
- *, /
- ^
- UnarySub (aka. neg, aka. (-x))
- =, <, >
