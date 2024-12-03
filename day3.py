import re

pattern = r'mul\((\d{1,3},\d{1,3})\)'

def cal_sum(line):
    total = 0
    pairs = re.findall(pattern, line)
    for pair in pairs:
        x, y = pair.split(',')
        total += int(x) * int(y)
    return total

with open('inputs/day3.txt', 'r') as file_in:
    total = 0
    instruction = file_in.read()
    instruction = re.sub(r"don't\(\).*?($|do\(\))", '', instruction, flags=re.DOTALL)
print(cal_sum(instruction))