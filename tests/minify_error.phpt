--TEST--
Minification error
--INI--
extension=target/debug/libphp_oxc_minifier.so
--FILE--
<?php
$minifier = new JavascriptMinifier();
try {
    $minifier->minify("var a = 1;\nvar b = 2; // comment\nvar c = ;");
} catch ( \MinificationException $e ) {
    print implode("\n", $e->getErrors());
}
?>
--EXPECT--
x Unexpected token
   ,-[3:9]
 2 | var b = 2; // comment
 3 | var c = ;
   :         ^
   `----
