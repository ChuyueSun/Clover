method onlineMax(a: array<int>, x: int) returns (ghost m:int, p:int)
{
  p:= 0;
  var best := a[0];
  var i:=1;
  while i<x
    invariant 0<=i<=x
    invariant forall j::0<=j<i==> a[j]<=best
    invariant exists j::0<=j<i && a[j]==best
  {
    if a[i]>best{
      best:=a[i];
    }
    i:=i+1;
  }
  m:=best;
  i:=x;
  while i<a.Length
    invariant x<=i<=a.Length
    invariant forall j::x<=j<i ==> a[j]<=m
  {
    if a[i]>best{
      p:=i;
      return;
    }
    i:=i+1;
  }
  p:=a.Length;

}
method TestMethod(){
  var arr1 := new int[5];
  arr1[0] := 1;
  arr1[1] := 2;
  arr1[2] := 3;
  arr1[3] := 4;
  arr1[4] := 5;
  var g1, test1 := onlineMax(arr1, 3);
  print("Test 1: onlineMax(...) = ", test1, "\n");

  var arr2 := new int[5];
  arr2[0] := 5;
  arr2[1] := 4;
  arr2[2] := 3;
  arr2[3] := 2;
  arr2[4] := 1;
  var g2, test2 := onlineMax(arr2, 2);
  print("Test 2: onlineMax(...) = ", test2, "\n");

  var arr3 := new int[5];
  arr3[0] := 2;
  arr3[1] := 2;
  arr3[2] := 2;
  arr3[3] := 2;
  arr3[4] := 2;
  var g3, test3 := onlineMax(arr3, 4);
  print("Test 3: onlineMax(...) = ", test3, "\n");

  var arr4 := new int[5];
  arr4[0] := 1;
  arr4[1] := 1;
  arr4[2] := 2;
  arr4[3] := 2;
  arr4[4] := 1;
  var g4, test4 := onlineMax(arr4, 1);
  print("Test 4: onlineMax(...) = ", test4, "\n");

  var arr5 := new int[5];
  arr5[0] := 3;
  arr5[1] := 1;
  arr5[2] := 4;
  arr5[3] := 1;
  arr5[4] := 5;
  var g5, test5 := onlineMax(arr5, 1);
  print("Test 5: onlineMax(...) = ", test5, "\n");
}

method Main(){
  TestMethod();
}
