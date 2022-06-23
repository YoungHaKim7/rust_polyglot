# R - plot( )함수를 이용한 삼각함수 시각화
# https://m.blog.naver.com/padosori60/220861591002

x <- seq(-10, 10, 0.1)
plot(x, sin(x), type = "l", col = "orange", main = "Sine Function")
grid()
