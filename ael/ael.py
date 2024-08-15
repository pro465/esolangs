from copy import deepcopy
from dataclasses import dataclass
import sys

@dataclass
class Push:
    data: object

def isdigit(c): return c in "0123456789"

def parse(prog):
    res=[]
    while len(prog)>0:
        prog=prog.strip()
        if prog[0]=='[':
            c=0
            i=1
            while c>0 or prog[i]!=']':
                if prog[i]=="'": 
                    i+=1
                    continue
                if prog[i]=='[': c+=1
                elif prog[i]==']': c-=1
                i+=1

            res.append(Push(parse(prog[1:i])))
            prog=prog[i+1:]
        elif prog[0]=="'":
            res.append(Push(ord(prog[1])))
            prog=prog[2:]
        elif isdigit(prog[0]):
            i=0
            while i<len(prog) and isdigit(prog[i]): i+=1
            res.append(Push(int(prog[:i])))
            prog=prog[i:]
        else:
            res.append(ord(prog[0]))
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
    return stack.pop()

def get_idx(stack):
    idx = stack.pop()
    return idx%len(stack[-1])

def run(stack, prog):
    for ins in prog:
        if isinstance(ins, Push): 
            stack.append(ins.data)
            continue
        if isinstance(ins, list): 
            stack.append(ins)
            continue

        ins=chr(ins)

        if ins == '$': stack.pop()
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
        elif ins == 'r':
            n = get_n(stack)
            if n==0: continue
            stack.append(deepcopy(stack[-n-1]))

        elif ins == '`': stack[-1]=op(lambda x,y: -y, 0, stack[-1])
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
        elif ins == '_': stack.append(int(input().strip()))
        elif ins == '#': 
            print_pretty(stack.pop())
            print()
        elif ins == '.': print_bytes(stack.pop())
        
        elif ins == '@':
            idx = get_idx(stack)
            l = stack.pop()
            stack.append(l[idx])
        elif ins == '&':
            obj = stack.pop()
            idx = get_idx(stack)
            stack[-1][idx]=obj
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

        elif ins == 'm':
            l=[]
            block=stack.pop()
            for i in stack.pop():
                stack.append(i)
                run(stack, block)
                l.append(stack.pop())
            stack.append(l)
        elif ins == 'e':
            block=stack.pop()
            for i in stack.pop():
                stack.append(i)
                run(stack, block)
        elif ins == 'l':
            l=[]
            block=stack.pop()
            cond=stack.pop()
            run(stack, cond)
            while stack.pop():
                run(stack, block)
                run(stack, cond)

        elif ins == 'i':
            l=[]
            block=stack.pop()
            if stack.pop():
                run(stack, block)

        else: print(ins, end='')

with open(sys.argv[1]) as f:
    run([], parse(f.read()))
