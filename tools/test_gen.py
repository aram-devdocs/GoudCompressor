import os
import json
import random
import string

# -----------------------------------------------------------------------------
# Configuration
# -----------------------------------------------------------------------------

# Directory to store generated test files
OUTPUT_DIR = "test_data"

# Control the file sizes (roughly by lines or elements, for demonstration)
FILE_SIZES = {
    "small": 100,    # e.g., lines, random chunks, or JSON records
    "medium": 1000,
    "large": 10000,
}

# A few repeated patterns to test compressibility
PATTERNS = [
    "AAAAAA",                  # single repeated character
    "abcabcabcabc",            # repeated short sequence
    "Lorem ipsum dolor sit ",  # repeated sentence fragment
]

# Example JSON structures for variety
# You can expand or modify these as needed
JSON_SCHEMAS = [
    {
        "type": "simple",
        "content": {
            "id": 1,
            "name": "Example",
            "attributes": ["red", "green", "blue"]
        }
    },
    {
        "type": "nested",
        "content": {
            "user": {
                "id": 123,
                "profile": {
                    "username": "test_user",
                    "roles": ["admin", "editor"]
                }
            }
        }
    },
    {
        "type": "array_of_objects",
        "content": [
            {"id": i, "data": f"value_{i}"} for i in range(10)
        ]
    }
]

# -----------------------------------------------------------------------------
# Data Generators
# -----------------------------------------------------------------------------

def generate_random_text(num_lines: int) -> str:
    """
    Generate random text with alphanumeric characters.
    Each line is a random string of random length (between 20 and 60).
    """
    lines = []
    for _ in range(num_lines):
        line_length = random.randint(20, 60)
        line = ''.join(random.choices(string.ascii_letters + string.digits, k=line_length))
        lines.append(line)
    return "\n".join(lines)

def generate_repeated_pattern_text(num_lines: int) -> str:
    """
    Generate text that repeats known patterns for testing compressibility.
    """
    lines = []
    for _ in range(num_lines):
        # Pick a random pattern from the list and repeat it multiple times in the same line
        pattern = random.choice(PATTERNS)
        repeat_count = random.randint(1, 5)
        line = pattern * repeat_count
        lines.append(line)
    return "\n".join(lines)

def generate_mixed_text(num_lines: int) -> str:
    """
    Generate a mix of random text lines and repeated pattern lines.
    """
    lines = []
    for _ in range(num_lines):
        if random.random() < 0.5:
            # 50% chance a line is random text
            line_length = random.randint(20, 60)
            line = ''.join(random.choices(string.ascii_letters + string.digits, k=line_length))
        else:
            # 50% chance a line is repeated pattern
            pattern = random.choice(PATTERNS)
            repeat_count = random.randint(1, 5)
            line = pattern * repeat_count
        lines.append(line)
    return "\n".join(lines)

def generate_json_data(schema_index: int, size_factor: int) -> dict:
    """
    Use one of the predefined JSON schemas and expand it based on size_factor.
    This can be adjusted to produce deeper nesting, more fields, etc.
    """
    base_schema = JSON_SCHEMAS[schema_index]["content"]
    json_type = JSON_SCHEMAS[schema_index]["type"]

    # For demonstration, replicate or expand the base schema multiple times
    if isinstance(base_schema, dict):
        # Wrap repeated dicts in a list to get variable size
        return {
            "schemaType": json_type,
            "data": [base_schema for _ in range(size_factor)]
        }
    elif isinstance(base_schema, list):
        # Expand the existing array
        expanded_list = []
        for _ in range(size_factor):
            expanded_list.extend(base_schema)
        return {
            "schemaType": json_type,
            "data": expanded_list
        }
    else:
        # If base_schema is not dict or list, just replicate it
        return {
            "schemaType": json_type,
            "data": [base_schema for _ in range(size_factor)]
        }

def generate_random_json(size_factor: int) -> dict:
    """
    Generate random JSON structure (key-value pairs) for testing.
    Each key is a random string, and each value is either a random number
    or a random string.
    """
    data = {}
    for _ in range(size_factor):
        key_length = random.randint(3, 8)
        key = ''.join(random.choices(string.ascii_lowercase, k=key_length))
        if random.random() < 0.5:
            # random integer value
            data[key] = random.randint(0, 10000)
        else:
            # random string value
            val_length = random.randint(5, 15)
            data[key] = ''.join(random.choices(string.ascii_letters, k=val_length))

    return data

def generate_edge_case_files(num_lines: int) -> str:
    """
    Generate extreme edge-case content, such as lines with special characters,
    empty lines, Unicode characters, etc.
    """
    lines = []
    special_chars = "!@#$%^&*()_+-=[]{}|;':,.<>/?"
    unicode_samples = ["你好", "こんにちは", "🙂", "🌍"]
    
    for i in range(num_lines):
        # 25% special chars, 25% random unicode, 25% empty lines, 25% random text
        choice_val = random.random()
        if choice_val < 0.25:
            line_length = random.randint(5, 20)
            line = ''.join(random.choices(special_chars, k=line_length))
        elif choice_val < 0.5:
            line = random.choice(unicode_samples)
        elif choice_val < 0.75:
            line = ""  # empty line
        else:
            line_length = random.randint(5, 20)
            line = ''.join(random.choices(string.ascii_letters, k=line_length))
        lines.append(line)
    return "\n".join(lines)

# -----------------------------------------------------------------------------
# Main File Generation Logic
# -----------------------------------------------------------------------------

def main():
    # Ensure output directory exists
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    # Generate text files
    for size_label, size_value in FILE_SIZES.items():
        # Random text
        filename = f"{size_label}_random.txt"
        with open(os.path.join(OUTPUT_DIR, filename), "w", encoding="utf-8") as f:
            f.write(generate_random_text(size_value))

        # Repeated pattern text
        filename = f"{size_label}_pattern.txt"
        with open(os.path.join(OUTPUT_DIR, filename), "w", encoding="utf-8") as f:
            f.write(generate_repeated_pattern_text(size_value))

        # Mixed text
        filename = f"{size_label}_mixed.txt"
        with open(os.path.join(OUTPUT_DIR, filename), "w", encoding="utf-8") as f:
            f.write(generate_mixed_text(size_value))

        # Edge-case text
        filename = f"{size_label}_edge_cases.txt"
        with open(os.path.join(OUTPUT_DIR, filename), "w", encoding="utf-8") as f:
            f.write(generate_edge_case_files(size_value))

    # Generate JSON files
    # Example 1: Using predefined schemas
    for size_label, size_value in FILE_SIZES.items():
        for i, schema_info in enumerate(JSON_SCHEMAS):
            filename = f"{size_label}_schema_{i}.json"
            file_data = generate_json_data(i, max(size_value // 10, 1))
            with open(os.path.join(OUTPUT_DIR, filename), "w", encoding="utf-8") as f:
                json.dump(file_data, f, indent=2)

    # Example 2: Completely random JSON
    for size_label, size_value in FILE_SIZES.items():
        filename = f"{size_label}_random.json"
        random_json_data = generate_random_json(size_value // 10)  # Scale factor
        with open(os.path.join(OUTPUT_DIR, filename), "w", encoding="utf-8") as f:
            json.dump(random_json_data, f, indent=2)

    print(f"Generated test files in the '{OUTPUT_DIR}' directory.")

if __name__ == "__main__":
    main()