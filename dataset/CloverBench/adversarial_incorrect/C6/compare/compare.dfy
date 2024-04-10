method Compare<T(==)>(a: T, b: T) returns (eq: bool)
  ensures a==b ==> eq==false
  ensures a!=b ==> eq==true
{
  if a == b { eq := false; } else { eq := true; }
}
