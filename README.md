## Introduction

Exprust is an easy to use *expression parser* with some basic [customization Options](#Customisation).

## Capabilities

| Operation       		 | Example 		    | Result (as String) |
|------------------------|:----------------:|-------------------:|
| Addition        		 | 4 + 4   		    | 8 				 |
| Subtraction     		 | 4 - 4   		    | 0                  |
| Multiplication  		 | 4 * 4   		    | 16                 |
| Power					 | 4 ^ 4   		    | 256				 |
| Parenthesis			 | 4*(4+4) 		    | 32				 |
| Absolute Values 		 | \|-4*4\|		    | 16				 |
| [Functions](#Functions)| sin(3.14159265/2)| 1				 	 |
| Hexadecimals			 | 0xA15+4 		    | 2585 			     |
| Binary 				 | 0b11011 * 0xa4   | 4428 			     |
| Octals				 | (0o17 * 4)^0b110 | 46656000000        |
| Constants 			 | pi 			    | 3.1415926...       |
| Degrees				 | sin(30°) 		| 0.5 				 |

**Note That any number here can also be a float and any operation can be paired with any other, PEMDAS is fully implemented**

## Functions

* floor
* ceil
* round
* trunc
* fract
* abs
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

## Customisation

You can use a few flags to alter the output:

| Flag       | Effect 			       |
|:----------:|------------------------|
| NONE       | returns normal output   |
| AS_DEGREES | arc functions usually return the result in radians,<br>this switches it up to degrees |
| BIN 		 | Wraps result in a binary number |
| OCT 		 | Wraps result in an octal number |
| HEX        | wraps result in a hexadecimal number |
