1#(Create And Use PDA using Counter) This contract allows users to create their own counter stored in a PDA and also then increment their counter.

-> How does this work on-chain?
a) User calls create_counter
    - A new PDA is created using the "counter" seed and the user's wallet.
    - The counter is initialized to 0.

b) User calls increment_counter
    - The program retrieves the PDA (based on the user's wallet).
    - It increments the counter by 1.
-----------------------------------------------------------------------------------------------------------------------------------------------------
2#(Counter Program) It allows a user to create a counter account and then user can increment the counter.

-> How does this work on-chain?
a) User calls create
    - A program creates a counter (regular user-owned account).
    - The count is initialized to 0.

b) User calls increment
    - A program retrieves the User account.
    - User can increment the counter by 1.
------------------------------------------------------------------------------------------------------------------------------------------------------
3#(Modify Counter Program using some arithmetic operations) This can perform basic arithmetic operations (add, subtract, muliply and divide) on a stored counter value

-> How does this work on-chain? 
a) User calls create
    - The program creates a new on-chain account (BaseAccount) for the user.
    - It sets counter = 0 in the new account.
    - The creator’s public key is saved to identify who owns the counter.

b) User calls add (Increment)
    - The program retrieves the user’s account (BaseAccount).
    - It checks if the user is the creator (to ensure only the creator can modify).
    - The user increments the counter by a specified value (num).

c) User calls subtract (Decrement)
    - The program retrieves the user’s account (BaseAccount).
    - It checks if the user is the creator (only the creator can modify).
    - The program decreases the counter by a specified value (num).
    - If counter is less than num, the program sets counter = 0 to avoid negative values.

d) User calls multiply (Multiple)
    - The program retrieves the user’s account (BaseAccount).
    - It checks if the user is the creator (only the creator can modify).
    - The program multiplies the counter by the specified value (num).

e) User calls divide (Divide)
    - The program retrieves the user’s account (BaseAccount).
    - It checks if the user is the creator (only the creator can modify).
    - The program divides the counter by the specified value (num).
    - If num == 0, the program sets counter = 0 to prevent division by zero.
-------------------------------------------------------------------------------------------------------------------------------------------------
4#(Data List) This can store the string in the data and data_list field and also update it. 

-> How does this work on-chain?
a) Initialization:
    - User calls initialize with the string "Hello".
    - A new account is created, storing:
        data = "Hello"
        data_list = ["Hello"]

b) Update:
    - User calls update with the string "World".
    - The account is updated, storing:
        data = "World" (overwrites the previous value)
        data_list = ["Hello", "World"] (adds the new string to the list)
-------------------------------------------------------------------------------------------------------------------------------------------------
5#(Store and Retireve Data) This allows storing and retrieving a user's data (name and age) on the blockchain.

-> How does this work on-chain?
a) A user calls store_data:
    - A new on-chain account is created.
    - The user's name and age are saved in that account.
    - A log message confirms the data is stored.

b) A user calls retrieve_data:
    - The program reads the stored data from the blockchain.
    - It prints the retrieved values.
-------------------------------------------------------------------------------------------------------------------------------------------------
6#(Store Salary) This manages salary submissions and updates on the blockchain. It ensures that only the owner of a salary record can update their salary.

-> How does this work on-chain?
a) A user calls initialize with a salary amount:
    - A new salary account is created.
    - The salary is stored, and the user is set as the owner.

b) The user wants to update their salary:
    - They call update_salary with a new amount.
    - If the caller is the owner, the salary updates.
    - If someone else tries, an error is thrown (Unauthorized).
-------------------------------------------------------------------------------------------------------------------------------------------------
7#(Store Favorite Items) This smart contract stores and retrieves a user's favorite number, color, and hobbies on the blockchain. It ensures that each user has a unique on-chain account tied to their public key.

-> How does this work on-chain?
a) Example: Alice wants to store her favorite details:
    Favorite number: 7
    Favorite color: "Blue"
    Hobbies: ["Reading", "Coding", "Hiking"]

Step-by-Step Process

i) Alice calls set_favorites with her data.
ii) The contract checks if Alice’s "favorites" account exists.
iii) If the account doesn't exist, it creates a new one.
iv) The contract stores her data on-chain.
v) Next time, Alice can update her favorites without creating a new account.
    
