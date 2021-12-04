import sys

def file_parser(file_path):
    f = open(file_path, "r")
    file_content = f.readlines()
    f.close()
    
    result = []
    group = ""
    people_in_group = 0
    for line in file_content:
        if line == "\n" and group != "":
            result.append([people_in_group, group])
            group = ""
            people_in_group = 0
            continue
        
        group += line.strip()
        people_in_group += 1

    if group != "":
        result.append([people_in_group, group])

    return result

def count_part_1(group_responses):
    result = 0
    for response in group_responses:
        result += len(find_or_responses(response[1]))
    return result

def count_part_2(group_responses):
    result = 0
    for response in group_responses:
        result += len(find_and_responses(response))
    return result

def find_or_responses(group_response): # Part 1 'Or' responses
    result = set()
    for char in group_response:
        if not char in result:
            result.add(char)
    return result

def find_and_responses(group): # Part 2 'And' responses
    result = set()
    peoples = group[0]
    answer = group[1]
    unique_answer = find_or_responses(answer)

    for char in unique_answer:
        if peoples == answer.count(char):
            result.add(char)  
    return result

def main():
    if len(sys.argv) < 2:
        sys.stderr.write("Please provide a file path.\n")
        return
    
    group_responses = file_parser(sys.argv[1])
    print(f"Part 1: {count_part_1(group_responses)}")
    print(f"Part 2: {count_part_2(group_responses)}")
    

if "__main__" == __name__:
    main()