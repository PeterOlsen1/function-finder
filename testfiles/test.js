// Function to print a message
function hello(string) {
    console.log(string);
}

// Function to add two numbers
function add(a, b) {
    return a + b;
}

// Function to multiply two numbers
function multiply(a, b) {
    return a * b;
}

// Function to concatenate two strings
function concatenate(str1, str2) {
    return str1 + str2;
}

// Function with default parameters
function greet(name = "Guest") {
    console.log(`Hello, ${name}!`);
}

// Function to check if a number is even
function isEven(number) {
    return number % 2 === 0;
}

// Function that logs the length of a string
function logLength(string) {
    console.log(`Length of string: ${string.length}`);
}

// Function that returns the maximum of two numbers
function max(a, b) {
    return a > b ? a : b;
}

// Main function to call the above functions
function main() {
    hello("Hello World!");
    
    let sum = add(5, 3);
    console.log(`Sum: ${sum}`);
    
    let product = multiply(4, 7);
    console.log(`Product: ${product}`);
    
    let combined = concatenate("Hello", " World!");
    console.log(`Concatenated: ${combined}`);
    
    greet("Alice");
    greet();
    
    console.log(`Is 10 even? ${isEven(10)}`);
    console.log(`Is 7 even? ${isEven(7)}`);
    
    logLength("Rust");
    
    let maximum = max(15, 22);
    console.log(`Maximum: ${maximum}`);
    
    // Call test function (if exists)
    if (typeof test === "function") {
        test();
    }
}

// Call the main function to execute the program
main();

hello();