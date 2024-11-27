<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./docs/assets/logo-dark.svg">
  <img alt="Wolf logo" src="./docs/assets/logo-light.svg">
</picture>

<br>

# The Wolf Scripting Language

An experimental design for a static declarative logic language that transpiles
to Luau.

## Goals

- *Principled.* Wolf is built from first principles by composing together minimal, powerful features. Prior art is acknowledged, but freely moved beyond.
- *Consistent.* Similar things appear similar on the page, and divergent things do not.
- *Analysable.* Everything in Wolf is designed for static analysis, allowing
for smart safety checks without annotating everything for the computer.
- *Declarative.* In Wolf, processes are described, not organised. Step-by-step
instructions are not necessary to achieve a task.
- *Workspaced.* Instead of file-based organisation, Wolf advocates for coarser
workspace-centric access control and importing, reducing workflow friction.

## `wf` - the Wolf Reference Implementation

There is no reference implementation at this time, and there probably won't be for a while.