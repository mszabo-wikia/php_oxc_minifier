--TEST--
Basic minification
--INI--
extension=target/debug/libphp_oxc_minifier.so
--FILE--
<?php
$minifier = new JavascriptMinifier();
print $minifier->minify("var a = 1;\nvar b = 2; // comment");
?>
--EXPECT--
var a = 1, b = 2;
