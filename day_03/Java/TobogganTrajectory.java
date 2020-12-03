import java.util.*;
import java.io.*;

public class TobogganTrajectory {

    public static char[][] parseMapFile(String filePath) throws FileNotFoundException {
        Scanner fileReader = new Scanner(new File(filePath));
        
        ArrayList<char[]> mapData = new ArrayList<char[]>();
        while (fileReader.hasNextLine()) {
            mapData.add(fileReader.nextLine().toCharArray());
        }
        return mapData.toArray(new char[mapData.size()][]);
    }

    public static long countTreeAlongPath(char[][] mapData, int xStep, int yStep) {
        if (mapData == null || mapData.length == 0 || xStep <= 0 || yStep <= 0) {
            return -1;
        }

        int x = 0;
        int y = 0;
        long treeCounter = 0;

        while (y < mapData.length) {
            char[] currentLine = mapData[y];

            x = x % currentLine.length;

            if (currentLine[x] == '#') {
                treeCounter++;
            }

            x += xStep;
            y += yStep;
        }
        return treeCounter;
    }

    public static void main(String[] args) {
        
        if (args.length < 1) {
            System.err.println("Please provide a map file.");
            System.exit(1);
        }
        
        String mapFilePath = args[0];
        char[][] mapData = new char[0][0];
        try {
            mapData = parseMapFile(mapFilePath);
        } catch (FileNotFoundException e) {
            e.printStackTrace();
            System.exit(1);
        }
        
        long pattern1 = countTreeAlongPath(mapData, 1, 1);
        long pattern2 = countTreeAlongPath(mapData, 3, 1);
        long pattern3 = countTreeAlongPath(mapData, 5, 1);
        long pattern4 = countTreeAlongPath(mapData, 7, 1);
        long pattern5 = countTreeAlongPath(mapData, 1, 2);

        long part2Result = pattern1 * pattern2 * pattern3 * pattern4 * pattern5;

        System.out.printf("Part 1 result: %d\n", pattern2);
        System.out.printf("Part 2 result: %d\n", part2Result);
    }
}