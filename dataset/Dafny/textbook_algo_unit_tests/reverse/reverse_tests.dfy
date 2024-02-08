method reverse(a: array<int>)
  modifies a
  ensures forall i :: 0 <= i < a.Length ==> a[i] == old(a[a.Length - 1 - i])
{
  var i       := 0;
  var aLength := a.Length;

  var b := new int[a.Length];

  while i < aLength
    invariant 0 <= i && i <= aLength
    invariant forall k :: 0 <= k < i ==> b[k] == a[aLength - 1 - k]
    invariant forall k :: 0 <= k < aLength ==> a[k] == old(a[k])
  {
    b[i] := a[aLength - 1 - i];
    i := i + 1;
  }

  i := 0;
  while i < aLength
    invariant 0 <= i && i <= aLength
    invariant forall k :: 0 <= k < aLength ==> b[k] == old(a[aLength - 1 - k])
    invariant forall k :: 0 <= k < i ==> a[k] == b[k]
  {
      a[i] := b[i];
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

method TestFindEvenNumbers() {
  var a := new int[][1,2,3,4,5,6,7,8,9];
  reverse(a);
  print_array(a);
  // print("Test 1: FindEvenNumbers([1,2,3,4,5,6,7,8,9]) = ", test1, "\n");
  a := new int[][22, 18, 36, 72, 101];
  reverse(a);
  print_array(a);
  // print("Test 2: FindEvenNumbers([22, 18, 36, 72, 101]) = ", test2, "\n");
  a := new int[][-1,0,-1,2,4];
  reverse(a);
  print_array(a);
  // print("Test 3: FindEvenNumbers([-1,0,-1,2,4]) = ", test3, "\n");
  a := new int[][];
  reverse(a);
  print_array(a);
  // print("Test 4: FindEvenNumbers([]) = ", test4, "\n");
  a := new int[][11,23,35,47,59,71];
  reverse(a);
  print_array(a);
  // print("Test 5: FindEvenNumbers([11,23,35,47,59,71]) = ", test5, "\n");
}

method Main() {
  TestFindEvenNumbers();
}
