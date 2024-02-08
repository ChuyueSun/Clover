method LinearSearch3<T>(a: array<T>, P: T -> bool) returns (n: int)
  requires exists i :: 0 <= i < a.Length && P(a[i])
  ensures 0 <= n < a.Length && P(a[n])
  ensures forall k :: 0 <= k < n ==> !P(a[k])
{
  n := 0;
  while true
    invariant 0 <= n < a.Length
    invariant exists i :: n <= i < a.Length && P(a[i])
    invariant  forall k :: 0 <= k < n ==> !P(a[k])
    decreases a.Length - n
  {
    if P(a[n]) {
      return;
    }
    n := n + 1;
  }
}

method TestMethod(){
  var arr1 := new int[3] [1, 2, 3];
  var P1 := x => x == 2;
  assert P1(arr1[1]);
  var test1 := LinearSearch3(arr1, P1);
  print("Test 1: LinearSearch3(arr1, P1) = ", test1, "\n");

  var arr2 := new int[5] [6, 7, 8, 9, 10];
  var P2 := x => x > 9;
  assert P2(arr2[4]);
  var test2 := LinearSearch3(arr2, P2);
  print("Test 2: LinearSearch3(arr2, P2) = ", test2, "\n");

  var arr3 := new string[4] ["apple", "banana", "cherry", "date"];
  var P3 := x => x == "cherry";
  assert P3(arr3[2]);
  var test3 := LinearSearch3(arr3, P3);
  print("Test 3: LinearSearch3(arr3, P3) = ", test3, "\n");

  var arr4 := new bool[2] [true, false];
  var P4 := x => x == false;
  assert P4(arr4[1]);
  var test4 := LinearSearch3(arr4, P4);
  print("Test 4: LinearSearch3(arr4, P4) = ", test4, "\n");

  var arr5 := new int[5] [-1, -2, -3, -4, -5];
  var P5 := x => x%2==0;
  assert P5(arr5[1]);
  var test5 := LinearSearch3(arr5, P5);
  print("Test 5: LinearSearch3(arr5, P5) = ", test5, "\n");
}

method Main(){
  TestMethod();
}
