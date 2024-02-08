method LinearSearch(a: array<int>, e: int) returns (n:int)
  ensures 0<=n<=a.Length
  ensures n==a.Length || a[n]==e
  ensures n==a.Length ==> forall i::0<=i<a.Length ==> e!=a[i]
{
  n :=0;
  while n!=a.Length
    invariant 0<=n<=a.Length
    invariant forall i::0<=i<n ==> e!=a[i]
  {
    if e==a[n]{
      return;
    }
    n:=n+1;
  }
}

method TestLinearSearch(){
  var a:=new int[5] [1, 2, 3, 4, 5];
  var test := LinearSearch(a, 3);
  print("Test 1: LinearSearch(...) = ", test, "\n");

  a:=new int[] [10, 15, 20, 25, 30, 35, 40];
  test := LinearSearch(a, 30);
  print("Test 2: LinearSearch(...) = ", test, "\n");
  a:=new int[] [0];
  test := LinearSearch(a, 0);
  print("Test 3: LinearSearch(...) = ", test, "\n");

  a:=new int[] [-10, -20, -30];
  test := LinearSearch(a, -20);
  print("Test 4: LinearSearch(...) = ", test, "\n");
  a:=new int[] [321, 123, 456, 654];
  test := LinearSearch(a, 5);
  print("Test 5: LinearSearch(...) = ", test, "\n");
}

method Main(){
  TestLinearSearch();
}
