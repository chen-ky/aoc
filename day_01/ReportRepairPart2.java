// Will reimplement in Rust, time constrain reason.

import java.util.*;
import java.io.*;

public class ReportRepairPart2 {
    
    public static Integer[] readFile(String filePath) throws FileNotFoundException {
        
        Scanner fileReader = new Scanner(new File(filePath));
        ArrayList<Integer> data = new ArrayList<Integer>();
        
        while (fileReader.hasNextInt()) {
            data.add(fileReader.nextInt());
        }
        return data.toArray(new Integer[data.size()]);
    }
    
    public static void main(String[] args) {
        
        Integer[] data = new Integer[0];
        try {
            data = readFile("input");
        } catch (FileNotFoundException e) {
            e.printStackTrace();
            System.exit(1);
        }

        // Terrible algo
        boolean matched = false;
        for (Integer num1 : data) {
            for (Integer num2 : data) {
                for (Integer num3 : data) {
                    if (num1.intValue() + num2.intValue() + num3.intValue() == 2020) {
                        System.out.println(num1 * num2 * num3);
                        matched = true;
                        break;
                    }
                    if (matched) {
                        break;
                    }
                }
            }
            if (matched) {
                break;
            }
        }

    }
}