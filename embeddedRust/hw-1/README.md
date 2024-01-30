# Assignment 1 - Life
## Submitted by: Shrikrishna Bhat

## Description
The program is a simple implementation of Conway's Game of Life in Rust on the Embedded system that is microbit v2.

## Libraries used
```nanorand``` and other microbit libraries.

## How to run
```cargo add target thumbv7em-none-eabihf``` to add the target to the project.
```cargo build --release``` to build the project.
```cargo run --release``` to run the project.

## Methodology
a) The game of life code is given by the professor hence I have used the same code directly.

b) I used the seed starting at 9 because there is a possibilty of all blank cells at seed 9, 10 and 11 hence the output can be debugged easily.

c) I have created additional functions to handle buttons 'A' and 'B' such that we can easily use them in the code.

d) Initially, I set the timer to 1000ms such that it is clear that to see that display is being changed. If button A is pressed ```handle_button_a``` is called and if button B is pressed ```handle_button_b``` is called. Else we are setting the delay if 100 and life is being played.

e) After the life is done playing, initially we are setting delay to 3s such that we wait for input button and we can get to know whether button is pressed or not, and if it is pressed then the handler is called. Else the board is again started at random.

f) I also created a function to generate random number which is 0 and 1. I also created a complement function, which complements the board.

g) I also added a helper function ```print_board``` which prints the board on the serial monitor.

h) I had hard time in figuring out how to create functions ```handle_button_a``` and ```handle_button_b``` because I had to figure out which parameeters are needed to be passed. And also I had to deference the life board which I found out the hard way.

i) I have added many ```rprintln!()``` statements for easier debugging.
