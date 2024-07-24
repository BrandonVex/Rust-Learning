package main // package declaration of files and code

import "fmt" //import the fmt package to use the Println function to print

// main function is the entry point of the program
func main() {
	// Println function is used to print the message to the console
	fmt.Println("Hello, World!")
	fmt.Println("Welcome to Go Programming!")
	fmt.Println("This is my first Go Program!")

	// Declare a variable of type string
	// Assign a value to the variable
	// strings are only double quoted
	var name string = "John Doe"

	// Print the value of the variable
	fmt.Println(name)
}
