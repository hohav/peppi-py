#!/usr/bin/env python
import sys, peppi_py, pprint

game = peppi_py.game(sys.argv[1])

pprint.pprint(game['metadata'])
print(game['frames'][0]['ports']['0']['leader']['post'])
