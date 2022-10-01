# Maze_Game_Rust
# Overview


For learning Rust, I decided to recreate a maze game that can be played in the console. It is a simple framework of a game that can allow the creation of new maps with relative ease. The board is stored as a vector of strings. The player moves around through these different strings in order to reach the end. The program works by first having the user input a direction on the keyboard. It then checks if that new location has an empty character there. If so, the game will move the player over and erases the player’s old position from the board. If there is any other character in the way, the player is not allowed to move there. This allows the programmer to customize how they want the map to look like and what characters to use to represent walls. 

[Software Demo Video]([http://youtube.link.goes.here](https://youtu.be/MeOciHmQQAw))

# Development Environment

I used Visual Studio Code to write this program.

I used only the basic libraries that come with Rust. No extra libraries were required for this program. 

# Useful Websites

{Make a list of websites that you found helpful in this project}
* [The Rust Standard Library](https://doc.rust-lang.org/std/index.html)
* [Tutorial Point](https://www.tutorialspoint.com/rust/rust_variables.htm)

# Future Work

{Make a list of things that you need to fix, improve, and add in the future.}
* Add new maps
* Remove the need to press enter after each input
* Had keys and locked doors.
