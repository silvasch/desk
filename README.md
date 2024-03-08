# notiz

Notiz (pronounced `/noˈtiːts/`, the german word for note) is a tool that allows to to quickly save some text for later.

- [Usage](#usage)
- [Installation](#installation)

## Usage

```bash
$ echo "Hello, World!" | notiz # Store "Hello, World!" in a new note
# -> Stored the data in the note 'jolly-reading'

$ notiz # Print the contents of the last created/accessed note
# -> Hello, World! 

$ notiz -l # List all notes
# -> jolly-reading*
#    ruddy-speaker

$ notiz -n ruddy-speaker # Access a note that is not the last used one
# -> Hello, Mars!

$ echo "Hello, Jupiter!" | notiz -n jupiter-greeting # The -n flag can also be used to specify the name for a new note, instead of generating one
# -> Stored the data in the note 'jupiter-greeting'

$ notiz -i # Print more information about a note
# -> Hello, Jupiter!
#    Accessing note 'jupiter-greeting'
#    Description: ''
#    Created: 2024-03-08 07:41:21
#    Last accessed: 2024-03-08 07:43:06.343292454 +01:00

$ notiz -d "A greeting for beings from other planets" # Set a description for a note
$ notiz -l # The description is shown on the list
# -> jolly-reading
#    ruddy-speaker - A greeting for beings from other planets*
```

...and more!

## Installation

Using cargo:
```bash
$ cargo install notiz
```
