## Mandlebrot Set in Rust

Wikipedia article on plotting algorithms for the [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set).

Pseudo Code
---

```
for each pixel (Px, Py) on the screen:
  x0 = scaled x coordinate to lie between -2.5 and 1
  y0 = scaled y coordinate to lie between -1 and 1
  x = 0.0
  y = 0.0
  iteration = 0
  max_iteration = 1000
  while (x*x + y*y <= 4 and iteration < max_iteration):
    xtemp = x*x - y*y + x0
    y = 2*x*y + y0
    x = xtemp
    itearation = iteration + 1
  color = palette[iteration]
  put color at coordinates Px, Py
```

Result Image
---
![mandelbrot](example.png)
