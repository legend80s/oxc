mod functions;
mod modules;
mod vars;

use crate::tester::Tester;
use crate::{rule::RuleMeta, rules::eslint::no_unused_vars::NoUnusedVars};
use serde_json;

#[allow(clippy::too_many_lines)]
#[test]
fn test_eslint() {
    let pass = vec![
            ("var foo = 5;\n\nlabel: while (true) {\n  console.log(foo);\n  break label;\n}", None),
            ("var foo = 5;\n\nwhile (true) {\n  console.log(foo);\n  break;\n}", None),
            ("for (let prop in box) {\n        box[prop] = parseInt(box[prop]);\n}", None),
            (
                "var box = {a: 2};\n    for (var prop in box) {\n        box[prop] = parseInt(box[prop]);\n}",
                None,
            ),
            ("f({ set foo(a) { return; } });", None),
            ("a; var a;", Some(serde_json::json!(["all"]))),
            ("var a=10; alert(a);", Some(serde_json::json!(["all"]))),
            ("var a=10; (function() { alert(a); })();", Some(serde_json::json!(["all"]))),
            (
                "var a=10; (function() { setTimeout(function() { alert(a); }, 0); })();",
                Some(serde_json::json!(["all"])),
            ),
            ("var a=10; d[a] = 0;", Some(serde_json::json!(["all"]))),
            ("(function() { var a=10; return a; })();", Some(serde_json::json!(["all"]))),
            ("(function g() {})()", Some(serde_json::json!(["all"]))),
            ("function f(a) {alert(a);}; f();", Some(serde_json::json!(["all"]))),
            (
                "var c = 0; function f(a){ var b = a; return b; }; f(c);",
                Some(serde_json::json!(["all"])),
            ),
            ("function a(x, y){ return y; }; a();", Some(serde_json::json!(["all"]))),
            (
                "var arr1 = [1, 2]; var arr2 = [3, 4]; for (var i in arr1) { arr1[i] = 5; } for (var i in arr2) { arr2[i] = 10; }",
                Some(serde_json::json!(["all"])),
            ),
            ("var a=10;", Some(serde_json::json!(["local"]))),
            ("var min = \"min\"; Math[min];", Some(serde_json::json!(["all"]))),
            ("Foo.bar = function(baz) { return baz; };", Some(serde_json::json!(["all"]))),
            ("myFunc(function foo() {}.bind(this))", None),
            ("myFunc(function foo(){}.toString())", None),
            (
                "function foo(first, second) {\ndoStuff(function() {\nconsole.log(second);});}; foo()",
                None,
            ),
            ("(function() { var doSomething = function doSomething() {}; doSomething() }())", None),
            ("try {} catch(e) {}", None),
            ("/* global a */ a;", None),
            (
                "var a=10; (function() { alert(a); })();",
                Some(serde_json::json!([{ "vars": "all" }])),
            ),
            (
                "function g(bar, baz) { return baz; }; g();",
                Some(serde_json::json!([{ "vars": "all" }])),
            ),
            (
                "function g(bar, baz) { return baz; }; g();",
                Some(serde_json::json!([{ "vars": "all", "args": "after-used" }])),
            ),
            (
                "function g(bar, baz) { return bar; }; g();",
                Some(serde_json::json!([{ "vars": "all", "args": "none" }])),
            ),
            (
                "function g(bar, baz) { return 2; }; g();",
                Some(serde_json::json!([{ "vars": "all", "args": "none" }])),
            ),
            (
                "function g(bar, baz) { return bar + baz; }; g();",
                Some(serde_json::json!([{ "vars": "local", "args": "all" }])),
            ),
            (
                "var g = function(bar, baz) { return 2; }; g();",
                Some(serde_json::json!([{ "vars": "all", "args": "none" }])),
            ),
            ("(function z() { z(); })();", None),
            (" ", None),
            ("var who = \"Paul\";\nmodule.exports = `Hello ${who}!`;", None),
            ("export var foo = 123;", None),
            ("export function foo () {}", None),
            // FIXME
            // ("let toUpper = (partial) => partial.toUpperCase; export {toUpper}", None),
            ("export class foo {}", None),
            ("class Foo{}; var x = new Foo(); x.foo()", None),
            (
                "const foo = \"hello!\";function bar(foobar = foo) {  foobar.replace(/!$/, \" world!\");}\nbar();",
                None,
            ),
            ("function Foo(){}; var x = new Foo(); x.foo()", None),
            ("function foo() {var foo = 1; return foo}; foo();", None),
            ("function foo(foo) {return foo}; foo(1);", None),
            ("function foo() {function foo() {return 1;}; return foo()}; foo();", None),
            ("function foo() {var foo = 1; return foo}; foo();", None),
            ("function foo(foo) {return foo}; foo(1);", None),
            ("function foo() {function foo() {return 1;}; return foo()}; foo();", None),
            ("const x = 1; const [y = x] = []; foo(y);", None),
            ("const x = 1; const {y = x} = {}; foo(y);", None),
            ("const x = 1; const {z: [y = x]} = {}; foo(y);", None),
            ("const x = []; const {z: [y] = x} = {}; foo(y);", None),
            ("const x = 1; let y; [y = x] = []; foo(y);", None),
            ("const x = 1; let y; ({z: [y = x]} = {}); foo(y);", None),
            ("const x = []; let y; ({z: [y] = x} = {}); foo(y);", None),
            ("const x = 1; function foo(y = x) { bar(y); } foo();", None),
            ("const x = 1; function foo({y = x} = {}) { bar(y); } foo();", None),
            ("const x = 1; function foo(y = function(z = x) { bar(z); }) { y(); } foo();", None),
            ("const x = 1; function foo(y = function() { bar(x); }) { y(); } foo();", None),
            ("var x = 1; var [y = x] = []; foo(y);", None),
            ("var x = 1; var {y = x} = {}; foo(y);", None),
            ("var x = 1; var {z: [y = x]} = {}; foo(y);", None),
            ("var x = []; var {z: [y] = x} = {}; foo(y);", None),
            ("var x = 1, y; [y = x] = []; foo(y);", None),
            ("var x = 1, y; ({z: [y = x]} = {}); foo(y);", None),
            ("var x = [], y; ({z: [y] = x} = {}); foo(y);", None),
            ("var x = 1; function foo(y = x) { bar(y); } foo();", None),
            ("var x = 1; function foo({y = x} = {}) { bar(y); } foo();", None),
            ("var x = 1; function foo(y = function(z = x) { bar(z); }) { y(); } foo();", None),
            ("var x = 1; function foo(y = function() { bar(x); }) { y(); } foo();", None),
            // ("/*exported toaster*/ var toaster = 'great'", None),
            // ("/*exported toaster, poster*/ var toaster = 1; poster = 0;", None),
            // ("/*exported x*/ var { x } = y", None),
            // ("/*exported x, y*/  var { x, y } = z", None),
            // ("/*eslint use-every-a:1*/ var a;", None),
            // ("/*eslint use-every-a:1*/ !function(a) { return 1; }", None),
            // ("/*eslint use-every-a:1*/ !function() { var a; return 1 }", None),
            ("var _a;", Some(serde_json::json!([{ "vars": "all", "varsIgnorePattern": "^_" }]))),
            (
                "var a; function foo() { var _b; } foo();",
                Some(serde_json::json!([{ "vars": "local", "varsIgnorePattern": "^_" }])),
            ),
            (
                "function foo(_a) { } foo();",
                Some(serde_json::json!([{ "args": "all", "argsIgnorePattern": "^_" }])),
            ),
            (
                "function foo(a, _b) { return a; } foo();",
                Some(serde_json::json!([{ "args": "after-used", "argsIgnorePattern": "^_" }])),
            ),
            (
                "var [ firstItemIgnored, secondItem ] = items;\nconsole.log(secondItem);",
                Some(serde_json::json!([{ "vars": "all", "varsIgnorePattern": "[iI]gnored" }])),
            ),
            (
                "const [ a, _b, c ] = items;\nconsole.log(a+c);",
                Some(serde_json::json!([{ "destructuredArrayIgnorePattern": "^_" }])),
            ),
            (
                "const [ [a, _b, c] ] = items;\nconsole.log(a+c);",
                Some(serde_json::json!([{ "destructuredArrayIgnorePattern": "^_" }])),
            ),
            (
                "const { x: [_a, foo] } = bar;\nconsole.log(foo);",
                Some(serde_json::json!([{ "destructuredArrayIgnorePattern": "^_" }])),
            ),
            (
                "function baz([_b, foo]) { foo; };\nbaz()",
                Some(serde_json::json!([{ "destructuredArrayIgnorePattern": "^_" }])),
            ),
            (
                "function baz({x: [_b, foo]}) {foo};\nbaz()",
                Some(serde_json::json!([{ "destructuredArrayIgnorePattern": "^_" }])),
            ),
            (
                "function baz([{x: [_b, foo]}]) {foo};\nbaz()",
                Some(serde_json::json!([{ "destructuredArrayIgnorePattern": "^_" }])),
            ),
            // NOTE: eslint ignores variables used as iterators in for-in loops
            // when the loop does nothing but return. This is dumb behavior and
            // would add a lot of complexity + performance overhead to support.
            // Unless the community complains, we shouldn't support this
            // see https://github.com/eslint/eslint/issues/2342
            // ("(function(obj) { var name; for ( name in obj ) return; })({});", None),
            // ("(function(obj) { var name; for ( name in obj ) { return; } })({});", None),
            // ("(function(obj) { for ( var name in obj ) { return true } })({})", None),
            // ("(function(obj) { for ( var name in obj ) return true })({})", None),
            // ("(function(obj) { let name; for ( name in obj ) return; })({});", None),
            // ("(function(obj) { let name; for ( name in obj ) { return; } })({});", None),
            // ("(function(obj) { for ( let name in obj ) { return true } })({})", None),
            // ("(function(obj) { for ( let name in obj ) return true })({})", None),
            // ("(function(obj) { for ( const name in obj ) { return true } })({})", None),
            // ("(function(obj) { for ( const name in obj ) return true })({})", None),
            // ("(function(iter) { let name; for ( name of iter ) return; })({});", None),
            // ("(function(iter) { let name; for ( name of iter ) { return; } })({});", None),
            // ("(function(iter) { for ( let name of iter ) { return true } })({})", None),
            // ("(function(iter) { for ( let name of iter ) return true })({})", None),
            // ("(function(iter) { for ( const name of iter ) { return true } })({})", None),
            // ("(function(iter) { for ( const name of iter ) return true })({})", None),
            // FIXME
            // ("let x = 0; foo = (0, x++);", None),
            // ("let x = 0; foo = (0, x += 1);", None),
            // ("let x = 0; foo = (0, x = x + 1);", None),
            (
                "try{}catch(err){console.error(err);}",
                Some(serde_json::json!([{ "caughtErrors": "all" }])),
            ),
            ("try{}catch(err){}", Some(serde_json::json!([{ "caughtErrors": "none" }]))),
            (
                "try{}catch(ignoreErr){}",
                Some(
                    serde_json::json!([{ "caughtErrors": "all", "caughtErrorsIgnorePattern": "^ignore" }]),
                ),
            ),
            ("try{}catch(err){}", Some(serde_json::json!([{ "vars": "all", "args": "all" }]))),
            (
                "const data = { type: 'coords', x: 1, y: 2 };\nconst { type, ...coords } = data;\n console.log(coords);",
                Some(serde_json::json!([{ "ignoreRestSiblings": true }])),
            ),
            ("var a = 0, b; b = a = a + 1; foo(b);", None),
            ("var a = 0, b; b = a += a + 1; foo(b);", None),
            ("var a = 0, b; b = a++; foo(b);", None),
            ("function foo(a) { var b = a = a + 1; bar(b) } foo();", None),
            ("function foo(a) { var b = a += a + 1; bar(b) } foo();", None),
            ("function foo(a) { var b = a++; bar(b) } foo();", None),
            // (
            //     "var unregisterFooWatcher;\n// ...\nunregisterFooWatcher = $scope.$watch( \"foo\", function() {\n    // ...some code..\n    unregisterFooWatcher();\n});\n",
            //     None,
            // ),
            // (
            //     "var ref;\nref = setInterval(\n    function(){\n        clearInterval(ref);\n    }, 10);\n",
            //     None,
            // ),
            // (
            //     "var _timer;\nfunction f() {\n    _timer = setTimeout(function () {}, _timer ? 100 : 0);\n}\nf();\n",
            //     None,
            // ),
            // (
            //     "function foo(cb) { cb = function() { function something(a) { cb(1 + a); } register(something); }(); } foo();",
            //     None,
            // ),
            // ("function* foo(cb) { cb = yield function(a) { cb(1 + a); }; } foo();", None),
            // ("function foo(cb) { cb = tag`hello${function(a) { cb(1 + a); }}`; } foo();", None),
            // ("function foo(cb) { var b; cb = b = function(a) { cb(1 + a); }; b(); } foo();", None),
            (
                "function someFunction() {\n    var a = 0, i;\n    for (i = 0; i < 2; i++) {\n        a = myFunction(a);\n    }\n}\nsomeFunction();\n",
                None,
            ),
            // todo
            // (
            //     "(function(a, b, {c, d}) { d })",
            //     Some(serde_json::json!([{ "argsIgnorePattern": "c" }])),
            // ),
            // (
            //     "(function(a, b, {c, d}) { c })",
            //     Some(serde_json::json!([{ "argsIgnorePattern": "d" }])),
            // ),
            // ("(function(a, b, c) { c })", Some(serde_json::json!([{ "argsIgnorePattern": "c" }]))),
            // (
            //     "(function(a, b, {c, d}) { c })",
            //     Some(serde_json::json!([{ "argsIgnorePattern": "[cd]" }])),
            // ),
            ("(class { set foo(UNUSED) {} })", None),
            ("class Foo { set bar(UNUSED) {} } console.log(Foo)", None),
            // todo
            (
                "(({a, ...rest}) => rest)",
                Some(serde_json::json!([{ "args": "all", "ignoreRestSiblings": true }])),
            ),
            // todo
            // (
            //     "let foo, rest;\n({ foo, ...rest } = something);\nconsole.log(rest);",
            //     Some(serde_json::json!([{ "ignoreRestSiblings": true }])),
            // ),
            // ("/*eslint use-every-a:1*/ !function(b, a) { return 1 }", None),
            ("var a = function () { a(); }; a();", None),
            ("var a = function(){ return function () { a(); } }; a();", None),
            ("const a = () => { a(); }; a();", None),
            ("const a = () => () => { a(); }; a();", None),
            ("export * as ns from \"source\"", None),
            ("import.meta", None),
            // why do these count as passing but +=, -=, etc. doesn't? makes no
            // sense to me.
            // ("var a; a ||= 1;", None),
            // ("var a; a &&= 1;", None),
            // ("var a; a ??= 1;", None),
        ];

    let fail = vec![
            ("function foox() { return foox(); }", None),
            ("(function() { function foox() { if (true) { return foox(); } } }())", None),
            ("var a=10", None),
            ("function f() { var a = 1; return function(){ f(a *= 2); }; }", None),
            ("function f() { var a = 1; return function(){ f(++a); }; }", None),
            // ("/*global a */", None),
            (
                "function foo(first, second) {\ndoStuff(function() {\nconsole.log(second);});};",
                None,
            ),
            ("var a=10;", Some(serde_json::json!(["all"]))),
            ("var a=10; a=20;", Some(serde_json::json!(["all"]))),
            (
                "var a=10; (function() { var a = 1; alert(a); })();",
                Some(serde_json::json!(["all"])),
            ),
            ("var a=10, b=0, c=null; alert(a+b)", Some(serde_json::json!(["all"]))),
            (
                "var a=10, b=0, c=null; setTimeout(function() { var b=2; alert(a+b+c); }, 0);",
                Some(serde_json::json!(["all"])),
            ),
            (
                "var a=10, b=0, c=null; setTimeout(function() { var b=2; var c=2; alert(a+b+c); }, 0);",
                Some(serde_json::json!(["all"])),
            ),
            (
                "function f(){var a=[];return a.map(function(){});}",
                Some(serde_json::json!(["all"])),
            ),
            (
                "function f(){var a=[];return a.map(function g(){});}",
                Some(serde_json::json!(["all"])),
            ),
            (
                "function foo() {function foo(x) {\nreturn x; }; return function() {return foo; }; }",
                None,
            ),
            (
                "function f(){var x;function a(){x=42;}function b(){alert(x);}}",
                Some(serde_json::json!(["all"])),
            ),
            ("function f(a) {}; f();", Some(serde_json::json!(["all"]))),
            ("function a(x, y, z){ return y; }; a();", Some(serde_json::json!(["all"]))),
            ("var min = Math.min", Some(serde_json::json!(["all"]))),
            ("var min = {min: 1}", Some(serde_json::json!(["all"]))),
            ("Foo.bar = function(baz) { return 1; };", Some(serde_json::json!(["all"]))),
            ("var min = {min: 1}", Some(serde_json::json!([{ "vars": "all" }]))),
            (
                "function gg(baz, bar) { return baz; }; gg();",
                Some(serde_json::json!([{ "vars": "all" }])),
            ),
            (
                "(function(foo, baz, bar) { return baz; })();",
                Some(serde_json::json!([{ "vars": "all", "args": "after-used" }])),
            ),
            (
                "(function(foo, baz, bar) { return baz; })();",
                Some(serde_json::json!([{ "vars": "all", "args": "all" }])),
            ),
            (
                "(function z(foo) { var bar = 33; })();",
                Some(serde_json::json!([{ "vars": "all", "args": "all" }])),
            ),
            ("(function z(foo) { z(); })();", Some(serde_json::json!([{}]))),
            (
                "function f() { var a = 1; return function(){ f(a = 2); }; }",
                Some(serde_json::json!([{}])),
            ),
            ("import x from \"y\";", None),
            ("export function fn2({ x, y }) {\n console.log(x); \n};", None),
            ("export function fn2( x, y ) {\n console.log(x); \n};", None),
            ("/*exported max*/ var max = 1, min = {min: 1}", None),
            ("/*exported x*/ var { x, y } = z", None),
            (
                "var _a; var b;",
                Some(serde_json::json!([{ "vars": "all", "varsIgnorePattern": "^_" }])),
            ),
            // todo
            (
                "var a; function foo() { var _b; var c_; } foo();",
                Some(serde_json::json!([{ "vars": "local", "varsIgnorePattern": "^_" }])),
            ),
            (
                "function foo(a, _b) { } foo();",
                Some(serde_json::json!([{ "args": "all", "argsIgnorePattern": "^_" }])),
            ),
            (
                "function foo(a, _b, c) { return a; } foo();",
                Some(serde_json::json!([{ "args": "after-used", "argsIgnorePattern": "^_" }])),
            ),
            (
                "function foo(_a) { } foo();",
                Some(serde_json::json!([{ "args": "all", "argsIgnorePattern": "[iI]gnored" }])),
            ),
            (
                "var [ firstItemIgnored, secondItem ] = items;",
                Some(serde_json::json!([{ "vars": "all", "varsIgnorePattern": "[iI]gnored" }])),
            ),
            // /*
            //       {
            //        code: "const [ a, _b, c ] = items;\nconsole.log(a+c);",
            //        options: [{ destructuredArrayIgnorePattern: "^_" }],
            //        parserOptions: { ecmaVersion: 6 }
            //    },
            //    {
            //        code: "const [ [a, _b, c] ] = items;\nconsole.log(a+c);",
            //        options: [{ destructuredArrayIgnorePattern: "^_" }],
            //        parserOptions: { ecmaVersion: 6 }
            //    },
            //    {
            //        code: "const { x: [_a, foo] } = bar;\nconsole.log(foo);",
            //        options: [{ destructuredArrayIgnorePattern: "^_" }],
            //        parserOptions: { ecmaVersion: 6 }
            //    },
            //    {
            //        code: "function baz([_b, foo]) { foo; };\nbaz()",
            //        options: [{ destructuredArrayIgnorePattern: "^_" }],
            //        parserOptions: { ecmaVersion: 6 }
            //    },
            //    {
            //        code: "function baz({x: [_b, foo]}) {foo};\nbaz()",
            //        options: [{ destructuredArrayIgnorePattern: "^_" }],
            //        parserOptions: { ecmaVersion: 6 }
            //    },
            //    {
            //        code: "function baz([{x: [_b, foo]}]) {foo};\nbaz()",
            //        options: [{ destructuredArrayIgnorePattern: "^_" }],
            //        parserOptions: { ecmaVersion: 6 }
            //    },
            //    {
            //        code: `
            //        let _a, b;
            //        foo.forEach(item => {
            //            [_a, b] = item;
            //            doSomething(b);
            //        });
            //        `,
            //        options: [{ destructuredArrayIgnorePattern: "^_" }],
            //        parserOptions: { ecmaVersion: 6 }
            //    },
            // */
            ("
                // should report _x
                let _x, y;
                _x = 1;
                [_x, y] = foo;
                y;

                // should report _a
                let _a, b;
                [_a, b] = foo;
                _a = 1;
                b;",
                Some(serde_json::json!([{ "destructuredArrayIgnorePattern": "^_" }])) 
            ),
            ("
                // should report _x
                let _x, y;
                _x = 1;
                [_x, y] = foo;
                y;

                // should report _a
                let _a, b;
                _a = 1;
                ({_a, ...b } = foo);
                b;",
                Some(serde_json::json!([{ "destructuredArrayIgnorePattern": "^_", "ignoreRestSiblings": true }]))
            ),
            ("(function(obj) { var name; for ( name in obj ) { i(); return; } })({});", None),
            ("(function(obj) { var name; for ( name in obj ) { } })({});", None),
            ("(function(obj) { for ( var name in obj ) { } })({});", None),
            ("(function(iter) { var name; for ( name of iter ) { i(); return; } })({});", None),
            ("(function(iter) { var name; for ( name of iter ) { } })({});", None),
            ("(function(iter) { for ( var name of iter ) { } })({});", None),
            // ("\n/* global foobar, foo, bar */\nfoobar;", None),
            // ("\n/* global foobar,\n   foo,\n   bar\n */\nfoobar;", None),
            (
                "const data = { type: 'coords', x: 1, y: 2 };\nconst { type, ...coords } = data;\n console.log(coords);",
                None,
            ),
            (
                "const data = { type: 'coords', x: 2, y: 2 };\nconst { type, ...coords } = data;\n console.log(type)",
                Some(serde_json::json!([{ "ignoreRestSiblings": true }])),
            ),
            (
                "let type, coords;\n({ type, ...coords } = data);\n console.log(type)",
                Some(serde_json::json!([{ "ignoreRestSiblings": true }])),
            ),
            (
                "const data = { type: 'coords', x: 3, y: 2 };\nconst { type, ...coords } = data;\n console.log(type)",
                None,
            ),
            (
                "const data = { vars: ['x','y'], x: 1, y: 2 };\nconst { vars: [x], ...coords } = data;\n console.log(coords)",
                None,
            ),
            (
                "const data = { defaults: { x: 0 }, x: 1, y: 2 };\nconst { defaults: { x }, ...coords } = data;\n console.log(coords)",
                None,
            ),
            (
                "(({a, ...rest}) => {})",
                Some(serde_json::json!([{ "args": "all", "ignoreRestSiblings": true }])),
            ),
            // ("/* global a$fooz,$foo */\na$fooz;", None),
            // ("/* globals a$fooz, $ */\na$fooz;", None),
            // ("/*globals $foo*/", None),
            // ("/* global global*/", None),
            // ("/*global foo:true*/", None),
            // ("/*global 変数, 数*/\n変数;", None),
            // ("/*global 𠮷𩸽, 𠮷*/\n\\u{20BB7}\\u{29E3D};", None),
            ("export default function(a) {}", None),
            ("export default function(a, b) { console.log(a); }", None),
            ("export default (function(a) {});", None),
            ("export default (function(a, b) { console.log(a); });", None),
            ("export default (a) => {};", None),
            ("export default (a, b) => { console.log(a); };", None),
            ("try{}catch(err){};", Some(serde_json::json!([{ "caughtErrors": "all" }]))),
            (
                "try{}catch(err){};",
                Some(
                    serde_json::json!([{ "caughtErrors": "all", "caughtErrorsIgnorePattern": "^ignore" }]),
                ),
            ),
            (
                "try{}catch(ignoreErr){}try{}catch(err){};",
                Some(
                    serde_json::json!([{ "caughtErrors": "all", "caughtErrorsIgnorePattern": "^ignore" }]),
                ),
            ),
            (
                "try{}catch(error){}try{}catch(err){};",
                Some(
                    serde_json::json!([{ "caughtErrors": "all", "caughtErrorsIgnorePattern": "^ignore" }]),
                ),
            ),
            (
                "try{}catch(err){};",
                Some(serde_json::json!([{ "vars": "all", "args": "all", "caughtErrors": "all" }])),
            ),
            (
                "try{}catch(err){};",
                Some(serde_json::json!([
                    {
                        "vars": "all",
                        "args": "all",
                        "caughtErrors": "all",
                        "argsIgnorePattern": "^er"
                    }
                ])),
            ),
            ("var a = 0; a = a + 1;", None),
            ("var a = 0; a = a + a;", None),
            ("var a = 0; a += a + 1;", None),
            ("var a = 0; a++;", None),
            ("function foo(a) { a = a + 1 } foo();", None),
            ("function foo(a) { a += a + 1 } foo();", None),
            ("function foo(a) { a++ } foo();", None),
            ("var a = 3; a = a * 5 + 6;", None),
            ("var a = 2, b = 4; a = a * 2 + b;", None),
            // ("function foo(cb) { cb = function(a) { cb(1 + a); }; bar(not_cb); } foo();", None),
            // ("function foo(cb) { cb = function(a) { return cb(1 + a); }(); } foo();", None),
            // ("function foo(cb) { cb = (function(a) { cb(1 + a); }, cb); } foo();", None),
            // ("function foo(cb) { cb = (0, function(a) { cb(1 + a); }); } foo();", None),
            // https://github.com/eslint/eslint/issues/6646
            ("
                while (a) {
                    function foo(b) {
                        b = b + 1;
                    }
                    foo();
                }
            ", None),
            ("(function(a, b, c) {})", Some(serde_json::json!([{ "argsIgnorePattern": "c" }]))),
            (
                "(function(a, b, {c, d}) {})",
                Some(serde_json::json!([{ "argsIgnorePattern": "[cd]" }])),
            ),
            (
                "(function(a, b, {c, d}) {})",
                Some(serde_json::json!([{ "argsIgnorePattern": "c" }])),
            ),
            (
                "(function(a, b, {c, d}) {})",
                Some(serde_json::json!([{ "argsIgnorePattern": "d" }])),
            ),
            // ("/*global\rfoo*/", None),
            // ("(function ({ a }, b ) { return b; })();", None),
            // ("(function ({ a }, { b, c } ) { return b; })();", None),
            // https://github.com/eslint/eslint/issues/6646
            (
                "
                while(a) {
                    function foo(b) {
                        b = b + 1;
                    }
                    foo();
                }",
                None
            ),

            // https://github.com/eslint/eslint/issues/7124
            ("(function(a, b, c) {})", Some(serde_json::json!([{ "argsIgnorePattern": "c" }]))),
            ("let x = 0; x++, 0;", None),
            ("let x = 0; 0, x++;", None),
            // todo: handle sequence expressions correctly
            ("let x = 0; 0, (1, x++);", None),
            // ("let x = 0; foo = (x++, 0);", None),
            // ("let x = 0; foo = ((0, x++), 0);", None),
            ("let x = 0; x += 1, 0;", None),
            ("let x = 0; 0, x += 1;", None),
            ("let x = 0; 0, (1, x += 1);", None),
            // ("let x = 0; foo = (x += 1, 0);", None),
            // ("let x = 0; foo = ((0, x += 1), 0);", None),
            ("let z = 0; z = z + 1, z = 2;", None),
            ("let z = 0; z = z + 1, z = 2; z = 3;", None),
            ("let z = 0; z = z + 1, z = 2; z = z + 3;", None),
            ("let x = 0; 0, x = x+1;", None),
            ("let x = 0; x = x+1, 0;", None),
            // ("let x = 0; foo = ((0, x = x + 1), 0);", None),
            // ("let x = 0; foo = (x = x+1, 0);", None),
            ("let x = 0; 0, (1, x=x+1);", None),
            ("(function ({ a, b }, { c } ) { return b; })();", None),
            // todo
            // ("(function ([ a ], b ) { return b; })();", None),
            ("(function ([ a ], [ b, c ] ) { return b; })();", None),
            ("(function ([ a, b ], [ c ] ) { return b; })();", None),
            (
                "(function(_a) {})();",
                Some(serde_json::json!([{ "args": "all", "varsIgnorePattern": "^_" }])),
            ),
            (
                "(function(_a) {})();",
                Some(serde_json::json!([{ "args": "all", "caughtErrorsIgnorePattern": "^_" }])),
            ),
            ("var a = function() { a(); };", None),
            // ("var a = function(){ return function() { a(); } };", None),
            // ("const a = () => { a(); };", None),
            // ("const a = () => () => { a(); };", None),
            // /*
            //         {
            //         code: `let myArray = [1,2,3,4].filter((x) => x == 0);
            // myArray = myArray.filter((x) => x == 1);`,
            //         parserOptions: { ecmaVersion: 2015 },
            //         errors: [{ ...assignedError("myArray"), line: 2, column: 5 }]
            //     },
            //  */
            ("const a = 1; a += 1;", None),
            // ("var a = function() { a(); };", None),
            // ("var a = function(){ return function() { a(); } };", None),
            // ("const a = () => { a(); };", None),
            // ("const a = () => () => { a(); };", None),
            // ("let x = [];\nx = x.concat(x);", None),
            // /*
            //             {

            //             code: `let a = 'a';
            //             a = 10;
            //             function foo(){
            //                 a = 11;
            //                 a = () => {
            //                     a = 13
            //                 }
            //             }`,
            //             parserOptions: { ecmaVersion: 2020 },
            //             errors: [{ ...assignedError("a"), line: 2, column: 13 }, { ...definedError("foo"), line: 3, column: 22 }]
            //         },
            //         {
            //             code: `let foo;
            //             init();
            //             foo = foo + 2;
            //             function init() {
            //                 foo = 1;
            //             }`,
            //             parserOptions: { ecmaVersion: 2020 },
            //             errors: [{ ...assignedError("foo"), line: 3, column: 13 }]
            //         },
            //         {
            //             code: `function foo(n) {
            //                 if (n < 2) return 1;
            //                 return n * foo(n - 1);
            //             }`,
            //             parserOptions: { ecmaVersion: 2020 },
            //             errors: [{ ...definedError("foo"), line: 1, column: 10 }]
            //         },
            //         {
            //             code: `let c = 'c'
            // c = 10
            // function foo1() {
            //   c = 11
            //   c = () => {
            //     c = 13
            //   }
            // }

            // c = foo1`,
            //             parserOptions: { ecmaVersion: 2020 },
            //             errors: [{ ...assignedError("c"), line: 10, column: 1 }]
            //         }
            //      */
        ];

    Tester::new(NoUnusedVars::NAME, pass, fail).test_and_snapshot();
}
