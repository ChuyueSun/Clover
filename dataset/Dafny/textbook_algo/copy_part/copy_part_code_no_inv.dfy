method copy( src: array<int>, sStart: nat, dest: array<int>, dStart: nat, len: nat) returns (r: array<int>)
{
    if len == 0 { return dest; }
    var i: nat := 0;
    r := new int[dest.Length];
    while (i < r.Length)
    {
        r[i] := dest[i];
        i := i + 1;
    }

    i := 0;
    while (i < len)
      {
        r[dStart + i] := src[sStart + i];
        i := i + 1;
    }
}