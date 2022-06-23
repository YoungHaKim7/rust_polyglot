

x <- seq(-10, 10, 0.1)
# plot(x, sin(cos(x)), type = "l", col = "orange", main = "Sine Function")
# plot(x, 10 * sin(cos(tan(x))), type = "l", col = "orange", main = "Sine Function")
plot(x, sin(x) * cos(x), type = "l", col = "orange", main = "Sine Function")
grid()
