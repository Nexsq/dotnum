<h1>nexsqs_useless_macro - .num</h1>
<h5>A not so completely useless macro DSL</h5>

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

<details><summary><span>Control flow</span></summary><br>

  &nbsp;**•** <code>if, elif, else</code><br>

```
if (false) {
  break
} elif (false) {
  break
} else {
  print("hello world")
}
```

  &nbsp;**•** <code>while</code><br>

```
while (true) {
  print("hello world")
  sleep(200)
}
```

  &nbsp;**•** <code>loop</code><br>

```
loop (3) {
  print("this will print 3 times")
  continue
  print("this will never print")
}
```

</details>
<details><summary><span>Declarations</span></summary><br>

  &nbsp;**•** <code>function</code><br>

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

  &nbsp;**•** <code>async</code><br>

```
# simply spawns a new thread so sleep will not block the main one
# putting async in functions is not recommended
async {
  sleep(2000)
  print("and this after 2 seconds")
}
print("this will run immediately")
```

  &nbsp;**•** <code>await</code><br>

```
# waits till F key is pressed and then released
await (f) {
  print("f was pressed")
  await (!f) {
    print("and f was released")
  }
}
```
</details>
<br>
<h4>&nbsp;• built-in functions</h4>
  &nbsp;❯ <code># comment;</code><br>
  &nbsp;❯ <code>print(argument)</code><i> prints an argument</i><br>
  &nbsp;❯ <code>sleep(milliseconds)</code><i> sleeps for a given duration (blocks the thread)</i><br>
  &nbsp;❯ <code>click(key)</code><i> clicks the key</i><br>
  &nbsp;❯ <code>press(key)</code><i> keeps the key pressed</i><br>
  &nbsp;❯ <code>release(key)</code><i> releases the key</i><br>
  &nbsp;❯ <code>scroll(key, ver/hor)</code><i> scrolls in a direction</i><br>
  &nbsp;❯ <code>mouse(x, y, rel/abs)</code><i> moves the mouse cursor to a position</i><br>
  &nbsp;❯ <code>string("text")</code><i> writes some text (just like you would using a keyboard - outside the program)</i><br>
  &nbsp;❯ <code>time(ms/s/m/h/day/month/year)</code><i> returns current time</i><br>
  &nbsp;❯ <code>random(x, y)</code><i> returns a random number in range x to y</i><br>
  &nbsp;❯ <code>abs(value)</code><i> returns the absolute value</i><br>
  &nbsp;❯ <code>get_mouse(x/y)</code><i> returns mouse cursor coordinates</i><br>
  &nbsp;❯ <code>get_resolution(hor/ver)</code><i> returns screen resolution</i><br>
  &nbsp;❯ <code>get_color(x, y)</code><i> returns hex color of a given pixel</i><br>
  &nbsp;❯ <code>color("#hex", x, y, tolerance)</code><i> returns true if color of a given pixel is right</i><br>
  &nbsp;❯ <code>process(process.exe)</code><i> returns true if process is active</i><br>
  &nbsp;❯ <code>beep(pitch)</code><i> beeps</i><br>
  &nbsp;❯ <code>background()</code><i> makes the program run in background</i><br>
  &nbsp;❯ <code>exit()</code><i> exits the program</i><br><br>

<details><summary><span>Example macro.num</span></summary><br>

```
# config
let x = 1000
let y = 500
let d = 1
let key = RAlt

# moves the mouse smoothly to a point
def move(x, y, d, key) {
	await(key) {

		let currentX = get_mouse(hor)
		let currentY = get_mouse(ver)

		while(abs(currentX - x) > 3 || abs(currentY - y) > 3) {
			currentX = get_mouse(hor)
			currentY = get_mouse(ver)

			if(currentX < x) {
				mouse(3, 0, rel)
				sleep(d)
			} elif(currentX > x) {
				mouse(-3, 0, rel)
				sleep(d)
			}

			if(currentY < y) {
				mouse(0, 3, rel)
				sleep(d)
			} elif(currentY > y) {
				mouse(0, -3, rel)
				sleep(d)
			}
		}
	}
}

# runs the function on key press
async {
	while(true) {
		move(x, y, d, key)
	}
}
```
```
# simple key switch macro
let key = RAlt
let running = false

while(true) {
	await(key) {
		running = true
		async {
			await(!key) {
				await(key) {
					running = false
				}
			}
		}
	}
	while(running) {
		print("running")
		sleep(1000)
	}
}
```
</details>
