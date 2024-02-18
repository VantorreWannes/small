# small
File format for storing lots of data in a super small format. Slow to read and write.

## .sml types
```
bool: 1 bit,
char: [8 bits; 2 bits],
number: (bool, [4 bits; 3 bits]),
float: (bool, e, m),
option<y>: (bool, y) || (bool),
array<y>: [y; number],
struct<a, b, c>: (bool, a, b, c, bool),
```

### special type rules
- When a type is used inside an array, the its type is usually declared only once at the start of the array.
- If a array's length is set to 0 then the next item that follows after the array is a continuation of that array.
- If a type is used inside a struct then its type is repeated each time it occurs in a field.

### .sml item layout
[type, variant, value]

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
