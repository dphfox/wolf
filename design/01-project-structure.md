# Project structure

## Scripts

Wolf scripts contain the source code of the program.

They are saved as text files ending in `.wf`. File names are usually
`snake_case`.

## Workspaces

A workspace is a directory whose scripts will be considered together.

A directory is made into a workspace by adding a file named `.wfspace` exactly.

## Nesting

By default, scripts live side by side in the worskpace. However, scripts may be
nested inside of other scripts.

To do this, create a directory with the script's name (except for the `.wf`).
Any scripts in that directory will be nested under the script in the parent
directory.