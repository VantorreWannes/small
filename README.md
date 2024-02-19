# small
File format for storing lots of typed data in a small format. Slow to read and write.

## .sml types
```
bool: 1 bit,
char: [8 bits; 2 bits],
number: (bool, [4 bits; 3 bits]),
float: (bool, float),
option<y>: (bool, y) || (bool),
array<y>: [y; number],
struct<a, b, c>: (number, a, b, c),
```

### special type rules
- When a type is used inside an array, the its type is usually declared only once at the start of the array.
- If a type is used inside a struct then its type is repeated each time it occurs in a field.

### .sml item layout
[type, value]

## Example file layout
```
{
    number
    array<char>,
    array<
            struct<
                float, 
                option<
                    array<number>
                >, 
                array<char>
            >;
        number
    >,    
}
```
