method twoSum(nums: array<int>, target: int) returns (i: int, j: int)
  requires nums.Length > 1
  requires exists i,j::0 <= i < j < nums.Length &&  nums[i] + nums[j] == target
  ensures 0 <= i < j < nums.Length && nums[i] + nums[j] == target
  ensures forall ii,jj:: (0 <= ii < i && ii < jj < nums.Length) || (ii == i && ii < jj < j) ==> nums[ii] + nums[jj] != target
{
  var n := nums.Length;
  i := 0;
  j := 1;
  while i < n - 1
    invariant 0 <= i < j <= n
    invariant forall ii, jj :: 0 <= ii < i && ii < jj < n ==> nums[ii] + nums[jj] != target
  {
    j := i + 1;
    while j < n
      invariant 0 <= i < j <= n
      invariant forall jj :: i < jj < j ==> nums[i] + nums[jj] != target
    {
      if nums[i] + nums[j] == target {
        return;
      }
      j := j + 1;
    }

    i := i + 1;
  }
}

method TestTwoSum()
{
  var nums := new int[][2,7,11,15];
  var target := 9;
  var i, j := twoSum(nums, target);
  print( i,j, "\n");

  nums := new int[][3,2,4];
  target := 6;
  i, j := twoSum(nums, target);
  print( i,j, "\n");

  nums := new int[][3,3];
  target := 6;
  i, j := twoSum(nums, target);
  print( i,j, "\n");

  nums := new int[][0,0,1];
  target := 1;
  i, j := twoSum(nums, target);
  print( i,j, "\n");

  nums := new int[][4,2,1];
  target := 5;
  i, j := twoSum(nums, target);
  print( i,j, "\n");
}


method Main()
{
  TestTwoSum();
}
