#!/usr/bin/env python
import sys, peppi_py, timeit

peppi_py.read_slippi(sys.argv[1]) # warm up
print(timeit.timeit(lambda: peppi_py.read_slippi(sys.argv[1]), number=10))
