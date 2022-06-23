
# https://ytliu0.github.io/Stat390EF-R-Independent-Study-archive/Week03/NormalChisqT.html
dnorm(-3:3)

1 / sqrt(2 * pi) * exp(-(-3:3)^2 / 2)
# curve(dnorm(x), xlim = c(-3, 3))

curve(dnorm(x), xlim = c(-6, 6))
curve(dnorm(x, mean = 1, sd = 2), col = "red", add = "TRUE")
