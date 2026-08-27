# Kruste - ***K**eyboard-based **Rust**y **E**ditor*

*Kruste is an command-line, keyboard only text editor written in Rust using Ratatui & Crossterm.*

<hr>

### Open a file

`kruste <path>`

> [!NOTE]
> If the file at the given path doesn't exist, Kruste automatically creates the file.

<hr>

### Customization

Kruste will look for an custom configuration at the path set in the `KRUSTE_CONFIG` enviroment variable.

I recommend using the following location for your Kruste configuration:

**Windows** `C:\Users\<user>\.config\kruste\config.json`

**UNIX** `~/.config/kruste/config.json`

If Kruste can't find anything in that enviroment variable, e.g. when the variable isn't set, the editor will use the default config, which in JSON format would look like this:
```json
{
    "theme": {
        "colors": {
            "text": "#FFFFFF",
            "background": "#000000",
            "border": "#FFFFFF"
        }
    }
}
```

> [!WARNING]
> Kruste doesn't automatically generate any config.json file, even if it says, it uses the default config. The actual config.json has to be created seperatly.

> [!WARNING]
> The configuration system is still in development, so right now the only thing to configure are the text, background and border colors. You can take the example above as template.

### Bugs & Issues

If you have a bug, please open up an Issue and describe, what happened and how to replicate the bug.