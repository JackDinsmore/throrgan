import sys
from lib import compile

if len(sys.argv) == 2:
    compile(sys.argv[-1])