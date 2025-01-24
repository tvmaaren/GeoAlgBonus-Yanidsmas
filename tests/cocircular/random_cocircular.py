from math import sin,cos,pi
import sys
from random import random

n = int(sys.argv[1])

for _ in range(n):
    theta = random()*2*pi
    print((cos(theta),sin(theta)))
