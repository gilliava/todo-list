[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/RQfdh2iK)
# Todo List
Todo list
## Description

Use the console chrono and tui cargo bins to create a todo list in the terminal. If that doesn't work, I will use clap instead. Users with be able to add, edit, and delete elements on the todo list. They will also be able to print out the todo list. A stretch goal would be adding due dates for the tasks. 
## Installation
No special requirements, just follow the standard cargo build process. Eg cargo build
## How to use

You can run the crate with cargo run -- [command].
<br>Other examples are: cargo run --
        <br>add <task-name> <priority>      Add a task to the list, include priority of task (1-5) inclusive
        <br>remove <task-id>                Remove a task at the given index
        <br>list                            List the todos
        <br>clear                           Clear all the todos   
        <br>prioritize                      List the todos in order of priority (highest to lowest)
        <br>help                            Print help information
        <br>schedule                        List the todos by the date they were created (in Utc)
        <br>edit <id>                       Change the name of a task given id
