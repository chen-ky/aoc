import java.util.*;
import java.io.*;

public class HandyHaversacks {

    static HashMap<Node, Integer> cache = new HashMap<>();
    static HashMap<String, Node> seenColours = new HashMap<>();

    /** Parse and return head with childrens */
    public static Node parseLine(String line) {
        String[] headAndContents = line.split(" contain ", 2);
        String head = headAndContents[0].replaceAll(" bag.*$", "");
        String contents = headAndContents[1];
        Node headNode;
        // Head exist, colour's children has been defined
        if (seenColours.containsKey(head)) {
            headNode = seenColours.get(head);
        } else {
            headNode = new Node(head, null);
            seenColours.put(head, headNode);
        }

        if (contents.equals("no other bags.")) {
            return headNode;
        }
        ArrayList<Node> children = new ArrayList<Node>();
        for (String content : contents.split(", ")) {
            String[] countAndColour = content.split(" ", 2);
            int count = Integer.parseInt(countAndColour[0]);
            String colour = countAndColour[1];
            colour = colour.replaceAll(" bag.*$", "");
            
            for (int i = 0; i < count; i++) {
                if (seenColours.containsKey(colour)) {
                    children.add(seenColours.get(colour));
                } else {
                    Node newColour = new Node(colour, null);
                    seenColours.put(colour, newColour);
                    children.add(newColour);
                }
            }
        }
        headNode.children = children;
        return headNode;
    }

    public static void parseFile(String pathName) throws FileNotFoundException {
        Scanner fileReader = new Scanner(new File(pathName));

        while (fileReader.hasNextLine()) {
            parseLine(fileReader.nextLine());
        }
        fileReader.close();
    }

    public static ArrayList<Node> solvePart1() {
        ArrayList<Node> result = new ArrayList<>();
        int previousResultLength = result.size();
        
        while (true) {
            ArrayList<Node> evaluated = new ArrayList<>();
            for (String key : seenColours.keySet()) {
                if (evaluated.contains(seenColours.get(key))) {
                    continue;
                }
                Node currentNode = seenColours.get(key);
                if (result.contains(currentNode)) {
                    continue;
                } else if (currentNode.children == null) {
                    continue;
                }
                if (currentNode.children.contains(seenColours.get("shiny gold"))) {
                    result.add(currentNode);
                } else {
                    for (Node colour : currentNode.children) {
                        if (result.contains(colour)) {
                            result.add(currentNode);
                            break;
                        }
                    }
                }
            }
            // When result size does not change, result is complete.
            if (result.size() == previousResultLength) {
                break;
            }
            previousResultLength = result.size();
        }
        return result;
    }

    public static int findChildrenCount(Node startNode) {
        if (startNode.children == null) {
            return 0;
        }
        int result = 0;
        for (Node child : startNode.children) {
            if (cache.containsKey(child)) {
                result += cache.get(child);
                continue;
            }
            int child_count = findChildrenCount(child);
            cache.put(child, child_count);
            result += child_count;
        }
        result += startNode.children.size();
        return result;
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            System.err.println("Please provide input file path.");
            System.exit(1);
        }
        try {
            parseFile(args[0]);
        } catch (FileNotFoundException e) {
            e.printStackTrace();
            System.exit(1);
        }

        System.out.printf("Part 1: %d\n", solvePart1().size());
        System.out.printf("Part 2: %d\n", findChildrenCount(seenColours.get("shiny gold")));
    }
}


class Node {

    String colourName;
    ArrayList<Node> children;

    public Node(String colour, ArrayList<Node> children) {
        this.colourName = colour;
        this.children = children;
    }

    public String toString() {
        return this.colourName;
    }
}