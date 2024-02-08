method twoSum(nums: array<int>, target: int) returns (i: int, j: int)
    requires nums != null && nums.Length > 1
    requires exists a: int, b: int :: a < b < nums.Length && nums[a] + nums[b] == target
    ensures 0 <= i < j < nums.Length
    ensures nums[i] + nums[j] == target
    ensures forall k: int :: 0 <= k < i ==> !(exists l: int :: k < l < nums.Length && nums[k] + nums[l] == target)
    ensures forall l: int :: i < l < j ==> !(nums[i] + nums[l] == target)
