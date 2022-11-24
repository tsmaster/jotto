# The Jotto Problem

aka finding 5 non-overlapping worldle words

Matt Parker wrote a python script to run in a month to find 5 words
that don't overlap. People subsequently got runtimes down into
milliseconds using Rust, C, C++, and Java. Not really so much Java.

I want to give it a go in Rust.


## Wordle + Moby (all)
running a brute force 5-deep search, it took me 39m 7s to search for
all 5-tuples

## Wordle alone
huh, got no hits

## Wordle + selected Moby lists
5m 44s no hits

## Parker's list
Matt wasn't using the Wordle list, he used:
https://raw.githubusercontent.com/dwyl/english-words/master/words_alpha.txt

which boils down to 5977 words

I'm not in love with Parker's word list, the final solution produced is
GLACK HDQRS JOWPY MUNTZ VIBEX

27m 0.549s

But now I know a vibex is a subcutaneous marking of blood. Like a
bruise, I guess.

## Small Rust Improvements

One of the big improvements that Matt talked about was to change the
representation of a word from a string to a bitfield u32, where each
bit in the u32 represents whether that word had that letter (A = bit
0, B = bit 1, ... Z = bit 25). This makes checking for overlap simple,
just a bitwise AND.

So, I wrote a conversion routine, converting strings to u32s, and
wrote overlap_int(), which just checks to see if the and is non-zero.

Also, I wrote a preprocessing step that checks to see if the word at
index a overlaps with the word at index b, where a < b. I then
maintain a hashtable, indexed on a, of all b values that do not
overlap. This allows me to only consider words that have at least one
previous word that works, and then use the overlap_int() test for the
others.

Time: 1m3s

That's good for one day. I think I'd like to try pulling in an
implementation of Knuth's Algorithm X and see how it performs.

## Algorithm X

A few weeks later, I have implemented Algorithm X (perhaps not the
most performantly), and my implementation takes a total of

Time: 1m4s

which includes building the table. I'm using a vector of ints instead
of Knuth's linked lists. Knuth's implementation is assuredly a little
better.




