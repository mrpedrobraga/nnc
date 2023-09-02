# `nnc`, the `nano` compiler

The first official compiler for `nano`, written in rust.

Learn about [nano](http://mrpedrobraga.com/nano).

> 🚧 WORK IN PROGRESS  
> While the information here presented is in the present tense, most of this hasn't been implemented yet.  
> `nano`'s still a baby, experimental project.  
> To be blunt, do not have any hopes.

In the `nnc` executable there is everything you need to make nano programs: A compiler, debugging tools, linter, LSP server and a nano library you can import for your metaprogramming needs.

## Usage

Execute

```bash
$ nnc compile <filename>
```

on the folder where your &lt;filename&gt; file is.

Instead of having lots of compilation flags, compilation details are specified via nano annotations inside your code.

It's recommended to have an entry file (call it `index.nano`) with your compilation parameters, which then calls your entry point function.

```nano
#%compilation.output {
  windows_x86: 'dist/program.exe'
  nbbc: 'dist/program.nb8'
  wasm: 'program.wasm'
}

#%optimization {
  keep_symbol_names: yes
  force_always_inline: yes
}

#%also_include [
  "/lib/*"
]

import 'src/main.nano' as entry_point
entry_point.main()
```

## Installation

`nnc` is portable, just call it from the CLI.

Maybe put it somewhere on your PATH, idk.

## Contributing

Not accepting contributions yet!
