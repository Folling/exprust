## Introduction

Exprust is an easy to use *expression parser* with some light [customisation options](#customisation).

## Capabilities

| Operation       		 | Example 		    | Result (as String) |
|------------------------|:----------------:|-------------------:|
| Addition        		 | 4 + 4   		    | 8 				 |
| Subtraction     		 | 4 - 4   		    | 0                  |
| Multiplication  		 | 4 * 4   		    | 16                 |
| Power					 | 4 ^ 4   		    | 256				 |
| Parenthesis			 | 4*(4+4) 		    | 32				 |
| Absolute Values 		 | \|-4*4\|		    | 16				 |
| [Functions](#functions)| sin(3.14159265/2)| 1				 	 |
| Hexadecimals			 | 0xA15+4 		    | 2585 			     |
| Binary 				 | 0b11011 * 0xa4   | 4428 			     |
| Octals				 | (0o17 * 4)^0b110 | 46656000000        |
| [Constants](#constants) | pi 			    | 3.1415926...       |
| Degrees				 | sin(30°) 		| 0.5 				 |

**Note that any number here can also be a float and any operation can be paired with any other, PEMDAS is fully implemented**

## Functions

* floor
* ceil
* round
* trunc
* fract
* abs
* log2
* log10
* ln
* sig
* sqrt/root
* cbrt
* sin
* cos
* tan
* asin
* acos
* atan
* sinh
* cosh
* tanh
* asinh
* acosh
* atanh

## Constants

The following constants are supported:

* pi
* e
* phi

**Note that for the moment the actual symbols for pi and phi (π, ϕ) are not supported**


## Customisation

You can use a few flags to alter the output:

| Flag       | Effect 			       |
|:----------:|------------------------|
| NONE       | Returns normal output   |
| AS_DEGREES | Arc functions usually return the result in radians,<br>this switches it up to degrees |
| BIN 		 | Wraps result in a binary number |
| OCT 		 | Wraps result in an octal number |
| HEX        | Wraps result in a hexadecimal number |

## Requests

Extending this is fairly simple. There are a few things 
such as convenience options (e.g. allowing 4(4+4) instead of 4*(4+4))
which I could add, but frankly they're not a major concern for me at the moment.<br>
Should you have any requests though, feel free to open a new issue and I'll look into it.
