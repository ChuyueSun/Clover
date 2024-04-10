method LinearSearch(a: array<int>, e: int) returns (n:int)
{
  n :=0;
  while n!=a.Length
    invariant 0<=n<=a.Length
    invariant forall i::0<=i<n ==> e!=a[i]
  {
    if e==a[n]{
      break;
    }
    n:=n+1;
  }
  n := n - 1;

}
method TestLinearSearch(){
  var a:=new int[5] [1, 2, 3, 4, 5];
  assert a[2]==3;
  var test := LinearSearch(a, 3);
  print("Test 1: LinearSearch(...) = ", test, "\n");

  a:=new int[] [10, 15, 20, 25, 30, 35, 40];
  assert a[4]==30;
  test := LinearSearch(a, 30);
  print("Test 2: LinearSearch(...) = ", test, "\n");
  a:=new int[] [0];
  assert a[0]==0;
  test := LinearSearch(a, 0);
  print("Test 3: LinearSearch(...) = ", test, "\n");

  a:=new int[] [-10, -20, -30];
  assert a[1]==-20;
  test := LinearSearch(a, -20);
  print("Test 4: LinearSearch(...) = ", test, "\n");
  a:=new int[] [321, 123, 456, 654];
  assert a[1]==123;
  test := LinearSearch(a, 123);
  print("Test 5: LinearSearch(...) = ", test, "\n");
}

method Main()
{
  TestLinearSearch();
}
