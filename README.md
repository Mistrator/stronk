# stronk

Scale Pathfinder 2e creature statistics by using the Building Creatures guidelines in GM Core. This method is more accurate than using Elite and Weak adjustments and allows scaling across the entire level range.

The tool only scales creature numbers. If a creature has spells or special abilities, you may have to adjust them manually to be suitable for the target level.

## Compilation and installation

Install Rust to compile the tool. Rust is not needed to run a compiled binary.

Run `cargo build --release` and copy the resulting binary `target/release/stronk` into your PATH. No other files are necessary.

Alternatively, run the tool with Cargo: `cargo run -- <current_level> <target_level>`. The `--` is necessary if `current_level` is negative because Cargo misinterpretes it as a Cargo option.

## Usage

### Interactive prompt

Start an interactive prompt to scale statistics from `current_level` to `target_level`:
```
$ stronk <current_level> <target_level>
```

Prompt syntax:
```
<statistic_type> <statistic_value>
```

Statistic types:
- `perception / per`
- `acrobatics / arcana / lore / ...`: skills
- `ac`
- `fortitude / fort / reflex / ref / will`
- `hp`
- `resistance`: resistance to a damage type
- `weakness`: weakness to a damage type
- `strike-attack / att`: strike attack bonus
- `strike-damage / dmg`
- `spell-dc`
- `spell-attack`: spell attack bonus
- `unlimited-area-damage`: ability with no usage restrictions
- `limited-area-damage`: ability which can be used only once or has a cooldown

Statistic value is either a number (`15`, `+11`) or a damage expression (`2d6+8 bludgeoning`). A damage expression consists of a dice expression and/or a flat modifier, and must specify the damage type. Any damage type is acceptable. Persistent damage and other non-damage effects such as Knockdown are not supported.

Example prompts:
```
ac 15
fortitude +11
strike-damage 2d6+8 bludgeoning
strike-damage 2d12+17 piercing plus 3d6 fire plus 1 void
```

### Input file

Scale a stat block from `current_level` to `target_level`:
```
$ stronk <current_level> <target_level> <input_file>
```

The stat block is a plain text file containing one statistic per line. Each line follows the same syntax as the interactive prompt.

Comments and empty lines are supported and written to output. Comments start with `#` or `//`.

## How it works

In short, we first calculate the proficiency the creature has for a given statistic and then use this proficiency to rebuild the creature on `target_level` using the tables in Building Creatures guidelines in GM Core.

More specifically, we first look at the row for `current_level` and see where the value of a given statistic lies on that row. We have three cases and scaling methods:

- Exact scaling: The value matches one of the columns. This is the ideal case: we simply output the value of the same column for the `target_level` row.
- Interpolation: The value lies between two columns. First, we calculate the position of the value on the interval between the two columns. Then, we map this relative position to the interval between the same columns on the `target_level` row and interpolate the value at that position.
- Extrapolation: The value is outside the row by some amount. We output a value that is outside the `target_level` row by the same amount.

Damage scaling works the same way: we first calculate the average damage of a given damage expression, scale this number as described, and finally construct a new damage expression for the scaled average damage.

We round down fractional values as usual in Pathfinder, but we also output the fractional value. If this value is very close to the next integer, you may choose to round it up instead.

### Elite and Weak adjustments

Elite and Weak adjustments exaggerate the changes in numbers according to Monster Core. In practice, applying the Elite adjustment to a boss might make it too strong, and applying the Weak adjustment to a mook might make it too weak. Additionally, applying Elite and Weak adjustments multiple times compounds this error, making them unsuitable for scaling by more than one level. This tool is more suitable for such adjustments since we do not exaggerate the changes.

## Disclaimer

The author is not responsible for any character deaths caused by the tool.
