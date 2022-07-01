from functools import reduce
from sys import argv

def is_primo(x):
    if x <=1:
        return False
    for i in range(2,x-1):
        if x % i == 0:
            return False
    return True


fibonacci = lambda n: reduce(lambda f, _: f + [f[-1]+f[-2]], range (n-2), [0,1])

primos_fibs = list(filter(is_primo, fibonacci(int(argv[1]))))

print(f"\t Σ {list(primos_fibs)} = {sum(primos_fibs)}\n\t\b\b{'-'*6*len(primos_fibs)}")