Small crate that adds the [`TextOutline`](bevy_slow_text_outline::prelude::TextOutline) component.

## Example

Add the plugin:
```rs
app.add_plugins(SlowTextOutlinePlugin::default());
```

Add the component to your UI node with text:
```rs
commands.spawn((Text::new("Hello, World!"), TextOutline{ width: 1.0, ..default() }));
```

## Demo

Here is the `demo` example's output on a Mac, which has a scaling factor of 2.0.



## Performance

The current implementation is naive and has catastrophic performance degredation scaling with outline width. To avoid melting your GPU, widths are capped at 8 pixels by default (after scaling factors are applied). The max width can be adjusted with [`SlowTextOutlinePlugin`](bevy_slow_text_outline::prelude::SlowTextOutlinePlugin).

## Bevy compatibility

| `bevy` | `bevy_slow_text_outline` |
|-------|-------------------|
| 0.16  | 0.1.0 - main     |
