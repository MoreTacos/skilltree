# Dependency of main skilltree webserver. 
I am converting the SVG into a handlebars template with the right information.

## Built With
At first I was going to use regex, but it's easier to do the same thing with only built in rust functions. The library looks out for the <rect> block and adds the correct information to the SVG file to transform it into a handlebars template.
