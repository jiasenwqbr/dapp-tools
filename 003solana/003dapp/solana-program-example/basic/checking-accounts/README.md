## Checking Accounts
Solana Programs should perform checks on instructions to ensure security and that required invariants are not being violated.

Solana 程序应该对指令执行检查，以确保安全性，并且不违反所需的不变量。

These checks vary and depend on the exact task of the Solana Program.

这些检查各不相同，取决​​于 Solana 程序的具体任务。


In this example we see some of the common checks a Solana Program can perform:

在此示例中，我们看到了 Solana 程序可以执行的一些常见检查：

checking the program ID from the instruction is the program ID of your program

检查指令中的程序 ID 是否是程序的程序 ID

checking that the order and number of accounts are correct

检查帐户的顺序和数量是否正确


checking the initialization state of an account

检查帐户的初始化状态


etc.

等等。
