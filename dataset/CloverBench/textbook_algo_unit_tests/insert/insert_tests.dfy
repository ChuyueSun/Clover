method insert(line:array<char>, l:int, nl:array<char>, p:int, at:int)
  requires 0 <= l+p <= line.Length
  requires 0 <= p <= nl.Length
  requires 0 <= at <= l
  modifies line
  ensures forall i :: (0<=i<p) ==> line[at+i] == nl[i]
  ensures forall i :: (0<=i<at) ==> line[i] == old(line[i])
  ensures forall i :: (at+p<=i<l+p) ==> line[i] == old(line[i-p])
{
  ghost var initialLine := line[..];

  var i:int := l;
  while(i>at)
    invariant line[0..i] == initialLine[0..i]
    invariant line[i+p..l+p] == initialLine[i..l]
    invariant at<=i<=l
  {
    i := i - 1;
    line[i+p] := line[i];
  }

  assert line[0..at] == initialLine[0..at];
  assert line[at+p..l+p] == initialLine[at..l];

  i := 0;
  while(i<p)
    invariant 0<=i<=p
    invariant line[0..at] == initialLine[0..at]
    invariant line[at..at+i] == nl[0..i]
    invariant line[at+p..l+p] == initialLine[at..l]
  {
    line[at + i] := nl[i];
    i := i + 1;
  }

  assert line[0..at] == initialLine[0..at];
  assert line[at..at+p] == nl[0..p];
  assert line[at+p..l+p] == initialLine[at..l];
}

method print_array(a:array<char>)
{
  for i:= 0 to a.Length{
    print(a[i]);
  }
  print(";");
}
method Test1() {
  var line: array<char> := new char[]['a', 'b', 'c', '_', '_']; // Extra spaces to accommodate inserted characters
  var nl: array<char> := new char[]['x', 'y'];
  insert(line, 3, nl, 2, 0);
  print_array(line);
}
method Test2() {
  var line: array<char> := new char[]['a', 'b', 'c', '_', '_'];
  var nl: array<char> := new char[]['x', 'y'];
  insert(line, 3, nl, 2, 3);
  print_array(line);

}
method Test3() {
  var line: array<char> := new char[]['a', 'b', 'c', '_', '_'];
  var nl: array<char> :=new char[] ['x', 'y'];
  insert(line, 3, nl, 2, 1);
  print_array(line);

}

method Test4() {
  var line: array<char> := new char[]['a', 'b', 'c', '_'];
  var nl: array<char> := new char[]['x'];
  insert(line, 3, nl, 1, 2);
  print_array(line);

}
method Test5() {
  var line: array<char> :=new char[] ['_', '_'];
  var nl: array<char> := new char[]['x', 'y'];
  insert(line, 0, nl, 2, 0);
  print_array(line);

}

method TestInsert()
{
  Test1();
  Test2();
  Test3();
  Test4();
  Test5();
}


method Main()
{
  TestInsert();
}
