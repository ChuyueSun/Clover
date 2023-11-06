method insert(line:array<char>, l:int, nl:array<char>, p:int, at:int)
{

  var i:int := l;
  while(i>at)
  {
    i := i - 1;
    line[i+p] := line[i];
  }


  i := 0;
  while(i<p)
  {
    line[at + i] := nl[i];
    i := i + 1;
  }
}