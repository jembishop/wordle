# Wordle Solver 

A wordle solver written in Rust, designed to be
high performance. It uses the official wordle list, 
but I deliberately mixed together the allowed words
and the possible words together as this is more in 
the spirit of the game. 

It considers the information gain from each guess, 
but does not weight according to any likelyhood.

As it uses information gain, when there are only 
a few possibilities you should just guess those
rather than following the suggested word, its normally
pretty obvious what it is.

It doesn't handle hard mode.

![](https://github.com/jembishop/wordle/blob/main/screenshot.png)
