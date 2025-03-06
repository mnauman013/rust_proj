# 1: Proj
A simple Rust program that demonstrates the concepts of ownership, borrowing, and references. The program will take two strings as input, concatenate them, and then print the result without violating any ownership rules.

**Steps**

Create a function called concatenate_strings that takes two string slices as arguments and returns a new String as the result of concatenating the two input strings.
Inside the concatenate_strings function, create a new String called result. Use the push_str() method to append the contents of the first input string slice, followed by the second input string slice.
Return the result string from the function.
In the main function, create two String variables, string1 and string2, and initialize them with appropriate values.
Call the concatenate_strings function with references to string1 and string2 as arguments (using string slices). Store the result in a new variable called concatenated_string.
Print the concatenated_string variable to the console.
Compile and run the program to ensure it works as expected.


# Checklist

Write the concatenate_strings function signature.
Implement the concatenate_strings function.
Initialize two String variables in the main function.
Call the concatenate_strings function with string slices of the variables.
Print the result to the console.
Compile and run the program to test its functionality.

# 2: Calc (Calculator)

simple calculator using enums and pattern matching

*Steps*

Create an enum called Operation with variants Add, Subtract, Multiply, and Divide. Each variant should hold two f64 values.
Create a function called calculate that takes an Operation enum as an argument and returns an f64 result.
Implement the calculate function using pattern matching to perform the appropriate arithmetic operation based on the variant of the Operation enum.
In the main function, prompt the user to input the first number, the operation to be performed, and the second number.
Parse the user input into appropriate variables.
Create an Operation enum instance with the parsed input values.
Call the calculate function with the created Operation enum instance.
Print the result to the console.
Compile and run the program to ensure it works as expected.
*Checklist*

Define the Operation enum with the appropriate variants and values.
Write the calculate function signature.
Implement the calculate function using pattern matching to perform the appropriate arithmetic operation.
Prompt the user to input the first number, operation, and second number.
Parse the user input into appropriate variables.
Create an Operation enum instance with the parsed input values.
Call the calculate function with the created Operation enum instance.
Print the result to the console.
Compile and run the program to test its functionality.
