# ByteCulator
byteculator overcomes overflow problem, using strings for implementing basic math operations, 
so this way the calculations can even exceed the integer or float number boundaries

# Features:
    1- Solves and logs the expressions step by step
    2- Supported Operations: Addition (+), Substraction (-), Multiply (*), Power (^)
    3- For now calculations are limited to integers
    4- Numbers are stored as strings, so the numbers can be large as possible (Maximum number of supported digis equals to maximum supported string length in rust)
    5- you can also use Full logging to even log the calculations more detailed,
        use -fl param to enable full log before the terms you want:
        2 ^ 5 -fl + 10 ^ 5