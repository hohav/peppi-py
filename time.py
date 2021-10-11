#!/usr/bin/env python
import sys, peppi_py, timeit

print(timeit.timeit(lambda: peppi_py.game(sys.argv[1]), number=10))
