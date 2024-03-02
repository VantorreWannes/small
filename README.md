# small
Statically typed file format for storing lots of typed data in a small format.

## .sml types
```
void: 0 bit,
bool: 1 bit,
char: [8 bits; 2 bits],
number: (bool, [4 bits; 3 bits]),
float: (bool, float),
option<y>: (bool, y) || (bool),
array<y>: [y; number],
struct<a, b, c, ...>: (number, a, b, c),
enum<number, a, b, c, ...>: (number, struct<void, void, a, b, void, c>), //todo!
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
    string,
    array<
        struct<
            float, 
            option<
                array<number>
            >,
            array<float>
        >,
        struct<
            float, 
            option<
                array<number>
            >, 
            array<float>
        >;
        number
    >,    
}
```
