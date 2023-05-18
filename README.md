# RUSPATCH

本程序将rust源代码中的unsafe函数提取编译成动态链接库，并将源代码中对unsafe函数的调用修改为动态链接库函数的调用。

#### 编译方法：

```shell
cargo build
```

#### 使用方法：

```shell
Usage: ruspatch [OPTIONS] <DIRECTORY>

Arguments:
  <DIRECTORY>  rust project directory

Options:
  -i          inject thread
  -h, --help  Print help
```

成功转换后将会在项目目录中生成动态链接库文件。

需要手动编译修改后的源代码。

将修改后的源代码编译的二进制文件和动态链接库文件放入同一目录下可以执行。

#### 限制

1. rust项目只能编译为可执行文件（没有src/lib.rs）。
2. 源代码中所有use语句都在文件的开头。
3. 源代码中所有的unsafe函数不重名。
4. 不支持范型unsafe函数。
5. 如果unsafe函数的参数和返回值类型没有预导入，可能需要手动导入。

#### 模块说明：

##### toml_handler

给Cargo.toml添加依赖(libloading库和lazy_static库)。

##### file_collector

找出所有.rs文件，生成文件相对路径（src/...）到文件语法树的映射。

##### unsafe_fn_handler

将文件语法树中所有的unsafe函数放在文件的最外层（不放入任何块内），生成映射``[文件相对路径 -> 文件包含的unsafe的函数列表]``和``[unsafe函数名 -> unsafe函数的参数和返回值类型）]``，同时给unsafe函数添加``pub``和``#[no_mangle]``标签。

##### mod_handler

将文件语法树中所有的mod改为pub mod，并写回文件。

##### lib_builder

编译动态链接库

##### unsafe_block_handler

将文件语法树中所有的unsafe函数调用改为对动态链接库中函数的调用。

##### thread_injector

执行线程注入。
