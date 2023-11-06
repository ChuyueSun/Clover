method TestArrayElements(a:array<int>, j: nat, k: nat)
  requires 0<=j < a.Length
  modifies a
  ensures a[j] == 60
{
  a[j] := 60;
}

method TestMethod(){
  var arr1 := new int[5];
  TestArrayElements(arr1, 2, 0);
  print("Test 1: TestArrayElements(...) = ", arr1[2], "\n");

  var arr2:= new int[4];
  arr2[1] := 40;
  TestArrayElements(arr2, 1, 9);
  print("Test 2: TestArrayElements(...) = ", arr2[1], "\n");

  var arr3 := new int[6];
  arr3[3] := 30;
  TestArrayElements(arr3, 3, 7);
  print("Test 3: TestArrayElements(...) = ", arr3[3], "\n");

  var arr4 := new int[2];
  TestArrayElements(arr4, 0, 8);
  print("Test 4: TestArrayElements(...) = ", arr4[0], "\n");

  var arr5 := new int[3];
  arr5[2] := 70;
  TestArrayElements(arr5, 2, 8);
  print("Test 5: TestArrayElements(...) = ", arr5[2], "\n");
}

method Main(){
  TestMethod();
}
