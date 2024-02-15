# small
File format for storing lots of data in a super small format. May be slow to read and write.

## .sml type layout
```
bool: 1 bit,
char: x bits,
i8: x bits,
i16: x bits,
i32: x bits,
i64: x bits,
i128: x bits,
u8: x bits,
u16: x bits,
u32: x bits,
u64: x bits,
u128: x bits,
char: x bits,
f32: x bits,
f64: x bits,
array<x>: [x; u8],
struct: {X, Y, Z, ...},
option<x>: {bool, x} || {bool},
```

## Example file layout
```
char: x bits,
i8: x bits,
i16: x bits,
i32: x bits,
i64: x bits,
i128: x bits,
u8: x bits,
u16: x bits,
u32: x bits,
u64: x bits,
u128: x bits,
char: x bits,
f32: x bits,
f64: x bits,
---
bool,
[char; u8],
[char; u8],
[char; u8],
[
    {
        bool,
        [char; u8],
        [char; u8],
        option<f32>,
    },
    {
        bool,
        [char; u8],
        [char; u8],
        option<f32>,
    },
    {
        bool,
        [char; u8],
        [char; u8],
        option<f32>,
    },
    u8,
]
option<{[char; u8], i16}>
```
