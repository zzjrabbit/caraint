# The Cara Programming Language

This is a simple and stupid programming language, which is written with pure rust. This is one of its versions.

## About

This interpreter can run on any platform which supports filesystem operations and memory allocation and deallocation.

## Example

```cara
var test_array = [0;5];

for i in (0,5) {
    print(test_array[i]);
    test_array[i] = i+1;
}

insert(test_array, 0, 1);
remove(test_array, 0);

var length = len(test_array);

print(test_array);
print(length);

```