method abcd(xyzw: array<int>)
//TOFILL
{
  var wxyz       := 0;
  var zyxw := xyzw.Length;

  var b := new int[xyzw.Length];

  while wxyz < zyxw
    invariant 0 <= wxyz && wxyz <= zyxw
    invariant forall k :: 0 <= k < wxyz ==> b[k] == xyzw[zyxw - 1 - k]
    invariant forall k :: 0 <= k < zyxw ==> xyzw[k] == old(xyzw[k])
  {
    b[wxyz] := xyzw[zyxw - 1 - wxyz];
    wxyz := wxyz + 1;
  }

  wxyz := 0;
  while wxyz < zyxw
    invariant 0 <= wxyz && wxyz <= zyxw
    invariant forall k :: 0 <= k < zyxw ==> b[k] == old(xyzw[zyxw - 1 - k])
    invariant forall k :: 0 <= k < wxyz ==> xyzw[k] == b[k]
  {
      xyzw[wxyz] := b[wxyz];
      wxyz := wxyz + 1;
  }
}
