method twoSum(nums: array<int>, target: int) returns (i: int, j: int)
  requires nums.Length > 1
  requires exists i,j::0 <= i < j < nums.Length &&  nums[i] + nums[j] == target
  ensures 0 <= i < nums.Length && 0 <= j < nums.Length && nums[i] + nums[j] == target

{
  var n := nums.Length;
  i := n - 1;
  j := n - 2;
  while i >= 1
    invariant -1 <= j < i < n
    invariant forall ii, jj :: i < ii < n && 0 <= jj < ii ==> nums[ii] + nums[jj] != target
  {
    j := i - 1;
    while j >= 0
      invariant -1 <= j < i < n
      invariant forall jj :: j < jj < i ==> nums[i] + nums[jj] != target
    {
      if nums[i] + nums[j] == target {
        return;
      }
      j := j - 1;
    }

    i := i - 1;
  }
}
