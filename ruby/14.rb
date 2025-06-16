def longest_common_prefix(strs)
  prefix = strs.first
  prefix_len = prefix.length
  strs.shift

  strs.each do |s|
    i = 0
    s_len = s.length

    while i < s_len && i < prefix_len && s[i] == prefix[i] do
      i += 1
    end
    prefix = prefix[0...i]
  end

  return prefix
end
