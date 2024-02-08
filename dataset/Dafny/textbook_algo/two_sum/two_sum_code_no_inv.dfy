method twoSum(nums: array<int>, target: int) returns (i: int, j: int)

{
  var n := nums.Length;
  i := 0;
  j := 1;
  while i < n - 1
 {
    j := i + 1;
    while j < n
  {
      if nums[i] + nums[j] == target {
        return;
      }
      j := j + 1;
    }

    i := i + 1;
  }
}
