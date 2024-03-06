method maxArray(a: array<int>) returns (m: int)
  requires a.Length >= 1
  ensures forall k :: 0 <= k < a.Length ==> m >= a[k]
  ensures exists k :: 0 <= k < a.Length && m == a[k]
{
  m := a[0];
  var index := 1;
  while (index < a.Length)
    invariant 0 <= index <= a.Length
    invariant forall k :: 0 <= k < index ==> m >= a[k]
    invariant exists k :: 0 <= k < index && m == a[k]
    decreases a.Length - index
  {
    m := if m>a[index] then  m else a[index];
    index := index + 1;
  }
}
method TestMethod(){
  var a1 := new int[5];
  a1[0] := 1; a1[1] := 2; a1[2] := 3; a1[3] := 4; a1[4] := 5;
  var test1 := maxArray(a1);
  print("Test 1: maxArray([1,2,3,4,5]) = ", test1, "\n");

  var a2 := new int[5];
  a2[0] := -1; a2[1] := -2; a2[2] := -3; a2[3] := -4; a2[4] := -5;
  var test2 := maxArray(a2);
  print("Test 2: maxArray([-1,-2,-3,-4,-5]) = ", test2, "\n");

  var a3 := new int[3];
  a3[0] := 0; a3[1] := 0; a3[2] := 0;
  var test3 := maxArray(a3);
  print("Test 3: maxArray([0,0,0]) = ", test3, "\n");

  var a4 := new int[2];
  a4[0] := 5; a4[1] := 10;
  var test4 := maxArray(a4);
  print("Test 4: maxArray([5,10]) = ", test4, "\n");

  var a5 := new int[1];
  a5[0] := 99;
  var test5 := maxArray(a5);
  print("Test 5: maxArray([99]) = ", test5, "\n");
}

method Main(){
  TestMethod();
}
