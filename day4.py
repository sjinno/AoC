with open("./files/day4.txt", "r") as f:
    passports = f.read().split("\n\n")
print(f"Total number of passports: {len(passports)}.")

FIELDS = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
length_of_fields = len(FIELDS)

count = 0
for passport in passports:
    f_count = 0
    fields = passport.split()
    for field in fields:
        f = field.split(':')[0]
        if f in FIELDS:
            if f == "cid":
                continue
            f_count += 1
    if f_count == length_of_fields-1:
        count += 1

print(count)
