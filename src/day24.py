from z3 import *
# this is from Aoc 2023 day 24 part 2 where I used Z3
a = Real('a')
b, c = Reals('b c')
t = Real('t')
v = Real('v')
k = Real('k')
x, y, z = Reals('x y z')

s = Solver()

s.add(t >= 0)
s.add(v >= 0, k >= 0, a + t*x == 156689809620606 + t * (-26), b + t*y == 243565579389165 + t * (48), c + t*z == 455137247320393 + t * (-140), a + v*x == 106355761063908 + v * (73), b + v*y == 459832650718033 + v * (-206), c + v*z == 351953299411025 + v * (-52), a + k*x == 271915251832336 + k * (31), b + k*y == 487490927073225 + k * (-414), c + k*z == 398003502953444 + k * (-304))

s.check()
print(s.model().evaluate(a) + s.model().eval(b) + s.model().eval(c))
