from copy import deepcopy
from dataclasses import dataclass
import sys

@dataclass
class Push:
    data: object

@dataclass
class Map:
    code: object

def skip_str(prog):
    if prog[0]!='"':
        return 0

    i=1
    while i<len(prog) and prog[i]!='"':
        if prog[i]=='\\': i+=1
        i+=1
    return i+1

def isdigit(c): return c in "0123456789"

def parse(prog):
    res=[]
    while len(prog)>0:
        prog=prog.strip()
        if prog[0]=='"':
            l=skip_str(prog)
            res.append(Push(list(bytes(prog[1:l-1], "utf-8"))))
            prog=prog[l:]
        elif prog[0]=='[':
            c=0
            i=1
            while c>0 or prog[i]!=']':
                i+=skip_str(prog[i:])
                if prog[i]=='[': c+=1
                elif prog[i]==']': c-=1
                i+=1

            res.append(Map(parse(prog[1:i])))
            prog=prog[i+1:]
        elif isdigit(prog[0]):
            i=0
            while i<len(prog) and isdigit(prog[i]): i+=1
            res.append(Push(int(prog[:i])))
            prog=prog[i:]
        else:
            res.append(prog[0])
            prog=prog[1:]
    return res

def getc(): return sys.stdin.buffer.read(1)[0]

def print_bytes(b):
    if isinstance(b, list):
        for i in b: print_bytes(i)
    else:
        sys.stdout.buffer.write(bytes([b%256]))

def print_pretty(b):
    if isinstance(b, list):
        print("{ ", end="")
        for i in b: 
            print_pretty(i)
            print(" ", end="")
        print("}", end="")
    else: print(b, end="")

def adjust_len(a, b):
    i=0
    while len(a)<len(b):
        a.append(a[i])
        i+=1
    while len(b)<len(a):
        b.append(b[i])
        i+=1

def op(f, a, b):
    if isinstance(a, list) and isinstance(b, list):
        adjust_len(a, b)
        return [op(f, a_, b_) for a_, b_ in zip(a, b)]
    if isinstance(a, list):
        return [op(f, a_, b) for a_ in a]
    if isinstance(b, list):
        return [op(f, a, b_) for b_ in b]
    return f(a, b)

def prefixes(l): return [deepcopy(l[:i]) for i in range(len(l))]

def get_n(stack):
    if isinstance(stack[-1], int): return stack.pop()
    return 2

def get_idx(stack):
    if isinstance(stack[-1], int): return stack.pop()
    return 0

def run(stack, prog):
    for ins in prog:
        if isinstance(ins, Push): stack.append(ins.data)
        elif isinstance(ins, Map):
            l=[]
            for i in stack.pop():
                stack.append(i)
                run(stack, ins.code)
                l.append(stack.pop())
            stack.append(l)

        elif ins == '$': stack.pop()
        elif ins == ':': stack.append(deepcopy(stack[-1]))
        elif ins == ';': stack.append(deepcopy(stack[-2]))
        elif ins == '~': stack[-2], stack[-1] = stack[-1], stack[-2]
        elif ins == '!': stack.insert(-2, deepcopy(stack[-1]))

        elif ins == 'd':
            n = get_n(stack)
            if n==0: continue
            stack[-n-1:] = stack[-n:]+[stack[-n-1]]
        elif ins == 'b':
            n = get_n(stack)
            if n==0: continue
            stack[-n-1:] = [stack[-1]]+stack[-n-1:-1]

        elif ins == '`': stack[-1]*=-1
        elif ins == '+':
            y=stack.pop()
            x=stack.pop()
            stack.append(op(lambda x, y: x+y, x, y))
        elif ins == '-':
            y=stack.pop()
            x=stack.pop()
            stack.append(op(lambda x, y: x-y, x, y))
        elif ins == '*':
            y=stack.pop()
            x=stack.pop()
            stack.append(op(lambda x, y: x*y, x, y))
        elif ins == '/':
            y=stack.pop()
            x=stack.pop()
            stack.append(op(lambda x, y: x//y, x, y))
        elif ins == '%':
            y=stack.pop()
            x=stack.pop()
            stack.append(op(lambda x, y: x%y, x, y))

        elif ins == ',': stack.append(getc())
        elif ins == '#': 
            print_pretty(stack.pop())
            print()
        elif ins == '.': print_bytes(stack.pop())
        
        elif ins == '@':
            idx = get_idx(stack)
            l = stack.pop()
            stack.append(l[idx])
        elif ins == '|':
            idx = stack.pop()
            l = stack.pop()
            stack.append(l[:idx])
            stack.append(l[idx:])
        elif ins == '\\': stack[-1] = prefixes(stack[-1])
        elif ins == '?': stack.append([0]*stack.pop())
        elif ins == 'w': stack.append([stack.pop()])
        elif ins == 'c':
            y=stack.pop()
            x=stack.pop()
            stack.append(x+y)
        elif ins == 'p':
            y=stack.pop()
            x=stack.pop()
            stack.append(x+[y])
        else: print(ins)

with open(sys.argv[1]) as f:
    run([], parse(f.read()))
