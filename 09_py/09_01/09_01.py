import re

with open('input.txt') as f:
    lines = f.readlines()

for line in lines:
    print(line)

removeIgnored = re.sub('!\S', '', lines[0])

inGarbage = False
withoutGarbage = ""

for c in removeIgnored:
    if c == "<":
        inGarbage = True

    if not inGarbage:
        withoutGarbage += c

    if c == ">":
        inGarbage = False

sum = 0
currentScore = 0
for c in withoutGarbage:
    if c == '{':
        currentScore += 1
    if c == '}':
        sum += currentScore
        currentScore -= 1

print(sum)