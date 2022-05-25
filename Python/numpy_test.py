import numpy as np

a = np.array([0, 30, 45, 60, 90])

print('angles', a)
print('sine(a)', np.sinc(a * np.pi / 180))

a = np.arange(9, dtype=np.float_).reshape(3,3)
b = np.array([10,10,10])

print("a: ", a)
print("b: ", b)
print("a * 2", a * 2)
print("a + 2", a + b)
print("a * b", a * b)
print("aberage(a)", np.average(a))
print("mean(b)", np.mean(b))
