import os
import pathlib
import platform
from ctypes import *

pf = platform.system().lower()

# windows
if 'windows' == pf:
    libfile = 'mycalc_python.dll'
elif 'darwin' == pf:
    libfile = 'mycalc_python.dylib'
else:
    libfile = 'mycalc_python.so'

# 경로 지정
libpath = os.path.join(pathlib.Path(os.path.abspath('.')).parent.absolute(), 'target', 'debug', 'lib{}'.format(libfile))

print('libpath : {}'.format(libpath))

mycalc = cdll.LoadLibrary(libpath)

# Rust library 실행
print(mycalc.mul(100, 8))
print(mycalc.mul(8, 9))
