<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./docs/assets/logo-dark.svg">
  <img alt="Wolf logo" src="./docs/assets/logo-light.svg">
</picture>

<br>

# The Wolf Scripting Language

Wolf is an expression-based scripting language aiming to be:

- **Easy.** Wolf lets you bring your own paradigms, and provides the tools to use
them ergonomically.
- **Small.** Wolf is built from first principles by composing together a few, well
chosen features that are easily identified and repurposed.
- **Smart.** Wolf code can be analysed 100% statically with smart inference, so
you don't have to spell things out for the computer.
- **Parallel.** Wolf code avoids prescribing a specific order or method of running
code, so it can be rearranged or broken up across threads for performance.

## The Wolf Book

Visit https://wolf.phfox.net/ to read the Wolf Book, which describes the Wolf programming language's design and philosophy.

## `wf` - the Wolf Reference Implementation

`wf` is the Wolf Reference Implementation, a Rust-based set of language tools which focus on providing a readable implementation, and correctly processing Wolf programs.

See the `wf/` directory for more information.

## License

Licensed the same way as all of my open source projects: BSD 3-Clause + Security Disclaimer.

As with all other projects, you accept responsibility for choosing and using this project.

See [LICENSE](./LICENSE) or [the license summary](https://github.com/dphfox/licence) for details.
