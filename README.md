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


*Checklist*

Write the concatenate_strings function signature.
Implement the concatenate_strings function.
Initialize two String variables in the main function.
Call the concatenate_strings function with string slices of the variables.
Print the result to the console.
Compile and run the program to test its functionality.

# 2: Calc (Calculator)

simple calculator using enums and pattern matching

**Steps**

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


# 3: Bank

A simple basic banking system using Traits in Rust. The program will allow users to create accounts, deposit and withdraw money, and view their account balance.

**Steps**

Create a Trait called Account that defines the methods deposit, withdraw, and balance. These methods should take a mutable reference to self as an argument.
Implement the Account Trait for a struct called BankAccount. The BankAccount struct should have the fields account_number, holder_name, and balance.
In the implementation of the deposit method for BankAccount, add the deposit amount to the balance.
In the implementation of the withdraw method for BankAccount, subtract the withdraw amount from the balance.
In the implementation of the balance method for BankAccount, return the current balance.
In the main function, create two BankAccount instances with different account numbers and holder names.
Call the deposit method on one of the accounts, passing in a deposit amount.
Call the withdraw method on the other account, passing in a withdraw amount.
Call the balance method on both accounts and print the result to the console.
Compile and run the program to ensure it works as expected.

# 4: Filter

This Rust project implements a custom filtering function using a struct-based approach instead of closures. The program allows filtering elements from a collection (vector) based on a specified condition.

**🔹 Features**
Uses a struct (FilterCondition) to define the filter criteria.
Implements a method (is_match) to check if an item meets the condition.
Provides a function (custom_filter) that iterates over a collection and filters matching elements.
Beginner-friendly implementation with clear, structured Rust code.
**🔧 Usage**
Modify the FilterCondition struct to filter integers, strings, or other types.
Run the program to filter elements from a vector based on a chosen condition.
**📌 Example Output:**
Filtered numbers: [20, 20]
This means the function successfully filtered all occurrences of 20 from the input vector.

✅ Perfect for learning Rust structs, methods, and iteration! 🚀


# 5: Banking System with Error Handling (Bank_err) 🚀
This Rust project is a simple banking system that allows users to deposit, withdraw, and check their balance while handling errors gracefully. It demonstrates Rust Traits, Structs, and the Result type for error handling.

**🔹 Features**
✅ Create Bank Accounts with unique account numbers and holder names. \n
✅ Deposit Money with validation (no negative deposits). \n
✅ Withdraw Money with validation (no overdrafts or negative withdrawals). \n
✅ Error Handling using Result<(), String> for safe transactions. \n
✅ Display Account Balance after transactions. \n

**📌 How It Works**
1️⃣ Create an account with an initial balance. \n
2️⃣ Attempt deposits and withdrawals. \n
3️⃣ Handle errors using match. \n
4️⃣ Print the final balance after transactions. \n

**💡 Tech Used**
🦀 Rust (Traits, Structs, Pattern Matching, and Error Handling). \n

🚀 Run the Project
'''cargo run''' \n

Enter deposit/withdraw amounts and see how the system manages transactions securely

