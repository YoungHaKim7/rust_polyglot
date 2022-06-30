HelloWorld: HelloWorld.o
	ld -macosx_version_min 12.0.0 -o HelloWorld HelloWorld.o -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path` -e _start -arch arm64 


HelloWorld.o: HelloWorld.asm
	as -o HelloWorld.o HelloWorld.asm
