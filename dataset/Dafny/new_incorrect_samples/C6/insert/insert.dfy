method insert(line:array<char>, l:int, nl:array<char>, p:int, at:int)
  requires 0 <= l+p <= line.Length 
  requires 0 <= p <= nl.Length 
  requires 0 <= at <= l 
  modifies line
  ensures forall i :: (0<=i<p) ==> line[at+i] == nl[i] 
  ensures forall i :: (0<=i<at) ==> line[i] == old(line[i])
{
  ghost var initialLine := line[..];

  assert line[0..at] == initialLine[0..at];

  var i := 0;
  while(i<p)
    invariant 0<=i<=p
    invariant line[0..at] == initialLine[0..at]
    invariant line[at..at+i] == nl[0..i]
  {
    line[at + i] := nl[i];
    i := i + 1;
  }

  assert line[0..at] == initialLine[0..at];
  assert line[at..at+p] == nl[0..p];
}
