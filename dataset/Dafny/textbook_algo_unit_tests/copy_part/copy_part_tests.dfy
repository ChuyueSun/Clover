method copy( src: array<int>, sStart: nat, dest: array<int>, dStart: nat, len: nat) returns (r: array<int>)
   requires src.Length >= sStart + len
   requires dest.Length >= dStart + len
   ensures r.Length == dest.Length
   ensures r[..dStart] == dest[..dStart]
   ensures r[dStart + len..] == dest[dStart + len..]
   ensures forall k:int :: dStart<= k < len+dStart ==> r[k] == src[sStart + k-dStart]

{
    if len == 0 { return dest; }
    var i: nat := 0;
    r := new int[dest.Length];
    while (i < r.Length)
      invariant i <= r.Length
      invariant r[..i] == dest[..i]
    {
        r[i] := dest[i];
        i := i + 1;
    }

    i := 0;
    while (i < len)
      invariant i <= len
      invariant r[..dStart] == dest[..dStart]
      invariant r[(dStart + len)..] == dest[(dStart + len)..]
      invariant r[dStart .. dStart + i] == src[sStart .. sStart + i]
      {
        r[dStart + i] := src[sStart + i];
        i := i + 1;
    }
}

method print_array(a:array<int>)
{
  for i:= 0 to a.Length{
    print(a[i]);
  }
  print(";");
}

method TestCopy1() {
    var src := new int[4];
    src[0], src[1], src[2], src[3] := 1, 2, 3, 4;
    
    var dest := new int[4];
    dest[0], dest[1], dest[2], dest[3] := 5, 6, 7, 8;

    var expected := new int[4];
    expected[0], expected[1], expected[2], expected[3] := 1, 2, 3, 8;
    
    var result := copy(src, 0, dest, 0, 3);
    print_array(result);
}

method TestCopy2() {
    var src := new int[3];
    src[0], src[1], src[2] := 9, 10, 11;
    
    var dest := new int[4];
    dest[0], dest[1], dest[2], dest[3] := 1, 2, 3, 4;

    var expected := new int[4];
    expected[0], expected[1], expected[2], expected[3] := 1, 10, 11, 4;
    
    var result := copy(src, 1, dest, 1, 2);
    print_array(result);
}

method TestCopy3() {
    var src := new int[4];
    src[0], src[1], src[2], src[3] := 12, 13, 14, 15;
    
    var dest := new int[4];
    dest[0], dest[1], dest[2], dest[3] := 4, 3, 2, 1;

    var expected := new int[4];
    expected[0], expected[1], expected[2], expected[3] := 4, 13, 14, 1;
    
    var result := copy(src, 1, dest, 1, 2);
    print_array(result);
}

method TestCopy4() {
    var src := new int[1];
    src[0] := 16;
    
    var dest := new int[4];
    dest[0], dest[1], dest[2], dest[3] := 1, 2, 3, 4;

    var expected := new int[4];
    expected[0], expected[1], expected[2], expected[3] := 1, 16, 3, 4;
    
    var result := copy(src, 0, dest, 1, 1);
    print_array(result);
}

method TestCopy5() {
    var src := new int[3];
    src[0], src[1], src[2] := 17, 18, 19;
    
    var dest := new int[4];
    dest[0], dest[1], dest[2], dest[3] := 4, 3, 2, 1;

    var expected := new int[4];
    expected[0], expected[1], expected[2], expected[3] := 17, 18, 19, 1;
    
    var result := copy(src, 0, dest, 0, 3);
    print_array(result);
}

// Execute all the test cases.
method Main() {
    TestCopy1();
    TestCopy2();
    TestCopy3();
    TestCopy4();
    TestCopy5();
}

