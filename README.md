<h1>nexsqs_useless_macro - .num</h1>
<h5>A not so completely useless macro DSL</h5>
<h6>(written in rust)</h6><br>

<h4>&nbsp;• syntax</h4>

  <span>&nbsp;DotNum is interpreted.<br>
  &nbsp;It tokenizes the code, so the syntax is very lenient.<br>
  &nbsp;Semicolons are only required when using multiple commands on a single line.<br>
  &nbsp;Comments can be used anywhere</span>

```
sleep(200); print("this is valid")

if (true) { print("so is this") }

if(true){ # <- no spaces needed here
  print("this is also valid")
}
```
<br>
<h4>&nbsp;• control flow</h4>

  &nbsp;**❯** <code>if, elif, else</code><br>

```
if (false) {
  break
} elif (false) {
  break
} else {
  print("hello world")
}
```

  &nbsp;**❯** <code>while</code><br>

```
while (true) {
  print("hello world")
  sleep(200)
}
```

  &nbsp;**❯** <code>loop</code><br>

```
loop (3) {
  print("this will print 3 times")
  continue
  print("this will never print")
}
```
<br>
<h4>&nbsp;• declarations</h4>

  &nbsp;**❯** <code>function</code><br>

```
def foo(x, y) {
  if ((x == 0 && y == 0) || x > 2) {
	print("both numbers = 0, or x > 2")
  }
}

foo(3, 0)
```
```
def baz(x) {
  x = x + 2
  return x
}

# this will print 4
print(baz(2) + " is the number!")
```

  &nbsp;**❯** <code>async</code><br>

```

```

  &nbsp;**❯** <code>await</code><br>

```

```
<br>
<h4>&nbsp;• built-in functions</h4>
  &nbsp;❯ <code># &lt;comment&gt;</code><br>
  &nbsp;❯ <code>print(&lt;argument&gt;)</code><i> prints an argument</i><br>
  &nbsp;❯ <code>sleep(&lt;milliseconds&gt;)</code><i> sleeps for a given duration (blocks the thread)</i><br>
  &nbsp;❯ <code>click(&lt;key&gt;)</code><i> clicks the key</i><br>
  &nbsp;❯ <code>press(&lt;key&gt;)</code><i> keeps the key pressed</i><br>
  &nbsp;❯ <code>release(&lt;key&gt;)</code><i> releases the key</i><br><br>

<details><summary><span>Example macro.num:</span></summary><br>

```
# config
x = 500
y = 500
d = 1

# moves the mouse smoothly to a point
def move(x, y, d) {
	var currentX = get_mouse(hor)
	var currentY = get_mouse(ver)

	while(currentX != x || currentY != y) {
		currentX = get_mouse(hor)
		currentY = get_mouse(ver)

		if(currentX < x) {
			mouse(1, 0, rel)
			sleep(d)
		} elif(currentX > x) {
			mouse(-1, 0, rel)
			sleep(d)
		}

		if(currentY < y) {
			mouse(0, 1, rel)
			sleep(d)
		} elif(currentY > y) {
			mouse(0, -1, rel)
			sleep(d)
		}
	}
}

# runs the function on RAlt release
async {
	while(true) {
		await(RAlt) {
			await(!RAlt) {
				beep(300)
				move(x, y, d)
			}
		}
	}
}
```
</details>
