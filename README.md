# testp
Given an implementation file name as standard input, outputs the corresponding test file name (or vice versa) to standard output.
Later, this will be called from Neovim/Lua to open the corresponding test file next to the buffer [currently](currently.md) opened with the implementation (or test) file.

## Requirements
- Must be able to receive standard input.
- Must output the corresponding file name to standard output.
- Should allow an option to specify the project root to search as a regular expression.
- If the corresponding file does not exist, nothing should be output.
  - However, an option should allow creating the file if it does not exist.
