def is_decimal(x):
	assert(type(x) == str)
	found_period = false
	for d in x:
		if not d.isnumeric():
			if d == '.':
				if not found_period:
					found_period = true
				else:
					return false
			else:
				return false
	return true

def verify(x):
	total = x[0]
	if not is_decimal(total):
		return false
	else:
		total = float(total)
for i in x:
		is_decimal(i)
	
s = "3.33 0.03 0.03 2.17 0.063 0.912 0.093 0.032"
print(verify(s.strip().split(" ")))
Numeric
	int N
	dec N
impl Numeric
	print(self) ([Char])
		Append(print(self.int) Char::Period print(self.dec))
	
let n [Numeric]
let s print#(n)
