# Json Parser

Really simple json parser written in rust

Im trying to learn rust more, so the code is really messy.

This parser supports basic types and structure

```json
{
  "hello": "world",
  "int": 69,
  "array": [1, 2, 3],
  "complex": {
    "really": {
      "nested": {
        "stuff": []
      }
    }
  },
  "array_of_objects": [
    {
      "sub_array": ["hello", 1, 2]
    }
  ]
}
```

## Known issues

1. Commas in array are technically not required.

These two arrays are parsed the same way
```json
"array_first": [1, 2, 3]
"array_second": [1 2 3]

```

2. Scientific notation is not supported at all

Numbers like these wont be parsed at all
```json
"number": 12e1
```