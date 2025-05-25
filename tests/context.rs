// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[test]
fn use_test() {
    microcad_lang::microcad_test!(
        r#"
        // use debug from `std/module.µcad`
        use std::debug;
        debug::assert(true);

        // use all symbols from std::debug for test checks
        use std::debug::*;

        // use symbol `circle` in file `geo2d.µcad`
        use std::geo2d::circle;
        assert_valid(circle);

        // use all symbols in file `geo3d.µcad`
        use std::geo3d::*;
        assert_valid(sphere);
        assert_valid(cube);

        // alias `bar` in `std/text/foo.µcad` into `baz`
        use std::test::foo::bar as baz;
        assert_valid(baz);

        // use print from `std/module.µcad`
        use std::print;
        assert_valid(print);
        print("Hello");

        // public use algorithm from `std/module.µcad`
        pub use std::algorithm;
        assert_valid(algorithm);
        assert_invalid(use_test::algorithm);

        module my_module() {
            circle(radius = 1);
            sphere(radius = 1);
        }

        assert_valid(my_module);
    "#
    );
}

#[test]
fn locals_test() {
    microcad_lang::microcad_test!(
        r#"
        // This tests the local stack
        use std::debug::*;
        // new local variable #1
        i = 1;
        {
            // accessing #1
            assert(i == 1);
            // new local variable #2 with same name
            i = 2;
            // accessing #2
            assert(i == 2);
        }
        // accessing #1
        assert(i == 1);
        // concentric namespaces
        {
            {
                {
                    // accessing #1
                    assert(i == 1);
                }
            }
        }
        // overwrite #1
        i = 3;
        // access #1
        assert(i == 3);
        p = std::math::pi;
        q = p > 3.;
        assert(q);
    "#
    );
}
