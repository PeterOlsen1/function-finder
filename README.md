# function-finder!

This repo is my first attempt at rust.

In my job over the summer, I find it hard to keep track of where all my functions are called, so I'm taking it upon myself to create some sort of executeable / extension that will find all times a function is called, and display information about them.

Ideas:
```
Interfacing
  -Allow users to pass in a function name and filename through CLI interface
    -If no arguments are found, list all functions?
    -If the function name is close to one found,
Parsing
  -Use multithreading for faster fileIO
    -Start single threaded, figure out the basics first
  -Parse files until we find 'function ____()'
    -Start with explicitly defined function since anything more would probably be too much at the moment
```

