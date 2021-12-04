import java.io.*;
import java.util.*;

public class BinaryBoarding {
    
    public static ArrayList<Long> takenSeat = new ArrayList<Long>();

    public static String[] readFile(String filePath) throws FileNotFoundException {
        if (filePath == null) {
            return null;
        }
        ArrayList<String> result = new ArrayList<String>();
        Scanner fileReader = new Scanner(new File(filePath));
        while (fileReader.hasNext()) {
            result.add(fileReader.next());
        }
        return result.toArray(new String[result.size()]);
    }

    public static long findSeat(String binarySeat) {
        if (binarySeat == null) {
            return -1;
        } else if (binarySeat.length() != 10) {
            return -1;
        }
        char[] binaryRow = binarySeat.substring(0, 7).toCharArray();
        char[] binaryColumn = binarySeat.substring(7).toCharArray();

        double minRow = 0;
        double minColumn = 0;
        double maxRow = 127;
        double maxColumn = 7;
        double[] result = new double[2];

        for (int i = 0; i < binaryRow.length; i++) {
            char currentChar = binaryRow[i];
            double[] newRange = narrowSearchRange('F', minRow, 'B', maxRow, currentChar);
            minRow = newRange[0];
            maxRow = newRange[1];
            if (currentChar == 'F') { result[0] = minRow; } 
            else { result[0] = maxRow; }
        }

        for (int i = 0; i < binaryColumn.length; i++) {
            char currentChar = binaryColumn[i];
            double[] newRange = narrowSearchRange('L', minColumn, 'R', maxColumn, currentChar);
            minColumn = newRange[0];
            maxColumn = newRange[1];
            if (currentChar == 'L') { result[1] = minColumn; } 
            else { result[1] = maxColumn; }
        }
        return uniqueSeatId(Math.round(result[0]), Math.round(result[1]));
    }

    public static double[] narrowSearchRange(char lowerChar, double inclusiveLow, char higherChar, double inclusiveHigh, char currentChar) {
        if (inclusiveLow == inclusiveHigh) {
            return new double[] {inclusiveLow, inclusiveHigh};
        }

        double newLow;
        double newHigh;
        if (currentChar == lowerChar) {
            newLow = inclusiveLow;
            newHigh = Math.floor(inclusiveHigh - ((inclusiveHigh - inclusiveLow) / 2));
        } else if (currentChar == higherChar) {
            newLow = Math.round(inclusiveLow + ((inclusiveHigh - inclusiveLow) / 2));
            newHigh = inclusiveHigh;
        } else {
            return null;
        }
        return new double[] {newLow, newHigh};
    }

    public static long uniqueSeatId(long row, long column) {
        return (row * 8) + column;
    }

    public static long findEmptySeat(ArrayList<Long> takenSeat) {
        for (int i = 0; i < takenSeat.size() - 1; i++) {
            if (takenSeat.get(i + 1) == takenSeat.get(i) + 1) {
                continue;
            } else {
                return takenSeat.get(i) + 1;
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            System.err.println("Please provide file path");
            System.exit(1);
        }

        String[] fileData = null;
        try {
            fileData = readFile(args[0]);
        } catch (FileNotFoundException e) {
            System.err.println("Cannot find file.");
            System.exit(1);
        }
        if (fileData == null) {
            System.err.println("Something went wrong!");
            System.exit(1);
        }

        for (String binarySeat : fileData) {
            takenSeat.add(findSeat(binarySeat));
        }
        takenSeat.sort(Comparator.naturalOrder());
        System.out.printf("Last seat: %d\n", takenSeat.get(takenSeat.size() - 1));
        System.out.printf("Empty seat: %d\n", findEmptySeat(takenSeat));
    }
}
