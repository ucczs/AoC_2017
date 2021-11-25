import re

registers = {}

with open('input.txt') as f:
    lines = f.readlines()

for line in lines:
    x = re.search("(.+) (.+) ([-0-9]+) if (.+) ([<>=!]+) ([-0-9]+)", line)

    targetRegister = x.group(1)
    action = x.group(2)
    value = int(x.group(3))

    conditionalRegister = x.group(4)
    compareAction = x.group(5)
    compareValue = int(x.group(6))

    if targetRegister not in registers.keys():
        registers[targetRegister] = 0

    trueValue = 0
    if conditionalRegister in registers.keys():
        trueValue = registers[conditionalRegister]

    comparisonSuccessful = eval(str(trueValue) + " " + compareAction + " " + str(compareValue))

    if comparisonSuccessful:
        if action == "inc":
            registers[targetRegister] += value
        else:
            registers[targetRegister] -= value

result = max(registers.values())
print(line)