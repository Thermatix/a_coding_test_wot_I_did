# Mars Rover Coding challenge

## Notes

I hadn't originally intended to do this in rust however I decided to try it as a scoping exercise 
and I realised that I had enough knowledge and ability to do it in rust.
It was an interesting project and as usual for me with rust I learned from what I built and shored up other
knowledge I already had. I admit to finding most coding tests a bit boring but I found working on this quite
interesting, it helps that I like Space and Mars Rover related stuff; I don't even normally do write-ups but
I felt in this case I had to.

The bit I enjoyed working on the most was the `rover_commands` module, whilst it isn't 
building a parsing grammar it felt a bit like it and was just interesting overall to work on.
It was also quite vindicating as it proved my idea worked.

I'm also quite proud with how the execution of the commands works as it ended up with a clean looking chunk
of code in `fn main()`.

## Usage

The executable can be used in two ways:

+ **Load file**: `cargo run rover.commands` - with `rover.commands` being a text file containing the commands to be executed
+ **Manual Mode**: `cargo run` - this allows you to manually enter commands pressing enter between commands, to finish you you have to `CTRL-C` followed by `enter`, this is a bug I'm not sure how to resolve

The first command run regardless of usage method needs to be the grid size command `x y`, replacing `x y`
with the max bounds of the grid.

The provided test case is in the `rover.commands` file.

## Bugs

### No error reporting

Because of some difficulty there is no reporting of any internal errors
The executable will however still panic in certain uncovered issues

### less then zero bounds

Whilst it's not possible to go beyond the max grid size bounds it's possible to go under,
this will however cause an over flow **panic**. 
It is because I couldn't figure out the rust idiomatic way to handle going below zero for
**unsized** value types when adding a negative number.
