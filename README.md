# HYDRAlang
### An esoteric programming language based on binary and heads.

HYDRAlang is named after the mythical beast hydra, which when one of its heads gets cut off, two more rise in its place.

HYDRAlang is focused on manipulating a HYDRA, which is a list of bytes represented as binary data called "heads". The HYDRA starts out with a single head, initialized to 0 (00000000 in the actual program). There are a range of commands that can manipulate the heads of the HYDRA.  

`-` inverts the rightmost bit of the current head of the HYDRA; if the bit is equal to 1, it becomes 0, and vice versa.  

`;` moves the rightmost bit of the current head of the HYDRA to the end.  

`%` creates a new head of the HYDRA that's a copy of the current head, similar to a single hydra head becoming two after it is cut.  

`>` makes the next head of the HYDRA the current head. If `>` would move past the last head of the HYDRA, it wraps around to the first head.

 `!` outputs the binary data of the current head of the HYDRA as a character.
 
 `[` and `]` loops through all code between them as long as the first bit of the current head of the HYDRA is 1.
 
 `#` resets the current head of the HYDRA to its default value of 00000000.
