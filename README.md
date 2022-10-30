# ByteCulator
byteculator overcomes overflow problem, using strings for implementing basic math operations, 
so this way the calculations can even exceed the integer or float number boundaries

# Input note:
terms and operators must all be seperated by spaces suchs: 2 + 5 * 6 - 4

# Features:
    1- Supported Operations: Addition (+), Substraction (-), Multiply (*), Power (^)
    2- Numbers are stored as strings, so the numbers can be large as possible (Maximum number of supported digis equals to maximum supported string length in rust)
    3- Solves and logs the expressions step by step
        step by step log can be disabled by -l param:
        e.g.: 4 * 25 -l + 2 ^ 4
            => just logs 4 * 25 and then show final result
    4- you can also use Full logging to even solve the calculations in more details,
        including digit by digit multiply, or step by step power
        use +f param to enable full log before the terms you want:
        2 ^ 5 +f + 10 ^ 5 -f * 10
        and -f to turn full log off.
    5- -l/+l and -f/+f params can be used in any place, so you can exactly determine
        which part of the expression should be logged or full logged.
    5- For now calculations are limited to integers
    6- Priorities will be applied soon
    7- enter x or empty input to close the app.