f = open("iodine_in_salt.csv", "r")
s = f.read()
io = {}
for line in s.split('\n'):
	words = line.split(' ')
	#print(words)
	if line == "":
		continue
	assert(len(words) >= 2)
	value = int(words[0])	
	location = " ".join(words[1:]).upper()
	print(value)
	print(location)
	try:
		io[location]
	except KeyError:
		io[location] = []
	io[location].append(value)
	

print(io)
def average(x):
	assert(x != None)
	assert(len(x) >= 0)
	s = 0.0
	for i in x:
		s = s + i
	return s/len(x)
ans = io
for l in io:
	values = io[l]
	ans[l] = average(values)

print(ans)
sort_ans = sorted(ans.items(), key=lambda x: x[1], reverse=True)

print(sort_ans)

v = []
for x in sort_ans:
	v.append(x[1])
print(v)
del v

	
#150 - 1100
ad = []
for a in sort_ans:
	ad.append((a[0], 100*150/a[1], 100*1100/a[1]))

print(ad)