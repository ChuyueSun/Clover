method twoSum(nums: array<int>, target: int) returns (i: int, j: int)
{
  i := 0;
  j := 0;
  var found := false;

  while (i < nums.Length - 1) && !found
    invariant 0 <= i < nums.Length
    invariant !found ==> forall ii, jj :: (0 <= ii < i && ii < jj < nums.Length) ==> nums[ii] + nums[jj] != target
  {
    j := i + 1;
    while (j < nums.Length) && !found
      invariant i + 1 <= j <= nums.Length
      invariant forall jj :: i < jj < j ==> nums[i] + nums[jj] != target
    {
      if nums[i] + nums[j] == target {
        found := true;
      } else {
        j := j + 1;
      }
    }
    if !found {
      i := i + 1;
    }
  }
}
