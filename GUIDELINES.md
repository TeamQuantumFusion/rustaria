# What is dis?
To keep the codebase clean we have standards that you need to follow in our codebase to make this maintainable.

## Logging
The target name **MUST** be set, and it follows this format.

```
{kind}@{module.path.path}
```

so an example for initializing chunks would be `init@rustaria.chunk`

### Kinds
- init (For first time initialization methods.)
- tick (Anything on the hot `tick` methods)
- draw (Anything on the hot `draw` methods, client only)
- reload (Anything on the reloading methods)
- misc (Anything else)
- plugin (Reserved for plugin logging)