# rust-algorithms

| Problem            | Difficulty | Implmentation                                     | Link                                                                   | problem source |
| ------------------ | ---------- | ------------------------------------------------- | ---------------------------------------------------------------------- | -------------- |
| Contains Duplicate | Easy       | [contains-duplicate](src/bin/contains-duplicate/) | [contains-duplicate](https://leetcode.com/problems/contains-duplicate) | leetcode       |
| Valid Anagram      | Easy       | [is-anagram](src/bin/is-anagram/)                 | [contains-duplicate](https://leetcode.com/problems/valid-anagram)      | leetcode       |

## Iter

- `iter().all()`

  Traverses over all the values and applies a closure that retunr a boolean. At the first false it stops and returns false, if it completes returns true.

- `string.chars().iter()` has no iter().

## HashMap

- **entry**
<ul>

`hash.entry(letter)` wish will check if the entry exists; which takes not a reference but a value.

- **or_insert**

<ul>

If the values does not exist you can append `or_insert(val)` for a default value; If it does exist you will get a `mut &self` of the value.

```
let count = char_count.entry(letter).or_insert(0);
*count += 1;

//or

 *(char_count.entry(letter).or_insert(0))+=1;
```

</ul>

- **and_modify**
<ul>
If the values exists it will modify it via a clousure(lambda), no if not option; it can be later appended to `or_insert`

`char_count.entry(letter).and_modify(|x| *x -= 1).or_insert(0);`

</ul>

</ul>

- **contains_key**
- **with_capacity**  
  It gives the compiler more info of the possible min size of the HashMap or Vector or HashSet; it will still be of lengeth 0 or any previous inserts you made.

  ```
  let mut vec = Vec::with_capacity(10);
  let mut char_count = HashMap::with_capacity(s.len());
  ```

## Vector and Array

- **Filling**

  The proccess of initializing the array, vector, or any other "collections" of stuff, with the same value.

  `let mut char_count = [0; 26];`
  `let mut char_count = vec![0; 26];`
