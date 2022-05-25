a.out : Calculator.o CalculatorTutorial.o

	g++ a.out Calculator.o CalculatorTutorial.o


Calculator.o : Calculator.h CalculatorTutorial.cpp

CalculatorTutorial.o: Calculator.h Calculator.cpp

# make clean의 명령어를 수행.
clean:
	rm -rf *.o
	rm -rf $(TARGET)
