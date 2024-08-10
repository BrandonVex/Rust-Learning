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
	var nameone string = "Pig" // first variable and type declaration (string)
	var nametwo = "Cow"        // type inference
	var namethree string       //no value assigned but can be assigned later

	// Print the value of the variable
	fmt.Println(nameone, nametwo, namethree)

	nameone = "Dog"   // reassigning the value of the variable
	namethree = "Cat" // assigning the value of the variable after declaration

	// Print the value of the variable
	fmt.Println(nameone, nametwo, namethree)

	// another way to initialize a variable
	namefour := "Elephant" // short variable declaration, only use when initializing a variable
	fmt.Println(namefour)  // := is used to declare and initialize a variable but can only be used inside a function

	// ints
	// all 3 are valid ways to declare an integer variable
	var ageOne int = 20
	var ageTwo = 30
	ageThree := 40

	fmt.Println(ageOne, ageTwo, ageThree)
}

//PrintLn is a function that is used to print the message to the console (print line)

// Go is a statically typed language, which means that the type of a variable is known at compile time.
// Go is a compiled language, which means that the code you write needs to be compiled before it can be run.
