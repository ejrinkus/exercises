# Advent of Code 2021 - Rockstar edition

An implementation of AoC 2021 solutions in [Rockstar](https://codewithrockstar.com/)
This project uses the [Rocky](https://github.com/gaborsch/rocky) interpreter
built by [gaborsch](https://github.com/gaborsch).  This interpreter is built in
Java, and thus requires Java to be installed to run.

# Instructions

Make sure you have Java installed.  Then, simply clone this repo and run the
`play` script.  You can either provide it with three arguments (day, part, test),
or no arguments and the script will prompt you for the information.  Make sure
you are navigated to the directory where the `play` script is sitting.

Examples:
```
# Run day 1, part 1, using test input 1.
> ./play 1 a 1

# Run day 1, part 2, using test input 1.
> ./play 1 b 1

# Run day 2, part 1, using the real input for the problem.
> ./play 2 a ""

# Same as the previous example, but by using the prompts:
> ./play
Which song (1-25)? 
> 2
A-Side or B-side (A/b)? 
> <press Enter, inputs 'a' by default>
Soundtest (leave empty to do it live)? 
> <press Enter, inputs the empty string by default>
```

# Structure

The solution for each part of each day is a separate code file.  The path to
each file is `songs/day<day number>/day<day number>.rock` for part 1 solutions,
and `songs/day<day number>/day<day number>_b_side.rock` for part 2 solutions.

Input files follow a similar structure, and live under the `lyrics` folder.  For
test inputs, append `_test<test number>` to the file names.  The extension for
input files is `.txt`.