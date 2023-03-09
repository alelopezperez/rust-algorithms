# 217. Contains Duplicate

## Dificulty

##### Easy

## Computational Complexity

<ul>

### Time Complexity O(N)

### Space Complexity O(N)

</ul>

## Algorithm Explanation

The `fn` signatures recieves a vector of numbers so we create a `HashSet` of `i32` and iterate through the `vec` with as a (ref so to not lose ownership), and insert it to the `HashSet`

If at any point we see that we already added the value to the `HashSet`, we stop the loop and `return true` else if we finish the for loop `return false`
