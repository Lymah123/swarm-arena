#!/usr/bin/env python3
import sys

code = r"""[<+>>>>>>>>++++++++++<<<<<<<-]>+++++[<+++++++++>-]+>>>>>>+[<<+++[>>[-<]<[>]<-]>>[>+>]<[<]>]>[[->>>>+<<<<]>>>+++>-]<[<<<<]<<<<<<<<+[->>>>>>>>>>>>[<+[->>>>+<<<<]>>>>]<<<<[>>>>>[<<<<+>>>>-]<<<<<-[<<++++++++++>>-]>>>[<<[<+<<+>>>-]<[>+<-]<++<<+>>>>>>-]<<[-]<<-<[->>+<-[>>>]>[[<+>-]>+>>]<<<<<]>[-]>+<<<-[>>+<<-]<]<<<<+>>>>>>>>[-]>[<<<+>>>-]<<++++++++++<[->>+<-[>>>]>[[<+>-]>+>>]<<<<<]>[-]>+>[<<+<+>>>-]<<<<+<+>>[-[-[-[-[-[-[-[-[-<->[-<+<->>]]]]]]]]]]<[+++++[<<<++++++++<++++++++>>>>-]<<<<+<->>>>[>+<<<+++++++++<->>>-]<<<<<[>>+<<-]+<[->-<]>[>>.<<<<[+.[-]]>>-]>[>>.<<-]>[-]>[-]>>>[>>[<<<<<<<<+>>>>>>>>-]<<-]]>>[-]<<<[-]<<<<<<<<]++++++++++."""

prepend_data = "47742b8952e4396e605d1edd1ce29f8"

memory = [0] * 30000
# Initialize memory with prepend data
for i, char in enumerate(prepend_data):
    memory[i] = ord(char)

ptr = 0
pc = 0

# Precompute bracket matches
bracket_map = {}
stack = []
for i, char in enumerate(code):
    if char == '[':
        stack.append(i)
    elif char == ']':
        if stack:
            left = stack.pop()
            bracket_map[left] = i
            bracket_map[i] = left

while pc < len(code):
    cmd = code[pc]
    
    if cmd == '>':
        ptr += 1
        if ptr >= len(memory):
            ptr = len(memory) - 1
    elif cmd == '<':
        if ptr > 0:
            ptr -= 1
    elif cmd == '+':
        memory[ptr] = (memory[ptr] + 1) % 256
    elif cmd == '-':
        memory[ptr] = (memory[ptr] - 1) % 256
    elif cmd == '.':
        sys.stdout.write(chr(memory[ptr]))
        sys.stdout.flush()
    elif cmd == ',':
        memory[ptr] = 0
    elif cmd == '[':
        if memory[ptr] == 0:
            pc = bracket_map.get(pc, pc + 1)
    elif cmd == ']':
        if memory[ptr] != 0:
            pc = bracket_map.get(pc, pc - 1)
    
    pc += 1

print()
