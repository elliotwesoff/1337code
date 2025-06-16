# @param {Integer[]} nums
# @return {Integer}
def remove_duplicates(nums)
  len = nums.length
  i = 1

  while i < len do
    if nums[i] == nums[i - 1]
      nums.delete_at(i)
      len -= 1
      next
    end
    i += 1
  end

  nums.length
end
