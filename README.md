# function-finder! 🔎🔎🔎

This repo is my first attempt at rust. 🦀

In my job over the summer, I find it hard to keep track of where all my functions are called, so I'm taking it upon myself to create some sort of executeable / extension that will find all times a function is called, and display information about them. Not entirely sure yet how this will be executed or how far I will go with it, but the project is still in its infancy.

TODO:
```
New Parsers:
    -Parser to find all function calls in a file (regardless of where it was defined)
        -Or do we want it to only find functions defined in the scope of the parser?
    -Parser to find all in a directory / subdirectories
        -How do we want to display it?
Currently working:
    -finish parse_line to remove the need to a bunch of parsers
    -return a Definition
        -Add async / export flags to the definition struct
Finders:
    Make other finders use parse_line
```
Ideas:
```
Interfacing
    -Allow users to pass in a function name and filename through CLI interface
        -Allow users to specify
            -Filename
            -Function name
            -Directory name
            -Display type (what we want to show to the user)
                -Only functions
                -Only calls
        -If no arguments are found, list all functions?
        -If the function name is close to one found, do stuff?
            -This may be harder
Parsing
    -Add support for export and async?
        -Possible new parts of Definition type that specify it is an export or async
    -Use multithreading for faster fileIO
        -Start single threaded, figure out the basics first
    -Parse files until we find 'function ____()'
        -Start with functions defined by 'function'
        -See later about:
            -Functions contained inside other functions
                -Hold all lines of a file in some vector
                -Parse lines of the vector to reduce fileIO?
                    -This might be a little slow / memory intense depending on size of files
            -Arrow functions assigned to values
```

