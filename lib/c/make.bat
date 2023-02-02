:: call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

clang sha.c -o sha.obj -c --target=x86_64-pc-windows-msvc
clang -shared sha.obj -o sha_c.dll --target=x86_64-pc-windows-msvc -W1,"/DEF:sha.def"
dumpbin /exports sha_c.dll