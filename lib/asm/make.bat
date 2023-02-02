:: call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

ml64 -c sha.asm
clang -shared sha.obj -o sha_asm.dll --target=x86_64-pc-windows-msvc -W1,"/DEF:sha.def"
dumpbin /exports sha_asm.dll