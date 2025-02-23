A very simple arduino project that creates a serial interface for Morse paddles.
The firmware detects when any paddle is pressed, and writes a status update to the serial port.
The client reads the status from the serial port and then acts accordingly.
Serial port status is handled, so disconecting the board will correctly close the connection. Reconnecting is not handled, so you should
exit the program and restart it.

The program calls dit.sh and dah.sh respectively (currently, the configuration is hardcoded to L paddle = dit, R paddle = dah)
These scripts should contain infinite loops repeating the action required to interface the software you're using. The program
will kill the script when the paddle is released. This is because this software does not bother with timing, so
it's effectively a software On/Off.

TODO:
* Hardware schematic
