# File structure

## lib.rs

Defines all the public methods used for the program

## instructions.rs

Defines how to handle each Instruction in the program when built

## actions.rs

The functions that do the actual brainfuck writing and write to the program output. Like a middle man

## utils.rs

Helper functions used to handle memory, etc

# Available instructions (right now)

## SetVar

Create or change the value of a variable

## UnsetVar

Set the value of a variable to 0 and remove it from the list of variables and allocated indexes

## Sum

Set the value of the first variable to the sum of the two

## AddString

Create a variable that points to the beginning of a string (like c)
