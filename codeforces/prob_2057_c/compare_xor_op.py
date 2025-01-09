# Define the new sets of numbers
sets_to_compare = [
    (7, 16, 11),  # Set 1
    (6, 21, 22)   # Set 2
]

# Compute results for each set
comparison_results = []
for a, b, c in sets_to_compare:
    a_xor_b = a ^ b
    b_xor_c = b ^ c
    a_xor_c = a ^ c
    result = a_xor_b + b_xor_c + a_xor_c
    comparison_results.append((a_xor_b, b_xor_c, a_xor_c, result))

print(comparison_results)

