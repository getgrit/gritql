<?php
// <- tag

if ($a) {} elseif ($b) {} else {}
// <- keyword
//         ^ keyword
//                        ^ keyword

for ($i = 0; $i < 1; $i++) {}
// <- keyword

while ($b) {}
// <- keyword

WHILE ($b) {}
// <- keyword

do { } while ($c);
// <- keyword
//     ^ keyword

foreach ($foos as $foo) {}
// <- keyword
//             ^ keyword

try {} catch (Exception $e) {}
// <- keyword
//     ^ keyword

function a() {}
// <- keyword

class A {}
// <- keyword

throw new Exception("oh");
// <- keyword
//    ^ keyword

function b(
  int $a,
  // <- type.builtin

  string $b,
  // <- type.builtin

  Person $e
  // ^ type
): Dog {}
// ^ type

interface T {}
// ^ keyword

trait T {}
// ^ keyword
