import sys

def check_is_valid_password_part1(character, minimum, maximum, password):
    if not (isinstance(character, str) \
        and isinstance(minimum, int) \
        and isinstance(maximum, int) \
        and isinstance(password, str)):
            return False

    counter = 0
    for password_char in password:
        if password_char == character:
            counter += 1

    if counter >= minimum and counter <= maximum:
        return True
    return False

def check_is_valid_password_part2(character, position1, position2, password):
    if not (isinstance(character, str) \
        and isinstance(position1, int) \
        and isinstance(position2, int) \
        and isinstance(password, str)):
            return False
    
    position1_match = False
    position2_match = False
    if position1 <= len(password):
        position1_match = password[position1 - 1] == character
    if position2 <= len(password):
        position2_match = password[position2 - 1] == character

    return position1_match ^ position2_match # XOR

def read_file(path_name):
    if not isinstance(path_name, str):
        return None
    f = open(path_name, "r")
    password_policy = f.readlines()
    f.close()
    return password_policy

def parse_line(line):
    if not isinstance(line, str):
        return None
    
    result = {}
    policy, password = line.split(": ", 1)
    min_max, char = policy.split(" ", 1)
    minimum, maximum = min_max.split("-", 1)

    result["minimum"] = int(minimum)
    result["maximum"] = int(maximum)
    result["character"] = char
    result["password"] = password

    return result

def main():
    if len(sys.argv) < 2:
        sys.stderr.write("Please provide password policy file path.\n")
        sys.exit(1)
    
    try:
        file_content = read_file(sys.argv[1])
    except FileNotFoundError as e:
        sys.stderr.write(e + "\n")
        sys.exit(1)
    valid_password_counter_part1 = 0
    valid_password_counter_part2 = 0
    for line in file_content:
        parsed_line = parse_line(line)

        if parsed_line == None:
            continue

        is_valid_password_part1 = check_is_valid_password_part1(
            parsed_line["character"], 
            parsed_line["minimum"],
            parsed_line["maximum"],
            parsed_line["password"])
        
        if is_valid_password_part1:
            valid_password_counter_part1 += 1

        is_valid_password_part2 = check_is_valid_password_part2(
            parsed_line["character"], 
            parsed_line["minimum"],
            parsed_line["maximum"],
            parsed_line["password"])
        
        if is_valid_password_part2:
            valid_password_counter_part2 += 1
    
    print(f"Valid password count (Part 1): {valid_password_counter_part1}")
    print(f"Valid password count (Part 2): {valid_password_counter_part2}")

if "__main__" == __name__:
    main()