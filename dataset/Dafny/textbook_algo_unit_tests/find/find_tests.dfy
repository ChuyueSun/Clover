method Find(a: array<int>, key: int) returns (index: int)
  ensures -1<=index<a.Length
  ensures index!=-1 ==> a[index]==key
  ensures index == -1 ==> (forall i::0 <= i < a.Length ==> a[i] != key)
{
  index := 0;
  while index < a.Length
    invariant 0<=index<=a.Length
    invariant (forall i::0 <= i < index==>a[i] != key)
  {
    if a[index] == key { return; }
    index := index + 1;
  }
  if index >= a.Length  {
    index := -1;
  }
}

method TestMethod(){
  var a1:= new int[5] [1, 2, 3, 4, 5];
  var test1 := Find(a1, 3);
  print("Test 1: Find(...) = ", test1, "\n");

  var a2:= new int[5] [5, 4, 3, 2, 1];
  var test2 := Find(a2, 1);
  print("Test 2: Find(...) = ", test2, "\n");

  var a3:= new int[5] [-2, -4, -6, -8, -10];
  var test3 := Find(a3, -8);
  print("Test 3: Find(...) = ", test3, "\n");

  var a4:= new int[5] [0, 10, 20, 30, 40];
  var test4 := Find(a4, 60);
  print("Test 4: Find(...) = ", test4, "\n");

  var a5:= new int[0] [];
  var test5 := Find(a5, 10);
  print("Test 5: Find(...) = ", test5, "\n");
}

method Main(){
  TestMethod();
}
