# Design doc for the markdown to html parser

## Scope : 
Implementation of the basic markdown syntax by the [commonmark specification](https://commonmark.org/).  

### what all will be implemented :

- Headings - h1 to h6 using the # to ###### syntax
- Italics
- Bold
- Links
- Images
- Blockquotes
- Unordered lists - using "-" or "*"
- Ordered Lists - 1. , 2. etc.
- horizontal rule using the "***" style
- Inline code using backticks
- code block using three backticks

## How the input file will be processed
Input markdown file will be checked to see if it contains the .md extention in the filename to confirm if its a markdown file.  

A basic template of a html document will be generated in the output .html file and then the contents of the metadata would be used to generate title. After which the file will be further processed and its corresponding content and appropriate tags will be inserted into the html file.

The file will then be streamed in lines and each line will be evaluated.
