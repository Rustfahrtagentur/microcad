# Modules

[![test](.test/builtin_modules.svg)](.test/builtin_modules.log)

```µcad,builtin_modules
mod a {
    mod b {
        mod c {
            part M1() {}
        }
    }

    part M2() {}
}

a::b::c::M1();
a::M2();
```
