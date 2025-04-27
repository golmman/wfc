# wfc

```json
[// image
  [ // pixel
    { // first possible color
      "color": 0,
      "patterns": [
        { colors: [0, 0, 0, 0, 0, 0, 0, 0] }
      ]
    }
  ]
]
```



```
struct ImageSuperposition<T> {
  width: u32,
  height: u32,
  pixels: Vev<PixelSuperposition>,
}

struct PixelSuperposition {
  possible_colors: Vec<ColorSuperposition>,
}

struct ColorSuperposition {
  color: Color,
  patterns: Vec<Pattern>,
}

struct Pattern8 {
  colors: Vec<Color>,
}

trait Pattern<const N: usize> {
  get_color(&self) -> [Color; N];
}
```
