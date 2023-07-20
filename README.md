# booky

## About
booky is a minimalistic TUI tool for managing your growing book collection. It is writtin in Rust and uses [Diesel](https://diesel.rs/) as it's ORM to query an sqlite database. The Tui is created using the awesome Rust crate called [ratatui](https://crates.io/crates/ratatui)

## Motivation
After learning Rust for 10 days I wanted to smart a small project that I could personally use. I live inside my terminal like the cool kids, so I wanted to create a small tool that could make it easy for me to manage my growing book collection.

Reading is fun, but I quickly forget which books I've read in a month/year. Or how many books of an author I have personally read. I want a simple tool that shows me my current list of books and allows me to see statistics like:

"Books read in the month of April"
"Books read in 2023"
"You've read 20 fictional books in 2020", etc.

As of now, booky can only perform CRUD operations and allows for a basic search on book titles. I plan to add more features as can be seen in the TODO section.

This project suffers from noobiness, I'm learning Rust and a bunch of things at the same time. I'm doing a lot of yucky experiments to gain knowledge, feel free to tell me my code sucks and how I can improve it. You can ofcourse also do a pr to add features or write better code.

## Features
- Basic Vim movement: Use `jk` to move and `/` to search.
- Ability to search books
- Sqlite database so you can store more books than you can read (and bring your collection everywhere) ;)
- Works in the terminal, you can now look cool adding books!
  
## Showcase
![image](https://github.com/Ay-can/booky/assets/61593654/98beb723-3757-4bf2-9459-c3ffec139e5b)
![image](https://github.com/Ay-can/booky/assets/61593654/b92287cf-7601-48a6-8da7-2a68dee7904a)
![image](https://github.com/Ay-can/booky/assets/61593654/0147bb38-e1c0-44c3-9dae-026c8b04121e)


## Building and Installing
- Make sure you have Rust installed.
- Clone this repo and cd into it.
- Build the project using `cargo b --release`
- The binary can be found in `target/release/booky'

If all goes well, booky will create a directory in `/documents/booky` containing a empty `books.db`

## Keybindings
- `?` -> To open help menu.
- `i` -> Insert a new book.
- `d` -> Delete current highlighted book (no confirmation).
- `u` -> Update current highlighted book.
- `jk` or `up/down` -> To change selected book.
- `/` -> Search for a book.
- `r` -> Clear search
- `q` -> Quit booky
  
## TODO
- Refactor a bunch of code, most importantly `handler.rs` and `ui.rs`(yucky yucky).
- Make booky render the ui better on smaller terminals, currently booky works best on a full screen terminal.
- Add dates to the books that you've read with chrono, startdate and enddate.
- Create a statistics tab that shows you how many books you've read this month/year, how many per genre and more.
- Add the ability to search/filter on more(author, genre, rating etc).
- Add a logger to booky that shows the user if changes were made to the db like an CRUD operation
- Add ability to export your books in json format, that way you can use them on your website I guess?

## Acknowledgements/Inspiration
I wouldn't be able to create booky without an awesome project called [kanban-tui](https://github.com/JosephFerano/kanban-tui)
I took a lot of inspiration from Joseph Ferano's kanban-tui project, it help me understand how I could create "forms" that I could use for adding/searching in booky. Check his cool project out!

## License
booky is licensed with the MIT license. Check the LICENSE file for more info.



