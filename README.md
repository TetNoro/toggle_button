# Ratatui Minimal Toggle Button

A minimal and probably bad code library for create a toggle button.

# How to use

**Create**

```
Toggle::new(Some(String::from("Hello world!")))
```

**Toggle**

```
toggle.state.toggle();

toggle.state = ToggleState::Disable

toggle.state = ToggleState::Enable
```

**Render**

```
frame.render_stateful_widget(toggle.clone(), f.area(), &mut toggle.state);
```

**Personalited**

```
let mut toggle = Toggle::new(Some(String::from("It's an example!")));
toggle.icon_disable = "(x )".to_string();
toggle.icon_enable = "( X)".to_string();
```