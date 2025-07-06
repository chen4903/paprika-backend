import os
from collections import Counter

def count_lines_of_code(file_path, line_counter):
    with open(file_path, 'r') as file:
        lines = file.readlines()

    code_lines = 0
    for line in lines:
        stripped_line = line.strip()
        # Counting code lines, excluding single-line comments and blank lines
        if stripped_line and not stripped_line.startswith('//') and not stripped_line.startswith('/*'):
            code_lines += 1
            line_counter[stripped_line] += 1

    return code_lines

def analyze_go_packages(directory):
    if not os.path.isdir(directory):
        raise ValueError("Provided path is not a directory")

    code_count_per_file = {}
    total_lines_of_code = 0
    line_counter = Counter()

    # Only walk through the 'src' directory
    src_directory = os.path.join(directory, 'src')
    if not os.path.isdir(src_directory):
        raise ValueError("No 'src' directory found in the provided path")

    for root, _, files in os.walk(src_directory):
        for file in files:
            if file.endswith('.rs'):
                full_path = os.path.join(root, file)
                file_code_lines = count_lines_of_code(full_path, line_counter)
                relative_file_path = os.path.relpath(full_path, directory)
                code_count_per_file[relative_file_path] = file_code_lines
                total_lines_of_code += file_code_lines

    return code_count_per_file, total_lines_of_code, line_counter

# Example usage
project_directory = '.'
file_code_counts, total_lines, line_counter = analyze_go_packages(project_directory)

# Output the file line counts
for file, count in file_code_counts.items():
    print(f"File '{file}': {count} lines")

# Output the total line count
print(f"Total lines of code: {total_lines}")

# Calculate and output the number of duplicate lines
duplicate_lines = sum(count - 1 for line, count in line_counter.items() if count > 1)
print(f"Total duplicate lines of code: {duplicate_lines}")