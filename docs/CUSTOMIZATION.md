# Customization

If Kruste can't find anything in that enviroment variable, e.g. when the variable isn't set, the editor will use the default config, which in JSON format would look like this:
```json
{
    "theme": {
        "colors": {
            "text": "#FFFFFF",
            "background": "#000000",
            "border": "#FFFFFF",
            "lines": {
                "linenumber_fg": "#FFFFFF",
                "linenumber_bg": "#808080",
                "cursorline_fg": "#FFFFFF",
                "cursorline_bg": "#4b4b4b",
            },
        },
        "settings": {
            "cursorline": {
                "modifier": "",
            },
        },
    }
}
```

<hr>

### Colors

Changeable colors:
- Text (aka Forground/fg)
- Background (aka bg)
- Border: Frames the editor
- Lines
    - Linenumber fg & bg
    - Cursorline fg & bg

<hr>

### Settings

Changeable settings:
- Cursorline Modifier
    - "BOLD" => Modifier::BOLD
    - "DIM" => Modifier::DIM
    - "ITALIC" => Modifier::ITALIC
    - "UNDERLINED" => Modifier::UNDERLINED
    - "SLOW_BLINK" => Modifier::SLOW_BLINK
    - "RAPID_BLINK" => Modifier::RAPID_BLINK
    - "REVERSED" => Modifier::REVERSED
    - "HIDDEN" => Modifier::HIDDEN
    - "CROSSED_OUT" => Modifier::CROSSED_OUT
    - "" => Modifier::empty() -> No modifier