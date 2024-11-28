using Namespace;

class C
{
    void M()
    {
        // unary
        a = +a;
        //  ^ operator
        a = -a;
        //  ^ operator
        a = !a;
        //  ^ operator
        a = ~a;
        //  ^ operator
        a = ++a;
        //  ^ operator
        a = --a;
        //  ^ operator
        a = a++;
        //   ^ operator
        a = a--;
        //   ^ operator
        a = a!;
        //   ^ operator
        a = a++;
        //   ^ operator
        a = a--;
        //   ^ operator

        // binary
        a = a + a;
        //    ^ operator
        a = a - a;
        //    ^ operator
        a = a * a;
        //    ^ operator
        a = a / a;
        //    ^ operator
        a = a % a;
        //    ^ operator
        a = a & a;
        //    ^ operator
        a = a | a;
        //    ^ operator
        a = a ^ a;
        //    ^ operator
        a = a >> a;
        //    ^ operator
        a = a << a;
        //    ^ operator
        a = a >>> a;
        //    ^ operator

        a = a == b;
        //    ^ operator
        a = a != b;
        //    ^ operator
        a = a < b;
        //    ^ operator
        a = a <= b;
        //    ^ operator
        a = a > b;
        //    ^ operator
        a = a >= b;
        //    ^ operator

        // assignment binary
        a += a;
        //^ operator
        a -= a;
        //^ operator
        a *= a;
        //^ operator
        a /= a;
        //^ operator
        a %= a;
        //^ operator
        a <<= a;
        //^ operator
        a >>= a;
        //^ operator
        a >>>= a;
        //^ operator

        // ternary
        string y = x ? "foo" : "bar";
        //           ^ operator
        //                   ^ operator

        // misc
        var l = (int i) => i;
        //              ^ operator
    }
}
