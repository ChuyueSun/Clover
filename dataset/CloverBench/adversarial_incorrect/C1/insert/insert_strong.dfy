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