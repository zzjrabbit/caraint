import sys

sys.set_int_max_str_digits(0)

fib_numbers = []

def fib(n):
    fib_numbers.append(1)
    fib_numbers.append(1)
    for i in range(2,n):
        fib_numbers.append(fib_numbers[i-1]+fib_numbers[i-2])

fib(10000)

# print(fib_numbers)