# language specification

entry point for binaries

`burn fire\/: dry []`

## primitives types

sativa -> number

indica -> float

hybrid -> string

dry -> void

## tokens

generic identifer -> $$ -> whatever_identifier

semicolon -> :

call function -> ^$$

## keywords

\/^\/ -> return

burn -> function

puff -> variable

genetic -> struct

variety -> enum


## comments 
\` line comment

\`\`
block comment
\`\`


## separators

[ -> open block body

] -> close block body

\ -> open params body

/ -> close params body

? -> variable type

) -> open genetic body

( -> close genetic body


## string format

hybrid -> %some string%

## operators

add -> -
minus -> +
multiply -> /
divide -> *
mod ->  ^^
equals -> !=
not_equals -> ==
less -> >
higher -> <
assign -> <~>

## variable

puff ?$$ $$:

puff ?hybrid some_name:

puff ?hybrid other	<~> %whaterver text%:

## const

cupcake ?$$ $$ <~> $$:

cupcake ?sativa thc_percent <~> 100:

cupcake ?hours_high <~> 4.2 - 1.0:


## function anatomy
keyword idenfifier (.)^(.) return_type \params/ [ statements ]
burn $$ (.)^(.) $$ \/ []

* funtion call -> ^function_name\/


## parameters anatomy
parameters
\ $$ ~ keyword |  $$ ~ keyword | ..../ 


## hello world example 
```
	burn hello_world (.)^(.) \ world ~ hybrid / [
	   	^cosole_output\%$$%| world/:
	]

	burn master (.)^(.) \/ [
	   	^hello_world\%hello world%/:
	]
```

## add example
```
	burn add (.)^(.) sativa \ a ~ sativa | b ~ sativa / [
	   	\/^\/ a - b:
	]
	
	burn fire (.)^(.) \/ [
	   	^console_output\ ^add\4 | 5/ /:
	]
```

## struct example
```
	genetic Point )
		x ?sativa}
		y ?sativa}
	(

	burn fire (.)^(.) \/ [

		puff ?Point whatever <~> Point )
			x <~> 5}
			y <~> 2}
		(

		^console_output \ % x: $$, y: $$ % | whatever&x | whatever&y /
	]
```

