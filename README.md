# Scrambler
Create words in a nonexistent language

The scrambler is a tool for creating a nonsense language. The primary use case is a Dungeons & Dragons campaign I am creating.
There is an ancient language I want my players to decypher bit by bit. I don't want to invent a language myself. Nor do I want to have to translate stuff. So, I am writing a tool for it.

This tool takes a sentence, scrambles each word to some nonsense that still looks like a word, and gives me the translation. The tool also remembers words it already scrambled so that each word only has a single scrabled counterpart.

Next to the actual use case, this tool is also my first journey into the Rust programming language.

## Taken from my DM notes

### Drow word rules
- A word consists of letters taken from the [Phoenician](https://en.wikipedia.org/wiki/Paleo-Hebrew_alphabet), [Greek](https://en.wikipedia.org/wiki/Greek_alphabet), and [Elder Futhark(Runic)](https://en.wikipedia.org/wiki/Elder_Futhark) alphabets.
- A word has similar length as its English translation. To be precise, the length of a word ranges from half the English length to double the English length. If rounding is needed, round towards the extremes: towards 0 for the lower bound and towards infinity for the upper bound.
- The same letter cannot appear more than two times consequently.

### Mapping tool
Create a tool that can do the following:
- Store the mapping between Drow and English for all known words.
- Look up the English translation for a Drow word and vice versa.
- Generate a new Drow word for an English word. It must first check if the word already exists. If it decides to generate a new word, it must follow the word rules above.
- Generate a new Drow sentence for an English sentence, using known words where applicable and generating new ones when needed.
- When a new word is generated, the user of the tool can choose to reject it. In this case, the tool generates a new word and puts the rejected word on a ban list. (The main reason for this are words that look silly.)
- Translate a Drow sentence back to English.

Because I am using ancient languages, the tool needs full unicode support. That's an interesting programming challenge!
