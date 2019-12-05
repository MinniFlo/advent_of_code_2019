file = open('input.txt', 'r')
sum = 0
for line in file:
    num = int(line.strip())
    num = (num // 3) - 2
    sum += num
    while True:
        num = (num // 3) - 2
        if num > 0:
            sum += num
        else:
            break
print(sum)
