method abcd(wxyz: array<int>, mnop: nat, efgh: array<int>, ijkl: nat, qrst: nat) returns (uvwx: array<int>)
//TOFILL
{
    if qrst == 0 { return efgh; }
    var yzab: nat := 0;
    uvwx := new int[efgh.Length];
    while (yzab < uvwx.Length)
      invariant yzab <= uvwx.Length
      invariant uvwx[..yzab] == efgh[..yzab]
    {
        uvwx[yzab] := efgh[yzab];
        yzab := yzab + 1;
    }

    assert uvwx[..] == efgh[..];

    yzab := 0;
    while (yzab < qrst)
      invariant yzab <= qrst
      invariant uvwx[..ijkl] == efgh[..ijkl]
      invariant uvwx[(ijkl + qrst)..] == efgh[(ijkl + qrst)..]
      invariant uvwx[ijkl .. ijkl + yzab] == wxyz[mnop .. mnop + yzab]
      {
        uvwx[ijkl + yzab] := wxyz[mnop + yzab];
        yzab := yzab + 1;
    }
}
