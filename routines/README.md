Routines are dynamic libraries that (may) perform a calculation each engine cycle. 
It is ideal to design each routine so it modifies only one set (array) of data, so that it may be kept simple and composed with other routines without issue.

This directory also stores the melody, which determine the order and context in which routines are called each update cycle.
