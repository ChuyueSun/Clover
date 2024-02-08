method FindEvenNumbers (arr: array<int>) returns (evenNumbers: array<int>)
  ensures forall x {:trigger (x%2) }:: x in arr[..] &&  (x%2==1)==> x in evenNumbers[..];
  ensures forall x :: x !in arr[..] ==> x !in evenNumbers[..]
  ensures forall k, l :: 0 <= k < l < evenNumbers.Length ==>
                           exists n, m :: 0 <= n < m < arr.Length && evenNumbers[k] == arr[n] && evenNumbers[l] == arr[m]
  ensures forall k :: 0 <= k < evenNumbers.Length ==> evenNumbers[k] % 2 == 1

{
  var evenList: seq<int> := [];
  ghost var indices: seq<int> := [];

  for i := 0 to arr.Length
    invariant 0 <= i <= arr.Length
    invariant 0 <= |evenList| <= i
    invariant forall x  {:trigger (x%2) }:: (x in arr[..i] && (x%2==1) )==> x in evenList[..]
    invariant forall k :: 0 <= k < |evenList| ==> evenList[k] % 2 == 1
    invariant forall x :: x !in arr[..i] ==> x !in evenList
    invariant |evenList| == |indices|
    invariant forall k :: 0 <= k < |indices| ==> indices[k] < i
    invariant forall k, l :: 0 <= k < l < |indices| ==> indices[k] < indices[l]
    invariant forall k :: 0 <= k < |evenList| ==> 0 <= indices[k] < i <= arr.Length && arr[indices[k]] == evenList[k]
  {
    if arr[i]%2==1
    {
      evenList := evenList + [arr[i]];
      indices := indices + [i];
    }
  }

  evenNumbers := new int[|evenList|](i requires 0 <= i < |evenList| => evenList[i]);
  assert evenList == evenNumbers[..];
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
  var test1 := FindEvenNumbers(a);
  print_array(test1);
  // print("Test 1: FindEvenNumbers([1,2,3,4,5,6,7,8,9]) = ", test1, "\n");
  a := new int[][22, 18, 36, 72, 101];
  var test2 := FindEvenNumbers(a);
  print_array(test2);
  // print("Test 2: FindEvenNumbers([22, 18, 36, 72, 101]) = ", test2, "\n");
  a := new int[][-1,0,-1,2,4];
  var test3 := FindEvenNumbers(a);
  print_array(test3);
  // print("Test 3: FindEvenNumbers([-1,0,-1,2,4]) = ", test3, "\n");
  a := new int[][];
  var test4 := FindEvenNumbers(a);
  print_array(test4);
  // print("Test 4: FindEvenNumbers([]) = ", test4, "\n");
  a := new int[][11,23,35,47,59,71];
  var test5 := FindEvenNumbers(a);
  print_array(test5);
  // print("Test 5: FindEvenNumbers([11,23,35,47,59,71]) = ", test5, "\n");
}

method Main() {
  TestFindEvenNumbers();
}
