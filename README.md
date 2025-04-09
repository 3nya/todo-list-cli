# to do list cli

Enya Chen (enya2), Maya Swaminathan (mayas9) , Adrian Vargas (avarga67)

### Project introduction
A simple to-do list in the form of a command line interface, built with Rust and SQL. CLIs are cool and we thought a to-do list would be a fun idea.  

### Technical overview
Should be do tasks such as:
- Create multiple to-do lists
- Add/remove tasks within a to-do list
- Check/uncheck tasks within a to-do list
- Display to-do lists
- Display all tasks within a list
- Store and update all data efficently in a SQL database  

Ideally work on the Rust aspect of the project and get the above tasks completed for checkpoint 1. Learn SQL and incorporate it within the Rust project for checkpoint 2. 

### Challenges
- We don't know SQL! So that part might be a little difficult. 


additional notes:
unique id for each todolist/item?
Item class -> string name of item, boolean checked/unchecked
Todo List Class -> vector of Items, string name of the list, vector of labels (optional)
multiple lists -> vector of Todo Lists
sort to-do list by deadline, labels (alphabetically)
command add label -> can put as many or as little tags as they want
check/uncheck add/remove, number the list and have the user type in the number
check/uncheck same command, toggles