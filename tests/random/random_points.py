import sys
from random import random

n = int(sys.argv[1])

for _ in range(n):
    (x,y) = (None,None)
    while True:
        (x,y) = (random()*2-1,random()*2-1)
        if x**2+y**2<=1:
            break
    print((x,y))
    
