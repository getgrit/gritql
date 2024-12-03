class A
{
    public void M()
    {
        foreach (int i in new[] { 1 })
        //           ^ variable
        {
            int j = i;
            //  ^ variable
        }

        var x = from a in sourceA
        //           ^ variable
        //                ^ variable
                join b in sourceB on a.FK equals b.PK
        //           ^ variable
        //                ^ variable
                group a by a.X into g
        //            ^ variable
        //                          ^ variable
                orderby g ascending
        //              ^ variable
                select new { A.A, B.B };
    }
}
