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
    -add displayers module to display information about results?
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

