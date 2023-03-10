242. Valid Anagram

## Dificulty

##### Easy

## Computational Complexity

<ul>

### Time Complexity O(3N)

### Space Complexity O(N)

</ul>

## Algorithm Explanation

The `fn` signature recieves two string, check first for equal length, using an array of number and length of s.len(), the lowercase charecter
will act as a key and the the array elements as value, 'a'=1 'b'= 2..., count +1 for the first string, and -1 for the second string. If the resultin array is all zeroes then is true, if not false.
