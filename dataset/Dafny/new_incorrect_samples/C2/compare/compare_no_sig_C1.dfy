method foo<T(==)>(a: T, b: T) returns (eq: bool)
  requires a==b
  ensures a==b ==> eq==true
  ensures a!=b ==> eq==false
{
  if a == b { eq := true; } else { eq := false; }
}
