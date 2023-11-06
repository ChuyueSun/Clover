method BinarySearch(a: array<int>, key: int) returns (n: int)
  requires forall i,j :: 0<=i<j<a.Length ==> a[i]<=a[j]
  ensures 0<= n <=a.Length
  ensures forall i :: 0<= i < n ==> a[i]<key
  ensures forall i :: n<= i < a.Length ==> a[i]>=key
{
  var lo, hi := 0, a.Length;
  while lo<hi
    invariant 0<= lo <= hi <= a.Length
    invariant forall i :: 0<=i<lo ==> a[i] < key
    invariant forall i :: hi<=i<a.Length ==> a[i] >= key
  {
    var mid := (lo + hi) / 2;
    if a[mid] < key {
      lo := mid + 1;
    } else {
      hi := mid;
    }
  }
  n:=lo;
}

method TestBinarySearch(){
  var a := new int[][1, 2, 3, 4, 5];
  var test1 := BinarySearch(a, 3);
  print("Test 1: BinarySearch(new int[]{1, 2, 3, 4, 5}, 3) = ", test1, "\n");
  a := new int[][10, 20, 30, 40, 50];
  var test2 := BinarySearch(a, 20);
  print("Test 2: BinarySearch(new int[]{10, 20, 30, 40, 50}, 20) = ", test2, "\n");
  a := new int[][2, 4, 6, 8, 10];
  var test3 := BinarySearch(a, 1);
  print("Test 3: BinarySearch(new int[]{2, 4, 6, 8, 10}, 1) = ", test3, "\n");
  a := new int[][5, 5, 5, 5, 5];
  var test4 := BinarySearch(a, 5);
  print("Test 4: BinarySearch(new int[]{5, 5, 5, 5, 5}, 5) = ", test4, "\n");
  a := new int[][0];
  var test5 := BinarySearch(a, 0);
  print("Test 5: BinarySearch(new int[]{0}, 0) = ", test5, "\n");
}

method Main(){
  TestBinarySearch();
}
