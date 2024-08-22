# function-finder! 🔎🔎🔎

This repo is my first attempt at a rust project. 🦀

In my job over the summer, I found it hard to keep track of where all my functions are called, so I'm taking it upon myself to create some sort of executeable / extension that will find all times a function is called, and display information about them. Not entirely sure yet how this will be executed or how far I will go with it, but the project is still in its infancy.

TODO:
```
New Parsers:
    -Parse params, take into account multiple parenthesis? (use a stack to figure out when the params end)
Currently working:
    -Keep all lines in a vector to easily display context around a function?
    -add displayers module to display information about results?
        -already finished display_hash for Definition hash tables
            -write display_hash for Calls
            -write display for call and def arrays
    -do we want to add this information to a file?
        -results.txt
    -do we want support for the display flag in the CLIParser?
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
    -Parse files until we find 'function ____()'
        -Start with functions defined by 'function'
        -See later about:
            -Functions contained inside other functions
                -Hold all lines of a file in some vector
                    -This would be better if we are going back in lines
                -Parse lines of the vector to reduce fileIO?
                    -This might be a little slow / memory intense depending on size of files
            -Arrow functions assigned to values
```

