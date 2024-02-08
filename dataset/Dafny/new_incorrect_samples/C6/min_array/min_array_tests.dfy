method minArray(a: array<int>) returns (r:int)
{
  r:=1000;
  var i:=0;
  while i<a.Length
    invariant 0 <= i <= a.Length
    invariant forall x :: 0 <= x < i ==> r <= a[x]
  {
    if r>a[i]{
      r:=a[i];
    }
    i:=i+1;
  }

}
method TestMinArray(){
  var a1 := new int[5];
  a1[0] := 1; a1[1] := 2; a1[2] := 3; a1[3] := 4; a1[4] := 5;
  var test1 := minArray(a1);
  print("Test 1: minArray([1,2,3,4,5]) = ", test1, "\n");

  var a2 := new int[5];
  a2[0] := -1; a2[1] := -2; a2[2] := -3; a2[3] := -4; a2[4] := -5;
  var test2 := minArray(a2);
  print("Test 2: minArray([-1,-2,-3,-4,-5]) = ", test2, "\n");

  var a3 := new int[3];
  a3[0] := 0; a3[1] := 0; a3[2] := 0;
  var test3 := minArray(a3);
  print("Test 3: minArray([0,0,0]) = ", test3, "\n");

  var a4 := new int[2];
  a4[0] := 5; a4[1] := 10;
  var test4 := minArray(a4);
  print("Test 4: minArray([5,10]) = ", test4, "\n");

  var a5 := new int[1];
  a5[0] := 99;
  var test5 := minArray(a5);
  print("Test 5: minArray([99]) = ", test5, "\n");
}

method Main(){
  TestMinArray();
}
