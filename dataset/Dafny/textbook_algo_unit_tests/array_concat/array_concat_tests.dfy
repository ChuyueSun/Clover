method concat(a:array<int>, b:array<int>) returns (c:array<int>)
  ensures c.Length==b.Length+a.Length
  ensures forall k :: 0 <= k < a.Length ==> c[k] == a[k]
  ensures forall k :: 0 <= k < b.Length ==> c[k+a.Length] == b[k]
  ensures a[..]+b[..]==c[..]
{
  c := new int[a.Length+b.Length];
  var i:= 0;
  while (i < c.Length)
    invariant 0 <= i <= c.Length
    invariant if i<a.Length then c[..i]==a[..i] else c[..i]==a[..]+b[..(i-a.Length)]
  {
    c[i] := if i<a.Length then a[i] else b[i-a.Length];
    i:=i+1;
  }
}

method print_array(a:array<int>)
{
  for i:= 0 to a.Length{
    print(a[i]);
  }
  print(";");
}

method TestMethod(){
  var a1 := new int[3][1,2,3];
  var b1 := new int[3][4,5,6];
  var test1 := concat(a1, b1);
  // print("Test 1: concat([1,2,3], [4,5,6]) = ");
  print_array(test1);

  var a2 := new int[3][-1,-2,-3];
  var b2 := new int[3][-4,-5,-6];
  var test2 := concat(a2, b2);
  print_array(test2);
  // print("Test 2: concat([-1,-2,-3], [-4,-5,-6]) = ", test2, "\n");

  var a3 := new int[0][];
  var b3 := new int[3][7,8,9];
  var test3 := concat(a3, b3);
  print_array(test3);
  // print("Test 3: concat([], [7,8,9]) = ", test3, "\n");

  var a4 := new int[2][10,11];
  var b4 := new int[0][];
  var test4 := concat(a4, b4);
  print_array(test4);
  // print("Test 4: concat([10,11], []) = ", test4, "\n");

  var a5 := new int[0][];
  var b5 := new int[0][];
  var test5 := concat(a5, b5);
  print_array(test5);
  // print("Test 5: concat([], []) = ", test5, "\n");
}

method Main(){
  TestMethod();
}
