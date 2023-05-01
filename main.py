def p(x): 
  for i in range(8):
    for j in range(8):
       print((x >> i * 8) >> j & 1, end="")
    print()

