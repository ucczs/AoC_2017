import re

with open('input.txt') as f:
    lines = f.readlines()

for line in lines:
    print(line)

removeIgnored = re.sub('!\S', '', lines[0])

inGarbage = False
garbage = ""
withoutGarbage = ""
ignoreNext = False

for c in removeIgnored:
    if c == "<" and not inGarbage:
        inGarbage = True
    elif c == ">":
        inGarbage = False
    else:
        if inGarbage:
            garbage += c

    if not inGarbage:
        withoutGarbage += c

sum = 0
currentScore = 0
for c in withoutGarbage:
    if c == '{':
        currentScore += 1
    if c == '}':
        sum += currentScore
        currentScore -= 1

print()
print(sum)
print(len(garbage))