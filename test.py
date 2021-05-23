#!/usr/bin/env python
import sys, peppi_py

game = peppi_py.game(sys.argv[1])

print(game['metadata'])
print(game['frames'][-1]['ports']['0']['leader']['post']['state'])
