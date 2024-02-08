method only_once<T(==)>(a: array<T>, key: T) returns (b:bool)
  ensures (multiset(a[..])[key] ==1 ) <==> b
{
  var i := 0;
  b := false;
  var keyCount := 0;
  while i < a.Length
    invariant 0 <= i <= a.Length
    invariant multiset(a[..i])[key] == keyCount
    invariant b <==> (keyCount == 1)
  {
    if (a[i] == key)
    {
      keyCount := keyCount + 1;
    }
    if (keyCount == 1)
    { b := true; }
    else
    { b := false; }
    i := i + 1;
  }

  assert a[..a.Length] == a[..];
}


method TestMethod(){

  var arr1 := new int[5]; arr1[0] := 1; arr1[1] := 2; arr1[2] := 3; arr1[3] := 4; arr1[4] := 5;
  var test1 := only_once(arr1, 3);
  print("Test 1: only_once(arr1, 3) = ", test1, "\n");

  var arr2 := new int[5]; arr2[0] := 1; arr2[1] := 1; arr2[2] := 1; arr2[3] := 1; arr2[4] := 1;
  var test2 := only_once(arr2, 1);
  print("Test 2: only_once(arr2, 1) = ", test2, "\n");

  var arr3 := new int[6]; arr3[0] := 2; arr3[1] := 2; arr3[2] := 3; arr3[3] := 3; arr3[4] := 5; arr3[5] := 1;
  var test3 := only_once(arr3, 5);
  print("Test 3: only_once(arr3, 5) = ", test3, "\n");

  var arr4 := new int[4]; arr4[0] := 1; arr4[1] := 2; arr4[2] := 3; arr4[3] := 4;
  var test4 := only_once(arr4, 2);
  print("Test 4: only_once(arr4, 2) = ", test4, "\n");

  var arr5 := new int[1]; arr5[0] := 10;
  var test5 := only_once(arr5, 10);
  print("Test 5: only_once(arr5, 10) = ", test5, "\n");
}

method Main(){
  TestMethod();
}
