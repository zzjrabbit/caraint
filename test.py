import sys

sys.set_int_max_str_digits(0)

def fib(n):
    if n == 1 or n == 2:
        return 1
    a,b = 1,1
    for i in range(3,n+1):
        t = a+b
        a = b
        b = t
    return b

print(fib(1000000))

