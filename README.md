A very simple arduino project that creates a serial interface for Morse paddles.
The firmware detects when any paddle is pressed, and writes a status update to the serial port.
The client reads the status from the serial port and then acts accordingly (at the moment, only printing to stdout).
Serial port status is handled, so disconecting the board will correctly close the connection (and currently exit the program).

TODO:
* Hardware schematic
* TUI for client
* Actual actions for the client, i.e. run a script on left or right paddle press
