x = 1
while x >=0:
    print("my first calculator")
    fn = float(input("first number is..:  "))
    sn = float(input("second number is .. : "))
    calc = input("so... what kind of calculation type would u like? (+,-,*,p(power),n(negative),r(reminder): ")
    x= x-1

    def cal(a, b, calc):
        if calc == "+":
            return (a + b)

        elif calc == "-":
            return (a - b)
        
        elif calc == "/":
            return (a / b)

        elif calc == "*":
            return (a * b)

        elif calc == "p":
            return (a ** b)

        elif calc == "n":
            return (-a)

        elif calc == "r":
            return (a % b)

    print("result is .... : " + str(cal(fn, sn, calc)))
    repl = input("do u what to use more calculators? say y or n (defalut value is y)")
    if repl == "y" or None:
        continue
    else:
        print("program shutdown. ")
        break
