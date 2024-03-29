sin_mu_cos <- function(scos_M) {
    scos_R <- sin(scos_M) * cos(scos_M)
    return(scos_R)
}

x <- seq(-10, 10, 0.1)
plot(x, sin_mu_cos(x), type = "l", col = "orange", main = "Sine Function")
grid()
