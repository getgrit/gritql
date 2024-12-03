class A : B, C
//    ^ type
//        ^ type
//           ^ type
{
    public void M()
    {
        int a;
        // <- type.builtin
        var a;
        // <- keyword

        int? a;
        // <- type.builtin
        // ^ operator
        A? a;
        // <- type
         // <- operator

        int* a;
        // <- type.builtin
        // ^ operator
        A* a;
        // <- type
         // <- operator

        ref A* a;
        // <- keyword
        //  ^ type
        //   ^ operator

        var a = x is int;
        //           ^ type.builtin
        var a = x is A;
        //           ^

        var a = x as int;
        //           ^ type.builtin
        var a = x as A;
        //           ^ type

        var a = (int)x;
        //       ^ type.builtin
        var a = (A)x;
        //       ^ type

        A<int, A> a = new A<int, A>();
        // <- type
        //^ type.builtin
        //     ^ type
        //                ^ type
        //                  ^ type.builtin
        //                       ^ type
    }
}

record A(int a, B b) : B(), I;
//     ^ type
//       ^ type.builtin
//              ^ type
//                     ^ type
//                          ^ type

record A : B, I;
//     ^ type
//         ^ type
//            ^ type
