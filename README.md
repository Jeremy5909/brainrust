# File structure

## lib.rs

Defines all the public methods used for the program

## instructions.rs

Highest level - handels instructions

## actions.rs

Lowest level - writes the actual brainfuck

## utils.rs

Medium level - methods for memory management

# Available instructions (right now)

## SetVar

Create or change the value of a variable

## UnsetVar

Set the value of a variable to 0 and remove it from the list of variables and allocated indexes

## Sum

Set the value of the first variable to the sum of the two

## AddString

Create a variable that points to the beginning of a string (like c)
