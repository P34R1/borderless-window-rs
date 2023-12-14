# Borderless-Window-RS

This program will fullscreen any application borderlessly.
The compiled binary when ran will

- Find all the visible windows
- Check if you gave the program as a parameter
- Otherwise List them out for you to select
- Fullscreen the program

## THIS PROBABLY ONLY WORKS ON WINDOWS

it wasn't tested on any other OS then windows 10, but uses winapi so should work on win11 too.

## Usage

```sh
./borderless-window-rs ProgramNameCaseSensitive
```

```text
./borderless-window-rs
? Which program would you like to borderlessly fullscreen?  
> Program 1
  Program 2
  Program 3
  Program 4
  Program 5
  Program 6
v Program 7
[↑↓ to move, enter to select, type to filter]
```
