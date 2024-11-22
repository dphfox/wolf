<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./docs/assets/logo-dark.svg">
  <img alt="Wolf logo" src="./docs/assets/logo-light.svg">
</picture>

<br>

# The Wolf Scripting Language

Wolf is an experimental design for a static scripting language that transpiles
to Luau.

The goal of Wolf is to build a language from first principles, composing
together minimal but powerful features. It should be statically analysable, but
lightweight to write.

## Goals

- *Principled.* Wolf is built by composing together minimal, powerful features.
Similar things appear similar on the page, and divergent things do not.
- *Analysable.* Everything in Wolf is designed for static analysis, allowing
for smart safety checks without annotating everything for the computer.
- *Declarative.* In Wolf, processes are described, not organised. Step-by-step
instructions are not necessary to achieve a task.
- *Workspaced.* Instead of file-based organisation, Wolf advocates for coarser
workspace-centric access control and importing, reducing workflow friction.