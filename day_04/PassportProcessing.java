import java.util.*;
import java.io.*;

public class PassportProcessing {

    public static ArrayList<HashMap<String, String>> parseFile(String filePath) throws FileNotFoundException {
        if (filePath == null) {
            return null;
        }

        Scanner fileReader = new Scanner(new File(filePath));
        
        ArrayList<HashMap<String, String>> passportData = new ArrayList<HashMap<String, String>>();
        String individualPassport = "";

        // Or condition for last line
        while (fileReader.hasNextLine() || !individualPassport.equals("")) {
            
            String currentLine;
            if (fileReader.hasNextLine()) {
                currentLine = fileReader.nextLine();
            } else {
                currentLine = "";
            }

            if (currentLine.equals("") && !individualPassport.equals("")) {
                
                HashMap<String, String> parsedPassportData = new HashMap<String, String>();
                String[] splittedString = individualPassport.split(" ");

                for (String attribute : splittedString) {
                    String[] keyValue = attribute.split(":", 2);
                    parsedPassportData.put(keyValue[0], keyValue[1]);
                }
                passportData.add(parsedPassportData);
                individualPassport = "";
            } else if (currentLine.equals("")) {
                individualPassport = "";
            } else {
                if (!individualPassport.equals("")) {
                    individualPassport += " ";
                }
                individualPassport += currentLine;
            }
        }
        return passportData;
    }

    public static int countValidPassportPart1(List<HashMap<String, String>> passportData) {
        if (passportData == null) {
            return -1;
        }
        
        int validCounter = 0;
        for (HashMap<String, String> passport : passportData) {
            if (validatePassportPart1(passport)) {
                validCounter++;
            }
        }
        return validCounter;
    }

    public static int countValidPassportPart2(List<HashMap<String, String>> passportData) {
        if (passportData == null) {
            return -1;
        }
        
        int validCounter = 0;
        for (HashMap<String, String> passport : passportData) {
            if (validatePassportPart2(passport)) {
                validCounter++;
            }
        }
        return validCounter;
    }

    public static boolean validatePassportPart1(HashMap<String, String> passport) {
        if (passport == null) {
            return false;
        }
        
        String[] requiredKey = new String[] {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};

        for (String key : requiredKey) {
            if (passport.containsKey(key)) {
                continue;
            } else {
                return false;
            }
        }
        return true;
    }

    public static boolean validatePassportPart2(HashMap<String, String> passport) {
         if (passport == null) {
            return false;
        }

        if (!validatePassportPart1(passport)) {
            return false;
        }

        if (
            isValidBirthYear(passport.get("byr")) &&
            isValidIssueYear(passport.get("iyr")) &&
            isValidExpirationYear(passport.get("eyr")) &&
            isValidHeight (passport.get("hgt")) &&
            isValidHairColour(passport.get("hcl")) &&
            isValidEyeColour(passport.get("ecl")) &&
            isValidPassportId(passport.get("pid"))
            ) {
                return true;
            }
        
        return false;
    }
    
    public static boolean isBase10(String number) {
        if (number == null) {
            return false;
        }
        if (number.matches("^\\d*\\d$")) {
            return true;
        }
        return false;
    }

    /** Does not accept hex with preceeding '0x' */
    public static boolean isBase16(String number) {
        if (number == null) {
            return false;
        }
        if (number.matches("^[\\da-fA-F]*[\\da-fA-F]$")) {
            return true;
        }
        return false;
    }

    public static boolean isValidBirthYear(String yearStr) {
        if (yearStr == null) {
            return false;
        }

        int year;
        if (isBase10(yearStr)) {
            year = Integer.parseInt(yearStr);
        } else {
            return false;
        }

        if (year >= 1920 && year <= 2002) {
            return true;
        }
        return false;
    }

    public static boolean isValidIssueYear(String yearStr) {
        if (yearStr == null) {
            return false;
        }

        int year;
        if (isBase10(yearStr)) {
            year = Integer.parseInt(yearStr);
        } else {
            return false;
        }

        if (year >= 2010 && year <= 2020) {
            return true;
        }
        return false;
    }

    public static boolean isValidExpirationYear(String yearStr) {
        if (yearStr == null) {
            return false;
        }

        int year;
        if (isBase10(yearStr)) {
            year = Integer.parseInt(yearStr);
        } else {
            return false;
        }

        if (year >= 2020 && year <= 2030) {
            return true;
        }
        return false;
    }

    public static boolean isValidHeight(String heightStr) {
        if (heightStr == null) {
            return false;
        }
        String unit;
        int height;

        if (heightStr.endsWith("cm")) {
            unit = "cm";
        } else if (heightStr.endsWith("in")) {
            unit = "in";
        } else {
            return false;
        }

        heightStr = heightStr.substring(0, heightStr.length() - 2);
        
        if (isBase10((heightStr))) {
            height = Integer.parseInt(heightStr);
        } else {
            return false;
        }

        if (unit.equals("cm")) {
            if (height >= 150 && height <= 193) {
                return true;
            }
        } else if (unit.equals("in")) {
            if (height >= 59 && height <= 76) {
                return true;
            }
        }
        return false;
    }

    public static boolean isValidHairColour(String hairColour) {
        if (hairColour == null) {
            return false;
        }

        if (hairColour.length() != 7) {
            return false;
        } else if (hairColour.charAt(0) != '#') {
            return false;
        }

        hairColour = hairColour.substring(1);
        
        if (isBase16(hairColour)) {
            return true;
        }
        return false;
    }

    public static boolean isValidEyeColour(String eyeColour) {
        if (eyeColour == null) {
            return false;
        }

        String[] validColour = new String[] {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};
        
        for (String colour : validColour) {
            if (eyeColour.equals(colour)) {
                return true;
            }
        }
        return false;
    }

    public static boolean isValidPassportId(String passportId) {
        if (passportId == null) {
            return false;
        }

        if (passportId.length() != 9) {
            return false;
        }
        if (isBase10(passportId)) {
            return true;
        }
        return false;
    }

    public static void main(String[] args) {
        
        ArrayList<HashMap<String, String>> passportData = null;
        try {
            passportData = parseFile("input");
        } catch (FileNotFoundException e) {
            System.err.println("Error parsing file: input");
            System.exit(1);
        }

        if (passportData == null) {
            System.err.println("Something has gone wrong!");
            System.exit(1);
        }

        System.out.printf("Valid passport count (Part 1): %d\n", countValidPassportPart1(passportData));
        System.out.printf("Valid passport count (Part 2): %d\n", countValidPassportPart2(passportData));
    }
}